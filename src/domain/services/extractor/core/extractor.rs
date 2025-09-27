use super::super::parsers::{get_parser_registry, parse_with_thread_local};
use crate::domain::models::CodeChunk;
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

        // Pre-compute byte-to-char mapping and line cache for performance
        let byte_to_char_cache = Self::build_byte_to_char_cache(source_code);
        let line_cache = Self::build_line_cache(source_code);

        // Extract comment ranges
        let file_comment_ranges =
            Self::extract_comment_ranges(tree, source_code, language, &byte_to_char_cache)?;

        // Create query
        let query = registry.create_query(language)?;

        // Find git root
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

        let language_string = language.to_string();

        // Extract standard function/class chunks
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        let mut all_captures = Vec::new();
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let capture_index = capture.index as usize;
                all_captures.push((node, capture_index));
            }
        }

        let standard_chunks: Vec<_> = all_captures
            .iter()
            .filter_map(|(node, capture_index)| {
                let capture_name = &query.capture_names()[*capture_index];
                Self::node_to_chunk_cached(
                    *node,
                    source_code,
                    &relative_file_path,
                    &language_string,
                    capture_name,
                    &file_comment_ranges,
                    &byte_to_char_cache,
                    &line_cache,
                )
            })
            .collect();

        // Middle chunk processing
        let middle_query = registry.create_middle_implementation_query(language)?;

        let large_chunks: Vec<_> = standard_chunks
            .iter()
            .filter(|chunk| chunk.content.len() >= 10 && chunk.content.lines().count() >= 3)
            .collect();

        let mut middle_chunks = Vec::new();

        // Process each large chunk individually
        for large_chunk in large_chunks {
            let mut chunk_parser = registry.create_parser(language)?;
            let chunk_tree = match chunk_parser.parse(&large_chunk.content, None) {
                Some(tree) => tree,
                None => continue,
            };

            let mut chunk_cursor = QueryCursor::new();
            let mut chunk_matches = chunk_cursor.matches(
                &middle_query,
                chunk_tree.root_node(),
                large_chunk.content.as_bytes(),
            );

            let chunk_byte_to_char_cache = Self::build_byte_to_char_cache(&large_chunk.content);
            let chunk_comment_ranges = Self::extract_comment_ranges(
                &chunk_tree,
                &large_chunk.content,
                language,
                &chunk_byte_to_char_cache,
            )?;

            while let Some(match_) = chunk_matches.next() {
                for capture in match_.captures {
                    let node = capture.node;
                    let capture_name = &middle_query.capture_names()[capture.index as usize];

                    if let Some(chunk_type) = registry
                        .get_extractor(language)?
                        .middle_capture_name_to_chunk_type(capture_name)
                    {
                        let start_byte = node.start_byte();
                        let end_byte = node.end_byte();
                        let content = &large_chunk.content[start_byte..end_byte];

                        let line_count = content.lines().count();
                        if line_count >= 2 && content.len() >= 30 && content.len() <= 2000 {
                            let middle_chunk = Self::build_middle_chunk_local(
                                node,
                                capture_name,
                                chunk_type,
                                content,
                                start_byte,
                                end_byte,
                                source_code,
                                &large_chunk.file_path,
                                &language_string,
                                &chunk_comment_ranges,
                                &chunk_byte_to_char_cache,
                                large_chunk.start_line,
                                &line_cache,
                            );
                            middle_chunks.push(middle_chunk);
                        }
                    }
                }
            }
        }

        // Add all chunks to final result
        chunks.extend(standard_chunks);
        chunks.extend(middle_chunks);

        // Zen chunk creation
        let line_count = source_code.lines().count();
        let zen_chunk = CodeChunk {
            content: source_code.to_string(),
            file_path: relative_file_path.to_owned(),
            start_line: 1,
            end_line: line_count,
            language: language_string.to_owned(),
            chunk_type: crate::domain::models::ChunkType::File,
            name: "entire_file".to_string(),
            comment_ranges: file_comment_ranges,
            original_indentation: 0,
        };
        chunks.push(zen_chunk);

        // Sorting and deduplication
        chunks.sort_by(|a, b| {
            let pos_cmp = (a.start_line, a.end_line).cmp(&(b.start_line, b.end_line));
            if pos_cmp == std::cmp::Ordering::Equal {
                let a_priority = match a.chunk_type {
                    crate::domain::models::ChunkType::Function => 0,
                    crate::domain::models::ChunkType::Class => 0,
                    crate::domain::models::ChunkType::Method => 0,
                    crate::domain::models::ChunkType::CodeBlock => 10,
                    crate::domain::models::ChunkType::File => 20,
                    _ => 5,
                };
                let b_priority = match b.chunk_type {
                    crate::domain::models::ChunkType::Function => 0,
                    crate::domain::models::ChunkType::Class => 0,
                    crate::domain::models::ChunkType::Method => 0,
                    crate::domain::models::ChunkType::CodeBlock => 10,
                    crate::domain::models::ChunkType::File => 20,
                    _ => 5,
                };
                a_priority.cmp(&b_priority)
            } else {
                pos_cmp
            }
        });

        chunks.dedup_by(|a, b| {
            a.start_line == b.start_line
                && a.end_line == b.end_line
                && a.chunk_type != crate::domain::models::ChunkType::File
                && b.chunk_type != crate::domain::models::ChunkType::File
        });

        Ok(chunks)
    }

    /// Build middle chunk with local coordinates (chunk-relative positioning)
    #[allow(clippy::too_many_arguments)]
    fn build_middle_chunk_local(
        node: tree_sitter::Node,
        capture_name: &str,
        chunk_type: crate::domain::models::ChunkType,
        content: &str,
        start_byte: usize,
        end_byte: usize,
        file_source_code: &str,
        file_path: &std::path::PathBuf,
        language_string: &str,
        chunk_comment_ranges: &[(usize, usize)],
        chunk_byte_to_char_cache: &[usize],
        parent_start_line: usize,
        file_line_cache: &[usize],
    ) -> crate::domain::models::CodeChunk {
        // Use cached byte-to-char conversion for chunk-local coordinates
        let start_char = Self::byte_to_char_cached(chunk_byte_to_char_cache, start_byte);
        let end_char = Self::byte_to_char_cached(chunk_byte_to_char_cache, end_byte);

        // Calculate absolute line numbers by adding parent chunk's start line
        let relative_start_line = node.start_position().row + 1;
        let relative_end_line = node.end_position().row + 1;
        let absolute_start_line = parent_start_line + relative_start_line - 1;
        let absolute_end_line = parent_start_line + relative_end_line - 1;

        let original_indentation_bytes = node.start_position().column;

        // Extract actual indentation characters from file source (zero-copy with line cache)
        let original_indent_chars = if original_indentation_bytes > 0 {
            Self::extract_line_indent_chars_cached(
                file_source_code,
                absolute_start_line - 1, // Convert to 0-based for file line cache
                original_indentation_bytes,
                file_line_cache,
            )
        } else {
            ""
        };

        let normalized_content =
            Self::normalize_first_line_indentation(content, original_indent_chars);

        // Adjust comment ranges to be relative to the normalized content (chunk-local)
        let indent_offset_chars = original_indent_chars.chars().count();

        let chunk_local_comment_ranges: Vec<(usize, usize)> = chunk_comment_ranges
            .iter()
            .filter_map(|&(comment_raw_pos_start, comment_raw_pos_end)| {
                // Check if comment is within this sub-chunk's boundaries
                if comment_raw_pos_start >= start_char && comment_raw_pos_end <= end_char {
                    // Convert to sub-chunk-relative positions
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

        crate::domain::models::CodeChunk {
            name: capture_name.to_owned(),
            content: normalized_content,
            chunk_type,
            language: language_string.to_owned(),
            file_path: file_path.to_owned(),
            start_line: absolute_start_line,
            end_line: absolute_end_line,
            comment_ranges: chunk_local_comment_ranges,
            original_indentation: indent_offset_chars,
        }
    }

    pub fn extract_from_file(file_path: &Path, language: &str) -> Result<Vec<CodeChunk>> {
        let content = fs::read_to_string(file_path)?;
        let tree = parse_with_thread_local(language, &content).ok_or_else(|| {
            GitTypeError::ExtractionFailed(format!("Failed to parse file: {:?}", file_path))
        })?;

        Self::extract_chunks_from_tree(&tree, &content, file_path, language)
    }

    /// Cached version of extract_comment_ranges
    pub fn extract_comment_ranges(
        tree: &Tree,
        source_code: &str,
        language: &str,
        byte_to_char_cache: &[usize],
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
                    // Use cached byte-to-char conversion
                    let start_char = Self::byte_to_char_cached(byte_to_char_cache, start_byte);
                    let end_char = Self::byte_to_char_cached(byte_to_char_cache, end_byte);
                    comment_ranges.push((start_char, end_char));
                }
            }
        }

        comment_ranges.sort_by_key(|&(start, _)| start);
        Ok(comment_ranges)
    }

    pub fn extract_line_indent_chars_corrected(
        source_code: &str,
        line_row: usize,
        indent_byte_length: usize,
    ) -> &str {
        // Find the target line without collecting all lines into Vec
        let mut current_line = 0;
        let mut line_start = 0;

        for (i, byte) in source_code.bytes().enumerate() {
            if byte == b'\n' {
                if current_line == line_row {
                    // Found target line: extract indentation
                    let line = &source_code[line_start..i];
                    if indent_byte_length == 0 {
                        return "";
                    }
                    if indent_byte_length <= line.len() {
                        return &line[..indent_byte_length];
                    } else {
                        return line;
                    }
                }
                current_line += 1;
                line_start = i + 1;
            }
        }

        // Handle last line (no trailing newline)
        if current_line == line_row {
            let line = &source_code[line_start..];
            if indent_byte_length == 0 {
                return "";
            }
            if indent_byte_length <= line.len() {
                return &line[..indent_byte_length];
            } else {
                return line;
            }
        }

        ""
    }

    /// Build a byte-to-char position cache for fast lookups
    fn build_byte_to_char_cache(source_code: &str) -> Vec<usize> {
        let mut cache = Vec::with_capacity(source_code.len() + 1);
        let mut char_count = 0;

        cache.push(0); // Position 0 = 0 chars

        for (byte_pos, _) in source_code.char_indices() {
            // Fill gaps for multi-byte characters
            while cache.len() <= byte_pos {
                cache.push(char_count);
            }
            char_count += 1;
            cache.push(char_count);
        }

        // Fill remaining positions to end of string
        while cache.len() <= source_code.len() {
            cache.push(char_count);
        }

        cache
    }

    /// Build a line-to-byte position cache for fast line lookups
    fn build_line_cache(source_code: &str) -> Vec<usize> {
        let mut line_starts = Vec::new();
        line_starts.push(0); // Line 0 starts at byte 0

        for (i, byte) in source_code.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(i + 1); // Next line starts after the newline
            }
        }

        line_starts
    }

    /// Fast indent extraction using line cache
    fn extract_line_indent_chars_cached<'a>(
        source_code: &'a str,
        line_row: usize,
        indent_byte_length: usize,
        line_cache: &[usize],
    ) -> &'a str {
        if indent_byte_length == 0 {
            return "";
        }

        // Get line boundaries from cache (O(1) lookup)
        let line_start = line_cache
            .get(line_row)
            .copied()
            .unwrap_or(source_code.len());
        let line_end = line_cache
            .get(line_row + 1)
            .copied()
            .unwrap_or(source_code.len());

        if line_start >= source_code.len() {
            return "";
        }

        // Extract the line (without newline)
        let line_end_adjusted =
            if line_end > line_start && source_code.as_bytes().get(line_end - 1) == Some(&b'\n') {
                line_end - 1
            } else {
                line_end
            };

        let line = &source_code[line_start..line_end_adjusted];

        // Return indent portion
        if indent_byte_length <= line.len() {
            &line[..indent_byte_length]
        } else {
            line
        }
    }

    /// Fast byte-to-char lookup using pre-computed cache
    fn byte_to_char_cached(cache: &[usize], byte_pos: usize) -> usize {
        if byte_pos >= cache.len() {
            cache.last().copied().unwrap_or(0)
        } else {
            cache[byte_pos]
        }
    }

    /// Cached version of node_to_chunk
    #[allow(clippy::too_many_arguments)]
    fn node_to_chunk_cached(
        node: tree_sitter::Node,
        source_code: &str,
        file_path: &Path,
        language: &str,
        capture_name: &str,
        file_comment_ranges: &[(usize, usize)],
        byte_to_char_cache: &[usize],
        line_cache: &[usize],
    ) -> Option<crate::domain::models::CodeChunk> {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = &source_code[start_byte..end_byte];

        // Skip if content is too small or empty
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() || content.len() < 10 {
            return None;
        }

        // Use cached byte-to-char conversion instead of slow char iteration
        let start_char = Self::byte_to_char_cached(byte_to_char_cache, start_byte);
        let end_char = Self::byte_to_char_cached(byte_to_char_cache, end_byte);

        let start_line = node.start_position().row + 1;
        let end_line = node.end_position().row + 1;
        let original_indentation_bytes = node.start_position().column;

        // Extract actual indentation characters from source (zero-copy with line cache)
        let original_indent_chars = if original_indentation_bytes > 0 {
            Self::extract_line_indent_chars_cached(
                source_code,
                node.start_position().row,
                original_indentation_bytes,
                line_cache,
            )
        } else {
            ""
        };

        let normalized_content =
            Self::normalize_first_line_indentation(content, original_indent_chars);

        // Adjust comment ranges to be relative to the normalized content
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

        let chunk_type = match capture_name {
            "function" => crate::domain::models::ChunkType::Function,
            "class" => crate::domain::models::ChunkType::Class,
            "method" => crate::domain::models::ChunkType::Method,
            _ => crate::domain::models::ChunkType::CodeBlock,
        };

        Some(crate::domain::models::CodeChunk {
            name: capture_name.to_owned(),
            content: normalized_content,
            chunk_type,
            language: language.to_owned(),
            file_path: file_path.to_path_buf(),
            start_line,
            end_line,
            comment_ranges: chunk_comment_ranges,
            original_indentation: indent_offset_chars,
        })
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

    fn normalize_first_line_indentation(content: &str, original_indent_chars: &str) -> String {
        // Zero-copy optimization: if no indentation needed, return content as-is
        if original_indent_chars.is_empty() {
            return content.to_owned();
        }

        if content.is_empty() {
            return original_indent_chars.to_owned();
        }

        // Find newline using memchr-like approach (fastest)
        let newline_pos = content.as_bytes().iter().position(|&b| b == b'\n');

        let mut result = String::with_capacity(original_indent_chars.len() + content.len());
        result.push_str(original_indent_chars);

        if let Some(pos) = newline_pos {
            // Multi-line: add first line, then rest
            result.push_str(&content[..pos]);
            result.push_str(&content[pos..]);
        } else {
            // Single line: add everything
            result.push_str(content);
        }

        result
    }
}
