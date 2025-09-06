use crate::extractor::parsers::{get_parser_registry, parse_with_thread_local};
use crate::models::CodeChunk;
use crate::{GitTypeError, Result};
use std::fs;
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, QueryCursor, Tree};

pub struct CommonExtractor;

impl CommonExtractor {
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

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];

                if let Some(chunk) = Self::node_to_chunk(
                    node,
                    source_code,
                    file_path,
                    language,
                    capture_name,
                    &file_comment_ranges,
                ) {
                    chunks.push(chunk);
                }
            }
        }

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
                let start = node.start_byte();
                let end = node.end_byte();

                if Self::is_valid_comment_node(node, language) {
                    comment_ranges.push((start, end));
                }
            }
        }

        comment_ranges.sort_by_key(|&(start, _)| start);
        Ok(comment_ranges)
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
            _ => false,
        }
    }

    fn node_to_chunk(
        node: Node,
        source_code: &str,
        file_path: &Path,
        language: &str,
        capture_name: &str,
        file_comment_ranges: &[(usize, usize)],
    ) -> Option<CodeChunk> {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = &source_code[start_byte..end_byte];

        let start_line = node.start_position().row + 1;
        let end_line = node.end_position().row + 1;
        let original_indentation = node.start_position().column;

        // Extract actual indentation characters from source
        let original_indent_chars = if original_indentation > 0 {
            Self::extract_line_indent_chars(
                source_code,
                node.start_position().row,
                original_indentation,
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

        let chunk_comment_ranges: Vec<(usize, usize)> = file_comment_ranges
            .iter()
            .filter_map(|&(comment_start, comment_end)| {
                if comment_start >= start_byte && comment_end <= end_byte {
                    Some((comment_start - start_byte, comment_end - start_byte))
                } else {
                    None
                }
            })
            .collect();

        let normalized_content = Self::normalize_first_line_indentation(
            content,
            &original_indent_chars,
        );

        Some(CodeChunk {
            content: normalized_content,
            file_path: file_path.to_path_buf(),
            start_line,
            end_line,
            language: language.to_string(),
            chunk_type,
            name,
            comment_ranges: chunk_comment_ranges,
            original_indentation,
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

    fn normalize_first_line_indentation(
        content: &str,
        original_indent_chars: &str,
    ) -> String {
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

    fn extract_line_indent_chars(
        source_code: &str,
        line_row: usize,
        indent_length: usize,
    ) -> String {
        let lines: Vec<&str> = source_code.lines().collect();
        if line_row < lines.len() {
            let line = lines[line_row];
            line.chars().take(indent_length).collect()
        } else {
            String::new()
        }
    }
}
