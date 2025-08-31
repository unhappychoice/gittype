use super::{ChunkType, CodeChunk, Language, NoOpProgressReporter, ProgressReporter};
use crate::{GitTypeError, Result};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, Query, QueryCursor, Tree};

#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            include_patterns: vec![
                "**/*.rs".to_string(),
                "**/*.ts".to_string(),
                "**/*.tsx".to_string(),
                "**/*.py".to_string(),
                "**/*.rb".to_string(),
                "**/*.go".to_string(),
            ],
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/__pycache__/**".to_string(),
            ],
        }
    }
}

pub struct CodeExtractor;

impl CodeExtractor {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    fn create_parser_for_language(language: Language) -> Result<Parser> {
        let mut parser = Parser::new();
        match language {
            Language::Rust => {
                parser
                    .set_language(tree_sitter_rust::language())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!(
                            "Failed to set Rust language: {}",
                            e
                        ))
                    })?;
            }
            Language::TypeScript => {
                parser
                    .set_language(tree_sitter_typescript::language_typescript())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!(
                            "Failed to set TypeScript language: {}",
                            e
                        ))
                    })?;
            }
            Language::Python => {
                parser
                    .set_language(tree_sitter_python::language())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!(
                            "Failed to set Python language: {}",
                            e
                        ))
                    })?;
            }
            Language::Ruby => {
                parser
                    .set_language(tree_sitter_ruby::language())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!(
                            "Failed to set Ruby language: {}",
                            e
                        ))
                    })?;
            }
            Language::Go => {
                parser
                    .set_language(tree_sitter_go::language())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!("Failed to set Go language: {}", e))
                    })?;
            }
        }
        Ok(parser)
    }

    pub fn extract_chunks(
        &mut self,
        repo_path: &Path,
        _options: ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        self.extract_chunks_with_progress(repo_path, _options, &NoOpProgressReporter)
    }

    pub fn extract_chunks_with_progress<P: ProgressReporter + ?Sized>(
        &mut self,
        repo_path: &Path,
        _options: ExtractionOptions,
        progress: &P,
    ) -> Result<Vec<CodeChunk>> {
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
            let entry =
                entry.map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                if let Some(language) = Language::from_extension(extension) {
                    if Self::should_process_file_static(path, &_options) {
                        files_to_process.push((path.to_path_buf(), language));
                    }
                }
            }
        }

        let total_files = files_to_process.len();
        progress.set_phase("Parsing AST".to_string());

        // Process files in parallel with better progress tracking
        // Split files into smaller chunks for better progress visibility
        let chunk_size = (total_files / 20).clamp(1, 10); // Process in smaller chunks of 1-10 files
        let mut all_chunks = Vec::new();
        let mut processed_files = 0;

        for chunk in files_to_process.chunks(chunk_size) {
            // Process this chunk in parallel
            let chunk_results: Result<Vec<Vec<CodeChunk>>> = chunk
                .par_iter()
                .map(|(path, language)| Self::extract_from_file_static(path, *language, &_options))
                .collect();

            // Update progress after each chunk
            processed_files += chunk.len();
            progress.set_file_counts(processed_files, total_files);

            // Update spinner for each chunk to show progress
            progress.update_spinner();

            // Collect results
            let chunk_results = chunk_results?;
            for file_chunks in chunk_results {
                all_chunks.extend(file_chunks);
            }
        }

        progress.set_file_counts(total_files, total_files);
        progress.set_current_file(None);
        progress.set_phase("Finalizing".to_string());

        Ok(all_chunks)
    }

    #[allow(dead_code)]
    fn should_process_file(&self, path: &Path, _options: &ExtractionOptions) -> bool {
        Self::should_process_file_static(path, _options)
    }

    fn should_process_file_static(path: &Path, _options: &ExtractionOptions) -> bool {
        let path_str = path.to_string_lossy();

        // Check exclude patterns first
        for pattern in &_options.exclude_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false)
            {
                return false;
            }
        }

        // Check include patterns
        for pattern in &_options.include_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false)
            {
                return true;
            }
        }

        false
    }

    pub fn extract_from_file(
        &mut self,
        file_path: &Path,
        language: Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        Self::extract_from_file_static(file_path, language, _options)
    }

    fn extract_from_file_static(
        file_path: &Path,
        language: Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        let content = fs::read_to_string(file_path)?;
        let mut parser = Self::create_parser_for_language(language)?;

        let tree = parser.parse(&content, None).ok_or_else(|| {
            GitTypeError::ExtractionFailed(format!("Failed to parse file: {:?}", file_path))
        })?;

        Self::extract_chunks_from_tree_static(&tree, &content, file_path, language, _options)
    }

    #[allow(dead_code)]
    fn extract_chunks_from_tree(
        &self,
        tree: &Tree,
        source_code: &str,
        file_path: &Path,
        language: Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        Self::extract_chunks_from_tree_static(tree, source_code, file_path, language, _options)
    }

    fn extract_chunks_from_tree_static(
        tree: &Tree,
        source_code: &str,
        file_path: &Path,
        language: Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();

        // Extract comment ranges for the entire file
        let file_comment_ranges = Self::extract_comment_ranges_static(tree, source_code, language);

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
            Language::Ruby => "
                (method name: (identifier) @name) @method
                (class name: (constant) @name) @class
                (module name: (constant) @name) @module
            ",
            Language::Go => "
                (function_declaration name: (identifier) @name) @function
                (method_declaration receiver: _ name: (field_identifier) @name) @method
                (type_spec name: (type_identifier) @name type: (struct_type)) @struct
                (type_spec name: (type_identifier) @name type: (interface_type)) @interface
            ",
        };

        let query = Query::new(tree.language(), query_str).map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Failed to create query: {}", e))
        })?;

        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        for match_ in matches {
            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &query.capture_names()[capture.index as usize];

                if let Some(chunk) = Self::node_to_chunk_static(
                    node,
                    source_code,
                    file_path,
                    language,
                    capture_name,
                    _options,
                    &file_comment_ranges,
                ) {
                    chunks.push(chunk);
                }
            }
        }

        Ok(chunks)
    }

    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    fn node_to_chunk(
        &self,
        node: Node,
        source_code: &str,
        file_path: &Path,
        language: Language,
        capture_name: &str,
        _options: &ExtractionOptions,
        file_comment_ranges: &[(usize, usize)],
    ) -> Option<CodeChunk> {
        Self::node_to_chunk_static(
            node,
            source_code,
            file_path,
            language,
            capture_name,
            _options,
            file_comment_ranges,
        )
    }

    fn node_to_chunk_static(
        node: Node,
        source_code: &str,
        file_path: &Path,
        language: Language,
        capture_name: &str,
        _options: &ExtractionOptions,
        file_comment_ranges: &[(usize, usize)],
    ) -> Option<CodeChunk> {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = &source_code[start_byte..end_byte];

        let start_line = node.start_position().row + 1;
        let end_line = node.end_position().row + 1;
        let original_indentation = node.start_position().column;


        let chunk_type = match capture_name {
            "function" => ChunkType::Function,
            "method" => ChunkType::Method,
            "class" | "impl" => ChunkType::Class,
            "struct" => ChunkType::Struct,
            "interface" => ChunkType::Interface,
            "module" => ChunkType::Module,
            "arrow_function" => ChunkType::Function,
            "function_expression" => ChunkType::Function,
            _ => return None,
        };

        let name =
            Self::extract_name_static(node, source_code).unwrap_or_else(|| "unknown".to_string());

        // Filter comment ranges that are within this chunk and make them relative to chunk content
        let chunk_comment_ranges: Vec<(usize, usize)> = file_comment_ranges
            .iter()
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
        let (normalized_content, normalized_comment_ranges) = Self::normalize_indentation_static(
            content,
            original_indentation,
            &chunk_comment_ranges,
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

    #[allow(dead_code)]
    fn extract_name(&self, node: Node, source_code: &str) -> Option<String> {
        Self::extract_name_static(node, source_code)
    }

    fn extract_name_static(node: Node, source_code: &str) -> Option<String> {
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
                if child.kind() == "identifier"
                    || child.kind() == "type_identifier"
                    || child.kind() == "property_identifier"
                    || child.kind() == "field_identifier"
                    || child.kind() == "constant"
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

    #[allow(dead_code)]
    fn normalize_indentation(
        &self,
        content: &str,
        original_indentation: usize,
        comment_ranges: &[(usize, usize)],
    ) -> (String, Vec<(usize, usize)>) {
        Self::normalize_indentation_static(content, original_indentation, comment_ranges)
    }

    fn normalize_indentation_static(
        content: &str,
        original_indentation: usize,
        comment_ranges: &[(usize, usize)],
    ) -> (String, Vec<(usize, usize)>) {
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
                    (0..orig_end)
                        .rev()
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

    #[allow(dead_code)]
    fn extract_comment_ranges(
        &self,
        tree: &Tree,
        source_code: &str,
        language: Language,
    ) -> Vec<(usize, usize)> {
        Self::extract_comment_ranges_static(tree, source_code, language)
    }

    fn extract_comment_ranges_static(
        tree: &Tree,
        source_code: &str,
        language: Language,
    ) -> Vec<(usize, usize)> {
        let mut comment_ranges = Vec::new();

        let comment_query = match language {
            Language::Rust => "[(line_comment) (block_comment)] @comment",
            Language::TypeScript => "(comment) @comment",
            Language::Python => "(comment) @comment",
            Language::Ruby => "(comment) @comment",
            Language::Go => "(comment) @comment",
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
                    Language::Ruby => node_kind == "comment",
                    Language::Go => node_kind == "comment",
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
