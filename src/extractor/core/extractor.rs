use crate::extractor::models::{CodeChunk, Language};
use crate::extractor::parsers::{get_parser_registry, parse_with_thread_local};
use crate::{GitTypeError, Result};
use std::fs;
use std::path::Path;
use tree_sitter::{Node, QueryCursor, Tree};

pub struct CommonExtractor;

impl CommonExtractor {
    pub fn extract_chunks_from_tree(
        tree: &Tree,
        source_code: &str,
        file_path: &Path,
        language: Language,
    ) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();
        let registry = get_parser_registry();

        let file_comment_ranges = Self::extract_comment_ranges(tree, source_code, language)?;
        let query = registry.create_query(language)?;

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        for match_ in matches {
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

    pub fn extract_from_file(file_path: &Path, language: Language) -> Result<Vec<CodeChunk>> {
        let content = fs::read_to_string(file_path)?;
        // Reuse per-thread parser instance for the language
        let tree = parse_with_thread_local(language, &content).ok_or_else(|| {
            GitTypeError::ExtractionFailed(format!("Failed to parse file: {:?}", file_path))
        })?;

        Self::extract_chunks_from_tree(&tree, &content, file_path, language)
    }

    fn extract_comment_ranges(
        tree: &Tree,
        source_code: &str,
        language: Language,
    ) -> Result<Vec<(usize, usize)>> {
        let registry = get_parser_registry();
        let comment_query = registry.create_comment_query(language)?;
        let mut comment_ranges = Vec::new();

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&comment_query, tree.root_node(), source_code.as_bytes());

        for m in matches {
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

    fn is_valid_comment_node(node: Node, language: Language) -> bool {
        let node_kind = node.kind();
        match language {
            Language::Rust => node_kind == "line_comment" || node_kind == "block_comment",
            Language::TypeScript => node_kind == "comment",
            Language::JavaScript => node_kind == "comment",
            Language::Python => node_kind == "comment",
            Language::Ruby => node_kind == "comment",
            Language::Go => node_kind == "comment",
            Language::Swift => node_kind == "comment" || node_kind == "multiline_comment",
            Language::Kotlin => node_kind == "line_comment" || node_kind == "multiline_comment",
            Language::Java => node_kind == "line_comment" || node_kind == "block_comment",
            Language::Php => node_kind == "comment" || node_kind == "shell_comment_line",
            Language::CSharp => node_kind == "comment",
        }
    }

    fn node_to_chunk(
        node: Node,
        source_code: &str,
        file_path: &Path,
        language: Language,
        capture_name: &str,
        file_comment_ranges: &[(usize, usize)],
    ) -> Option<CodeChunk> {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = &source_code[start_byte..end_byte];

        let start_line = node.start_position().row + 1;
        let end_line = node.end_position().row + 1;
        let original_indentation = node.start_position().column;

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

        let (normalized_content, normalized_comment_ranges) =
            Self::normalize_indentation(content, original_indentation, &chunk_comment_ranges);

        Some(CodeChunk {
            content: normalized_content,
            file_path: file_path.to_path_buf(),
            start_line,
            end_line,
            language,
            chunk_type,
            name,
            comment_ranges: normalized_comment_ranges,
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

    fn normalize_indentation(
        content: &str,
        original_indentation: usize,
        comment_ranges: &[(usize, usize)],
    ) -> (String, Vec<(usize, usize)>) {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return (content.to_string(), comment_ranges.to_vec());
        }

        let mut position_map = Vec::new();
        let mut normalized_lines = Vec::new();
        let mut _original_pos = 0;
        let mut normalized_pos = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_chars: Vec<char> = line.chars().collect();

            if line_idx == 0 {
                for _ in &line_chars {
                    position_map.push(Some(normalized_pos));
                    normalized_pos += 1;
                    _original_pos += 1;
                }
                normalized_lines.push(line.to_string());
            } else if line.trim().is_empty() {
                for _ in &line_chars {
                    position_map.push(None);
                    _original_pos += 1;
                }
                normalized_lines.push(String::new());
            } else {
                let current_indent = line.len() - line.trim_start().len();
                if current_indent >= original_indentation {
                    for i in 0..line_chars.len() {
                        if i < original_indentation {
                            position_map.push(None);
                        } else {
                            position_map.push(Some(normalized_pos));
                            normalized_pos += 1;
                        }
                        _original_pos += 1;
                    }
                    normalized_lines.push(line[original_indentation..].to_string());
                } else {
                    for _ in &line_chars {
                        position_map.push(Some(normalized_pos));
                        normalized_pos += 1;
                        _original_pos += 1;
                    }
                    normalized_lines.push(line.to_string());
                }
            }

            if line_idx < lines.len() - 1 {
                position_map.push(Some(normalized_pos));
                normalized_pos += 1;
                _original_pos += 1;
            }
        }

        let normalized_text = normalized_lines.join("\n");
        let mut final_ranges = Vec::new();

        for &(orig_start, orig_end) in comment_ranges {
            if orig_start < position_map.len() && orig_end <= position_map.len() {
                let norm_start = position_map.get(orig_start).and_then(|&pos| pos);
                let norm_end = if orig_end > 0 && orig_end <= position_map.len() {
                    (0..orig_end)
                        .rev()
                        .find_map(|i| position_map.get(i).and_then(|&pos| pos))
                        .map(|pos| pos + 1)
                } else {
                    None
                };

                if let (Some(start), Some(end)) = (norm_start, norm_end) {
                    if start < end && end <= normalized_text.len() {
                        final_ranges.push((start, end));
                    }
                }
            }
        }

        (normalized_text, final_ranges)
    }
}
