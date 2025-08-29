use std::path::Path;
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor, Node, Tree};
use ignore::WalkBuilder;
use crate::{Result, GitTypeError};
use super::{CodeChunk, Language, ChunkType, ProgressReporter, NoOpProgressReporter};

#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub max_lines: Option<usize>,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            include_patterns: vec!["**/*.rs".to_string(), "**/*.ts".to_string(), "**/*.tsx".to_string(), "**/*.py".to_string()],
            exclude_patterns: vec!["**/target/**".to_string(), "**/node_modules/**".to_string(), "**/__pycache__/**".to_string()],
            max_lines: None,
        }
    }
}

pub struct CodeExtractor {
    rust_parser: Parser,
    typescript_parser: Parser,
    python_parser: Parser,
}

impl CodeExtractor {
    pub fn new() -> Result<Self> {
        let mut rust_parser = Parser::new();
        rust_parser.set_language(tree_sitter_rust::language())
            .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to set Rust language: {}", e)))?;

        let mut typescript_parser = Parser::new();
        typescript_parser.set_language(tree_sitter_typescript::language_typescript())
            .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to set TypeScript language: {}", e)))?;

        let mut python_parser = Parser::new();
        python_parser.set_language(tree_sitter_python::language())
            .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to set Python language: {}", e)))?;

        Ok(Self {
            rust_parser,
            typescript_parser,
            python_parser,
        })
    }

    pub fn extract_chunks(&mut self, repo_path: &Path, options: ExtractionOptions) -> Result<Vec<CodeChunk>> {
        self.extract_chunks_with_progress(repo_path, options, &NoOpProgressReporter)
    }

    pub fn extract_chunks_with_progress<P: ProgressReporter + ?Sized>(
        &mut self,
        repo_path: &Path,
        options: ExtractionOptions,
        progress: &P,
    ) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();
        
        progress.set_phase("Scanning repository".to_string());
        
        // Use ignore crate to respect .gitignore files
        let walker = WalkBuilder::new(repo_path)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .git_global(true) // Respect global gitignore
            .git_exclude(true) // Respect .git/info/exclude
            .build();
        
        // Collect all files to process first to get total count
        let mut files_to_process = Vec::new();
        for entry in walker {
            let entry = entry.map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }

            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                if let Some(language) = Language::from_extension(extension) {
                    if self.should_process_file(path, &options) {
                        files_to_process.push((path.to_path_buf(), language));
                    }
                }
            }
        }

        let total_files = files_to_process.len();
        progress.set_phase("Parsing AST".to_string());

        for (i, (path, language)) in files_to_process.iter().enumerate() {
            progress.set_file_counts(i + 1, total_files);
            if i % 3 == 0 { // Update spinner every 3 files to reduce flicker
                progress.update_spinner();
            }
            
            let file_chunks = self.extract_from_file(path, *language, &options)?;
            chunks.extend(file_chunks);
        }

        progress.set_file_counts(total_files, total_files);
        progress.set_current_file(None);
        progress.set_phase("Finalizing".to_string());

        Ok(chunks)
    }

    fn should_process_file(&self, path: &Path, options: &ExtractionOptions) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check exclude patterns first
        for pattern in &options.exclude_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false) {
                return false;
            }
        }
        
        // Check include patterns
        for pattern in &options.include_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false) {
                return true;
            }
        }
        
        false
    }

    pub fn extract_from_file(&mut self, file_path: &Path, language: Language, options: &ExtractionOptions) -> Result<Vec<CodeChunk>> {
        let content = fs::read_to_string(file_path)?;
        let parser = match language {
            Language::Rust => &mut self.rust_parser,
            Language::TypeScript => &mut self.typescript_parser,
            Language::Python => &mut self.python_parser,
        };
        
        let tree = parser.parse(&content, None)
            .ok_or_else(|| GitTypeError::ExtractionFailed(format!("Failed to parse file: {:?}", file_path)))?;
        
        self.extract_chunks_from_tree(&tree, &content, file_path, language, options)
    }

    fn extract_chunks_from_tree(
        &self,
        tree: &Tree,
        source_code: &str,
        file_path: &Path,
        language: Language,
        options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();
        
        // Extract comment ranges for the entire file
        let file_comment_ranges = self.extract_comment_ranges(tree, source_code, language.clone());
        
        let query_str = match language {
            Language::Rust => "
                (function_item name: (identifier) @name) @function
                (impl_item type: (type_identifier) @name) @impl
                (struct_item name: (type_identifier) @name) @struct
            ",
            Language::TypeScript => "
                (function_declaration name: (identifier) @name) @function
                (method_definition name: (property_identifier) @name) @method
                (class_declaration name: (type_identifier) @name) @class
                (variable_declarator name: (identifier) value: (arrow_function)) @arrow_function
                (variable_declarator name: (identifier) value: (function_expression)) @function_expression
            ",
            Language::Python => "
                (function_definition name: (identifier) @name) @function
                (class_definition name: (identifier) @name) @class
            ",
        };

        let query = Query::new(tree.language(), query_str)
            .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to create query: {}", e)))?;
        
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());
        
        for match_ in matches {
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];
                
                if let Some(chunk) = self.node_to_chunk(node, source_code, file_path, language.clone(), &capture_name, options, &file_comment_ranges) {
                    chunks.push(chunk);
                }
            }
        }
        
        Ok(chunks)
    }

    fn node_to_chunk(
        &self,
        node: Node,
        source_code: &str,
        file_path: &Path,
        language: Language,
        capture_name: &str,
        options: &ExtractionOptions,
        file_comment_ranges: &[(usize, usize)],
    ) -> Option<CodeChunk> {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = &source_code[start_byte..end_byte];
        
        let start_line = node.start_position().row + 1;
        let end_line = node.end_position().row + 1;
        let original_indentation = node.start_position().column;
        
        if let Some(max_lines) = options.max_lines {
            if end_line - start_line + 1 > max_lines {
                return None;
            }
        }
        
        let chunk_type = match capture_name {
            "function" => ChunkType::Function,
            "method" => ChunkType::Method,
            "class" | "impl" => ChunkType::Class,
            "struct" => ChunkType::Struct,
            "arrow_function" => ChunkType::Function,
            "function_expression" => ChunkType::Function,
            _ => return None,
        };
        
        let name = self.extract_name(node, source_code).unwrap_or_else(|| "unknown".to_string());
        
        // Filter comment ranges that are within this chunk and make them relative to chunk content
        let chunk_comment_ranges: Vec<(usize, usize)> = file_comment_ranges.iter()
            .filter_map(|&(comment_start, comment_end)| {
                if comment_start >= start_byte && comment_end <= end_byte {
                    // Convert to relative position within chunk content
                    Some((comment_start - start_byte, comment_end - start_byte))
                } else {
                    None
                }
            })
            .collect();
        
        // Normalize indentation based on AST node position
        let (normalized_content, normalized_comment_ranges) = self.normalize_indentation(
            content,
            original_indentation,
            &chunk_comment_ranges
        );
        
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
    
    fn extract_name(&self, node: Node, source_code: &str) -> Option<String> {
        // For variable_declarator, we need to get the name from the first child
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
        
        // Try to find identifier child node for other cases
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" || child.kind() == "type_identifier" || child.kind() == "property_identifier" {
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

    fn normalize_indentation(&self, content: &str, original_indentation: usize, comment_ranges: &[(usize, usize)]) -> (String, Vec<(usize, usize)>) {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return (content.to_string(), comment_ranges.to_vec());
        }

        // Create character-by-character position mapping during normalization
        let mut position_map = Vec::new();
        let mut normalized_lines = Vec::new();
        let mut _original_pos = 0;
        let mut normalized_pos = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_chars: Vec<char> = line.chars().collect();
            
            if line_idx == 0 {
                // Keep first line as-is
                for _ in &line_chars {
                    position_map.push(Some(normalized_pos));
                    normalized_pos += 1;
                    _original_pos += 1;
                }
                normalized_lines.push(line.to_string());
            } else if line.trim().is_empty() {
                // Empty line - map all positions to None (removed)
                for _ in &line_chars {
                    position_map.push(None);
                    _original_pos += 1;
                }
                normalized_lines.push(String::new());
            } else {
                // Remove original_indentation spaces from subsequent lines
                let current_indent = line.len() - line.trim_start().len();
                if current_indent >= original_indentation {
                    // Remove indentation characters
                    for i in 0..line_chars.len() {
                        if i < original_indentation {
                            position_map.push(None); // Removed indentation
                        } else {
                            position_map.push(Some(normalized_pos));
                            normalized_pos += 1;
                        }
                        _original_pos += 1;
                    }
                    normalized_lines.push(line[original_indentation..].to_string());
                } else {
                    // Keep line as-is
                    for _ in &line_chars {
                        position_map.push(Some(normalized_pos));
                        normalized_pos += 1;
                        _original_pos += 1;
                    }
                    normalized_lines.push(line.to_string());
                }
            }
            
            // Handle newline character
            if line_idx < lines.len() - 1 {
                position_map.push(Some(normalized_pos));
                normalized_pos += 1;
                _original_pos += 1;
            }
        }

        let normalized_text = normalized_lines.join("\n");

        // Map AST comment ranges using the position mapping
        let mut final_ranges = Vec::new();
        
        for &(orig_start, orig_end) in comment_ranges {
            if orig_start < position_map.len() && orig_end <= position_map.len() {
                // Find mapped start position
                let norm_start = position_map.get(orig_start).and_then(|&pos| pos);
                
                // Find mapped end position (exclusive, so we look for the position just before end)
                let norm_end = if orig_end > 0 && orig_end <= position_map.len() {
                    // Find the last mapped position before orig_end
                    (0..orig_end).rev()
                        .find_map(|i| position_map.get(i).and_then(|&pos| pos))
                        .map(|pos| pos + 1) // +1 because end is exclusive
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

    fn extract_comment_ranges(&self, tree: &Tree, source_code: &str, language: Language) -> Vec<(usize, usize)> {
        let mut comment_ranges = Vec::new();
        
        let comment_query = match language {
            Language::Rust => "[(line_comment) (block_comment)] @comment",
            Language::TypeScript => "(comment) @comment",
            Language::Python => "(comment) @comment",
        };
        
        let query = match Query::new(tree.language(), comment_query) {
            Ok(q) => q,
            Err(_) => return comment_ranges, // Fallback to empty if query fails
        };
        
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());
        
        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let start = node.start_byte();
                let end = node.end_byte();
                
                // Validate that this is actually a comment node
                let node_kind = node.kind();
                let is_valid_comment = match language {
                    Language::Rust => node_kind == "line_comment" || node_kind == "block_comment",
                    Language::TypeScript => node_kind == "comment",
                    Language::Python => node_kind == "comment",
                };
                
                if !is_valid_comment {
                    continue;
                }
                
                comment_ranges.push((start, end));
            }
        }
        
        comment_ranges.sort_by_key(|&(start, _)| start);
        comment_ranges
    }
    
}
