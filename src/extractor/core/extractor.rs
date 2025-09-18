use crate::extractor::parsers::{get_parser_registry, parse_with_thread_local};
use crate::models::CodeChunk;
use crate::{GitTypeError, Result};
use std::fs;
use std::path::{Path, PathBuf};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, QueryCursor, Tree};

pub struct CommonExtractor;

impl CommonExtractor {
    /// Find git repository root starting from the given path
    fn find_git_repository_root(start_path: &Path) -> Option<PathBuf> {
        let mut current_path = start_path;

        // If start_path is a file, start from its parent directory
        if current_path.is_file() {
            current_path = current_path.parent()?;
        }

        loop {
            let git_dir = current_path.join(".git");
            if git_dir.exists() {
                return Some(current_path.to_path_buf());
            }

            // Move to parent directory
            match current_path.parent() {
                Some(parent) => current_path = parent,
                None => break,
            }
        }

        None
    }
    pub fn extract_chunks_from_tree(
        tree: &Tree,
        source_code: &str,
        file_path: &Path,
        language: &str,
    ) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();
        let registry = get_parser_registry();

        let file_comment_ranges = Self::extract_comment_ranges(tree, source_code, language)?;
        let query = registry.create_query(language)?;

        // Find git root once for this file
        let git_root = Self::find_git_repository_root(file_path);
        let relative_file_path = if let Some(ref git_root) = git_root {
            if let Ok(relative) = file_path.strip_prefix(git_root) {
                relative.to_path_buf()
            } else {
                file_path.to_path_buf()
            }
        } else {
            file_path.to_path_buf()
        };

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        // Extract standard function/class start chunks
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];

                if let Some(chunk) = Self::node_to_chunk(
                    node,
                    source_code,
                    &relative_file_path,
                    language,
                    capture_name,
                    &file_comment_ranges,
                ) {
                    chunks.push(chunk);
                }
            }
        }

        // Extract middle implementation chunks for more variety
        let middle_chunks = Self::extract_middle_chunks(
            tree,
            source_code,
            &relative_file_path,
            language,
            &file_comment_ranges,
        )?;
        chunks.extend(middle_chunks);

        // Add a whole-file chunk for Zen mode
        let line_count = source_code.lines().count();
        let zen_chunk = CodeChunk {
            content: source_code.to_string(),
            file_path: relative_file_path.clone(),
            start_line: 1,
            end_line: line_count,
            language: language.to_string(),
            chunk_type: crate::models::ChunkType::File,
            name: "entire_file".to_string(),
            comment_ranges: file_comment_ranges,
            original_indentation: 0,
        };
        chunks.push(zen_chunk);

        // Remove duplicates: prioritize specific chunk types over generic ones
        // Sort by position first, then prioritize Function/Class/etc over CodeBlock
        chunks.sort_by(|a, b| {
            let pos_cmp = (a.start_line, a.end_line).cmp(&(b.start_line, b.end_line));
            if pos_cmp == std::cmp::Ordering::Equal {
                // Prioritize specific types over generic CodeBlock, File chunks go last
                let a_priority = match a.chunk_type {
                    crate::models::ChunkType::Function => 0,
                    crate::models::ChunkType::Class => 0,
                    crate::models::ChunkType::Method => 0,
                    crate::models::ChunkType::CodeBlock => 10,
                    crate::models::ChunkType::File => 20, // File chunks (Zen mode) go last
                    _ => 5,
                };
                let b_priority = match b.chunk_type {
                    crate::models::ChunkType::Function => 0,
                    crate::models::ChunkType::Class => 0,
                    crate::models::ChunkType::Method => 0,
                    crate::models::ChunkType::CodeBlock => 10,
                    crate::models::ChunkType::File => 20, // File chunks (Zen mode) go last
                    _ => 5,
                };
                a_priority.cmp(&b_priority)
            } else {
                pos_cmp
            }
        });

        // Remove duplicates based on position, but keep File chunks separate
        chunks.dedup_by(|a, b| {
            a.start_line == b.start_line
                && a.end_line == b.end_line
                && a.chunk_type != crate::models::ChunkType::File
                && b.chunk_type != crate::models::ChunkType::File
        });

        Ok(chunks)
    }

    pub fn extract_middle_chunks(
        tree: &Tree,
        source_code: &str,
        file_path: &Path,
        language: &str,
        file_comment_ranges: &[(usize, usize)],
    ) -> Result<Vec<CodeChunk>> {
        let registry = get_parser_registry();
        let extractor = registry.get_extractor(language)?;
        let middle_query = registry.create_middle_implementation_query(language)?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&middle_query, tree.root_node(), source_code.as_bytes());

        let mut captures = Vec::new();
        while let Some(match_) = matches.next() {
            captures.extend(match_.captures.iter().map(|capture| {
                let node = capture.node;
                let capture_name = &middle_query.capture_names()[capture.index as usize];
                (node, capture_name)
            }));
        }

        let mut chunks: Vec<CodeChunk> = captures
            .into_iter()
            .map(|(node, capture_name)| {
                (
                    node,
                    capture_name,
                    extractor.middle_capture_name_to_chunk_type(capture_name),
                )
            })
            .filter(|(_, _, chunk_type)| chunk_type.is_some())
            .map(|(node, capture_name, chunk_type)| (node, capture_name, chunk_type.unwrap()))
            .map(|(node, capture_name, chunk_type)| {
                let start_byte = node.start_byte();
                let end_byte = node.end_byte();
                let content = &source_code[start_byte..end_byte];
                (
                    node,
                    capture_name,
                    chunk_type,
                    content,
                    start_byte,
                    end_byte,
                )
            })
            .filter(|(_, _, _, content, _, _)| {
                let line_count = content.lines().count();
                line_count >= 2 && content.len() >= 30 && content.len() <= 2000
            })
            .map(
                |(node, capture_name, chunk_type, content, start_byte, end_byte)| {
                    // Use node_to_chunk-like logic for proper indentation and comment handling
                    let start_char = Self::byte_to_char_position(source_code, start_byte);
                    let end_char = Self::byte_to_char_position(source_code, end_byte);

                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;
                    let original_indentation_bytes = node.start_position().column;

                    // Extract actual indentation characters from source
                    let original_indent_chars = if original_indentation_bytes > 0 {
                        Self::extract_line_indent_chars_corrected(
                            source_code,
                            node.start_position().row,
                            original_indentation_bytes,
                        )
                    } else {
                        String::new()
                    };

                    let normalized_content =
                        Self::normalize_first_line_indentation(content, &original_indent_chars);

                    // Adjust comment ranges to be relative to the normalized content
                    let indent_offset_chars = original_indent_chars.chars().count();

                    let chunk_comment_ranges: Vec<(usize, usize)> = file_comment_ranges
                        .iter()
                        .filter_map(|&(comment_raw_pos_start, comment_raw_pos_end)| {
                            // Check if comment is within this chunk's boundaries
                            if comment_raw_pos_start >= start_char
                                && comment_raw_pos_end <= end_char
                            {
                                // Convert to chunk-relative positions
                                let comment_start_pos = comment_raw_pos_start - start_char;
                                let comment_end_pos = comment_raw_pos_end - start_char;

                                // Account for added indentation at the very start of normalized content
                                let adjusted_start = comment_start_pos + indent_offset_chars;
                                let adjusted_end = comment_end_pos + indent_offset_chars;

                                Some((adjusted_start, adjusted_end))
                            } else {
                                None
                            }
                        })
                        .collect();

                    CodeChunk {
                        name: capture_name.to_string(),
                        content: normalized_content,
                        chunk_type,
                        language: language.to_string(),
                        file_path: file_path.to_path_buf(),
                        start_line,
                        end_line,
                        comment_ranges: chunk_comment_ranges,
                        original_indentation: indent_offset_chars,
                    }
                },
            )
            .collect();

        // Remove duplicates based on exact position and content
        chunks.sort_by(|a, b| {
            (a.start_line, a.end_line, &a.content).cmp(&(b.start_line, b.end_line, &b.content))
        });
        chunks.dedup_by(|a, b| {
            a.start_line == b.start_line && a.end_line == b.end_line && a.content == b.content
        });

        Ok(chunks)
    }

    pub fn extract_from_file(file_path: &Path, language: &str) -> Result<Vec<CodeChunk>> {
        let content = fs::read_to_string(file_path)?;
        // Reuse per-thread parser instance for the language
        let tree = parse_with_thread_local(language, &content).ok_or_else(|| {
            GitTypeError::ExtractionFailed(format!("Failed to parse file: {:?}", file_path))
        })?;

        Self::extract_chunks_from_tree(&tree, &content, file_path, language)
    }

    pub fn extract_comment_ranges(
        tree: &Tree,
        source_code: &str,
        language: &str,
    ) -> Result<Vec<(usize, usize)>> {
        let registry = get_parser_registry();
        let comment_query = registry.create_comment_query(language)?;
        let mut comment_ranges = Vec::new();

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&comment_query, tree.root_node(), source_code.as_bytes());

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;
                let start_byte = node.start_byte();
                let end_byte = node.end_byte();

                if Self::is_valid_comment_node(node, language) {
                    // Convert byte positions to character positions
                    let start_char = Self::byte_to_char_position(source_code, start_byte);
                    let end_char = Self::byte_to_char_position(source_code, end_byte);
                    comment_ranges.push((start_char, end_char));
                }
            }
        }

        comment_ranges.sort_by_key(|&(start, _)| start);
        Ok(comment_ranges)
    }

    /// Convert byte position to character position in the given string
    fn byte_to_char_position(source_code: &str, byte_pos: usize) -> usize {
        source_code[..byte_pos.min(source_code.len())]
            .chars()
            .count()
    }

    fn is_valid_comment_node(node: Node, language: &str) -> bool {
        let node_kind = node.kind();
        match language {
            "rust" => node_kind == "line_comment" || node_kind == "block_comment",
            "typescript" => node_kind == "comment",
            "javascript" => node_kind == "comment",
            "python" => node_kind == "comment",
            "ruby" => node_kind == "comment",
            "go" => node_kind == "comment",
            "swift" => node_kind == "comment" || node_kind == "multiline_comment",
            "kotlin" => node_kind == "line_comment" || node_kind == "multiline_comment",
            "java" => node_kind == "line_comment" || node_kind == "block_comment",
            "php" => node_kind == "comment" || node_kind == "shell_comment_line",
            "csharp" => node_kind == "comment",
            "c" => node_kind == "comment",
            "cpp" => node_kind == "comment",
            "haskell" => node_kind == "comment",
            "dart" => node_kind == "comment" || node_kind == "documentation_comment",
            "scala" => node_kind == "comment" || node_kind == "block_comment",
            _ => false,
        }
    }

    fn node_to_chunk(
        node: Node,
        source_code: &str,
        file_path: &Path,
        language: &str,
        capture_name: &str,
        file_comment_ranges: &[(usize, usize)], // Already in character positions
    ) -> Option<CodeChunk> {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = &source_code[start_byte..end_byte];

        // Convert byte positions to character positions to match file_comment_ranges
        let start_char = Self::byte_to_char_position(source_code, start_byte);
        let end_char = Self::byte_to_char_position(source_code, end_byte);

        let start_line = node.start_position().row + 1;
        let end_line = node.end_position().row + 1;
        let original_indentation_bytes = node.start_position().column;

        // Extract actual indentation characters from source
        // Note: original_indentation is in byte units from TreeSitter, but we need char units
        let original_indent_chars = if original_indentation_bytes > 0 {
            Self::extract_line_indent_chars_corrected(
                source_code,
                node.start_position().row,
                original_indentation_bytes,
            )
        } else {
            String::new()
        };

        let registry = get_parser_registry();
        let extractor = registry.get_extractor(language).ok()?;

        let chunk_type = extractor.capture_name_to_chunk_type(capture_name)?;

        let name = extractor
            .extract_name(node, source_code, capture_name)
            .or_else(|| Self::extract_name(node, source_code))
            .unwrap_or_else(|| "unknown".to_string());

        let normalized_content =
            Self::normalize_first_line_indentation(content, &original_indent_chars);

        // Simple position calculation:
        // code_start_pos = start_char (TreeSitter chunk の行頭)
        // chunk_start_pos = original_indentation (node.start_position().column)
        // comment_start_pos = comment生pos - code_start_pos

        // Adjust comment ranges to be relative to the normalized content.
        // Note:
        // - file_comment_ranges are character-based positions for the whole file
        // - We first convert them to chunk-relative character positions
        // - Then we add the first-line indentation characters we injected at the very
        //   beginning of the normalized content, so display-time positions match
        let indent_offset_chars = original_indent_chars.chars().count();

        let chunk_comment_ranges: Vec<(usize, usize)> = file_comment_ranges
            .iter()
            .filter_map(|&(comment_raw_pos_start, comment_raw_pos_end)| {
                // Check if comment is within this chunk's boundaries
                if comment_raw_pos_start >= start_char && comment_raw_pos_end <= end_char {
                    // Convert to chunk-relative positions
                    let comment_start_pos = comment_raw_pos_start - start_char;
                    let comment_end_pos = comment_raw_pos_end - start_char;

                    // Account for added indentation at the very start of normalized content
                    let adjusted_start = comment_start_pos + indent_offset_chars;
                    let adjusted_end = comment_end_pos + indent_offset_chars;

                    Some((adjusted_start, adjusted_end))
                } else {
                    None
                }
            })
            .collect();

        Some(CodeChunk {
            content: normalized_content,
            file_path: file_path.to_path_buf(),
            start_line,
            end_line,
            language: language.to_string(),
            chunk_type,
            name,
            comment_ranges: chunk_comment_ranges,
            // Store indentation as character count to keep extractor outputs character-based
            original_indentation: indent_offset_chars,
        })
    }

    fn extract_name(node: Node, source_code: &str) -> Option<String> {
        if node.kind() == "variable_declarator" {
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                let name_node = cursor.node();
                if name_node.kind() == "identifier" {
                    let start = name_node.start_byte();
                    let end = name_node.end_byte();
                    return Some(source_code[start..end].to_string());
                }
            }
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier"
                    || child.kind() == "type_identifier"
                    || child.kind() == "property_identifier"
                    || child.kind() == "field_identifier"
                    || child.kind() == "constant"
                    || child.kind() == "simple_identifier"
                {
                    let start = child.start_byte();
                    let end = child.end_byte();
                    return Some(source_code[start..end].to_string());
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        None
    }

    fn normalize_first_line_indentation(content: &str, original_indent_chars: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return content.to_string();
        }

        let mut result_lines = Vec::new();

        for (line_idx, line) in lines.iter().enumerate() {
            if line_idx == 0 {
                // First line: add original indentation characters from source
                result_lines.push(format!("{}{}", original_indent_chars, line));
            } else {
                // Other lines: keep as is
                result_lines.push(line.to_string());
            }
        }

        result_lines.join("\n")
    }

    pub fn extract_line_indent_chars_corrected(
        source_code: &str,
        line_row: usize,
        indent_byte_length: usize,
    ) -> String {
        let lines: Vec<&str> = source_code.lines().collect();
        if line_row < lines.len() {
            let line = lines[line_row];
            // Convert byte position to character position first
            if indent_byte_length <= line.len() {
                let indent_char_count = line[..indent_byte_length].chars().count();
                line.chars().take(indent_char_count).collect()
            } else {
                // If byte length exceeds line length, take all characters
                line.to_string()
            }
        } else {
            String::new()
        }
    }
}
