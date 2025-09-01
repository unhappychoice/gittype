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
                "**/*.swift".to_string(),
                "**/*.kt".to_string(),
                "**/*.kts".to_string(),
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
            Language::Swift => {
                parser
                    .set_language(tree_sitter_swift::language())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!(
                            "Failed to set Swift language: {}",
                            e
                        ))
                    })?;
            }
            Language::Kotlin => {
                parser
                    .set_language(tree_sitter_kotlin::language())
                    .map_err(|e| {
                        GitTypeError::ExtractionFailed(format!(
                            "Failed to set Kotlin language: {}",
                            e
                        ))
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
                (enum_item name: (type_identifier) @name) @enum
                (trait_item name: (type_identifier) @name) @trait
                (mod_item name: (identifier) @name) @module
                (type_item name: (type_identifier) @name) @type_alias
            ",
            Language::TypeScript => "
                (function_declaration name: (identifier) @name) @function
                (method_definition name: (property_identifier) @name) @method
                (class_declaration name: (type_identifier) @name) @class
                (variable_declarator name: (identifier) value: (arrow_function)) @arrow_function
                (variable_declarator name: (identifier) value: (function_expression)) @function_expression
                (interface_declaration name: (type_identifier) @name) @interface
                (type_alias_declaration name: (type_identifier) @name) @type_alias
                (enum_declaration name: (identifier) @name) @enum
                (internal_module name: (identifier) @name) @namespace
            ",
            Language::Python => "
                (function_definition name: (identifier) @name) @function
                (class_definition name: (identifier) @name) @class
            ",
            Language::Ruby => "
                (method name: (identifier) @name) @method
                (singleton_method object: (self) name: (identifier) @name) @class_method
                (singleton_method name: (identifier) @name) @singleton_method
                (class name: (constant) @name) @class
                (module name: (constant) @name) @module
                (call method: (identifier) @method_name (#match? @method_name \"^(attr_accessor|attr_reader|attr_writer)$\") arguments: (argument_list)) @attr_accessor
            ",
            Language::Go => "
                (function_declaration name: (identifier) @name) @function
                (method_declaration receiver: _ name: (field_identifier) @name) @method
                (type_spec name: (type_identifier) @name type: (struct_type)) @struct
                (type_spec name: (type_identifier) @name type: (interface_type)) @interface
                (const_declaration) @const_block
                (var_declaration) @var_block
                (type_spec name: (type_identifier) @name type: (type_identifier)) @type_alias
                (type_spec name: (type_identifier) @name type: (function_type)) @type_alias
                (type_spec name: (type_identifier) @name type: (pointer_type)) @type_alias
                (type_spec name: (type_identifier) @name type: (slice_type)) @type_alias
                (type_spec name: (type_identifier) @name type: (array_type)) @type_alias
                (type_spec name: (type_identifier) @name type: (map_type)) @type_alias
                (type_spec name: (type_identifier) @name type: (channel_type)) @type_alias
            ",
            Language::Swift => "
                (function_declaration name: (simple_identifier) @name) @function
                (class_declaration name: (type_identifier) @name) @class
                (protocol_declaration name: (type_identifier) @name) @protocol
                (_ 
                  declaration_kind: \"extension\"
                  name: (_) @name) @extension
            ",
            Language::Kotlin => "
                (function_declaration (simple_identifier) @name) @function
                (class_declaration (type_identifier) @name) @class
                (object_declaration (type_identifier) @name) @object
                (property_declaration (variable_declaration (simple_identifier) @name)) @property
                (companion_object) @companion
                (enum_entry (simple_identifier) @name) @enum_entry
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
            "class_method" => ChunkType::Method,
            "singleton_method" => ChunkType::Method,
            "attr_accessor" => ChunkType::Method,
            "alias" => ChunkType::Method,
            "class" | "impl" => ChunkType::Class,
            "struct" => ChunkType::Struct,
            "enum" => ChunkType::Enum,
            "trait" => ChunkType::Trait,
            "type_alias" => ChunkType::TypeAlias,
            "interface" | "protocol" => ChunkType::Interface,
            "module" | "extension" | "namespace" => ChunkType::Module,
            "arrow_function" => ChunkType::Function,
            "function_expression" => ChunkType::Function,
            "const_block" => ChunkType::Const,
            "var_block" => ChunkType::Variable,
            "object" => ChunkType::Class,
            "property" => ChunkType::Variable,
            "companion" => ChunkType::Class,
            "enum_entry" => ChunkType::Const,
            _ => return None,
        };

        let name = match capture_name {
            "const_block" => Self::extract_const_var_names_static(node, source_code, "const"),
            "var_block" => Self::extract_const_var_names_static(node, source_code, "var"),
            "attr_accessor" => Self::extract_attr_accessor_name_static(node, source_code),
            "alias" => Self::extract_alias_name_static(node, source_code),
            _ => Self::extract_name_static(node, source_code)
                .unwrap_or_else(|| "unknown".to_string()),
        };

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

    fn extract_const_var_names_static(
        node: Node,
        source_code: &str,
        declaration_type: &str,
    ) -> String {
        let mut names = Vec::new();
        let mut cursor = node.walk();

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "const_spec" || child.kind() == "var_spec" {
                    let mut spec_cursor = child.walk();
                    if spec_cursor.goto_first_child() {
                        loop {
                            let spec_child = spec_cursor.node();
                            if spec_child.kind() == "identifier" {
                                let start = spec_child.start_byte();
                                let end = spec_child.end_byte();
                                names.push(source_code[start..end].to_string());
                                break;
                            }
                            if !spec_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        if names.is_empty() {
            format!("{}_block", declaration_type)
        } else if names.len() == 1 {
            names[0].clone()
        } else {
            format!("{} ({})", names.join(", "), names.len())
        }
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
            Language::Swift => "[(comment) (multiline_comment)] @comment",
            Language::Kotlin => "[(line_comment) (multiline_comment)] @comment",
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
                    Language::Swift => node_kind == "comment" || node_kind == "multiline_comment",
                    Language::Kotlin => {
                        node_kind == "line_comment" || node_kind == "multiline_comment"
                    }
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

    fn extract_attr_accessor_name_static(node: Node, source_code: &str) -> String {
        let mut cursor = node.walk();
        let mut symbols = Vec::new();

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
                    let start = child.start_byte();
                    let end = child.end_byte();
                    let method_name = &source_code[start..end];

                    if method_name == "attr_accessor"
                        || method_name == "attr_reader"
                        || method_name == "attr_writer"
                    {
                        // Continue to find the arguments
                        if cursor.goto_next_sibling() {
                            let args_node = cursor.node();
                            if args_node.kind() == "argument_list" {
                                let mut args_cursor = args_node.walk();
                                if args_cursor.goto_first_child() {
                                    loop {
                                        let arg = args_cursor.node();
                                        if arg.kind() == "simple_symbol" {
                                            let start = arg.start_byte();
                                            let end = arg.end_byte();
                                            let symbol = &source_code[start..end];
                                            symbols
                                                .push(symbol.trim_start_matches(':').to_string());
                                        }
                                        if !args_cursor.goto_next_sibling() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        if symbols.is_empty() {
            "unknown_attr".to_string()
        } else {
            format!("{} ({})", symbols.join(", "), symbols.len())
        }
    }

    fn extract_alias_name_static(node: Node, source_code: &str) -> String {
        let mut cursor = node.walk();
        let mut alias_name = None;
        let mut original_name = None;

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
                    let start = child.start_byte();
                    let end = child.end_byte();
                    let name = &source_code[start..end];

                    if name == "alias" {
                        // Continue to find alias name and original name
                        if cursor.goto_next_sibling() {
                            let alias_node = cursor.node();
                            if alias_node.kind() == "identifier" {
                                let start = alias_node.start_byte();
                                let end = alias_node.end_byte();
                                alias_name = Some(&source_code[start..end]);

                                if cursor.goto_next_sibling() {
                                    let orig_node = cursor.node();
                                    if orig_node.kind() == "identifier" {
                                        let start = orig_node.start_byte();
                                        let end = orig_node.end_byte();
                                        original_name = Some(&source_code[start..end]);
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        match (alias_name, original_name) {
            (Some(alias), Some(original)) => format!("{} -> {}", alias, original),
            (Some(alias), None) => alias.to_string(),
            _ => "unknown_alias".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ruby_class_method_extraction() {
        let ruby_code = r#"
class User
  def self.find_by_email(email)
    puts "Finding user by email: #{email}"
  end
  
  def instance_method
    "instance"
  end
end
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(ruby_code.as_bytes()).unwrap();
        file.flush().unwrap();

        let mut extractor = CodeExtractor::new().unwrap();
        let options = ExtractionOptions::default();
        let chunks = extractor
            .extract_from_file(file.path(), Language::Ruby, &options)
            .unwrap();

        let class_methods: Vec<_> = chunks
            .iter()
            .filter(|chunk| chunk.name.contains("find_by_email"))
            .collect();
        assert!(
            !class_methods.is_empty(),
            "Should extract class method find_by_email"
        );

        let instance_methods: Vec<_> = chunks
            .iter()
            .filter(|chunk| chunk.name.contains("instance_method"))
            .collect();
        assert!(
            !instance_methods.is_empty(),
            "Should extract instance method"
        );
    }

    #[test]
    fn test_ruby_attr_accessor_extraction() {
        let ruby_code = r#"
class Product
  attr_accessor :name, :price
  attr_reader :id
  attr_writer :description
end
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(ruby_code.as_bytes()).unwrap();
        file.flush().unwrap();

        let mut extractor = CodeExtractor::new().unwrap();
        let options = ExtractionOptions::default();
        let chunks = extractor
            .extract_from_file(file.path(), Language::Ruby, &options)
            .unwrap();

        let attr_chunks: Vec<_> = chunks
            .iter()
            .filter(|chunk| {
                chunk.name.contains("name")
                    || chunk.name.contains("price")
                    || chunk.name.contains("id")
                    || chunk.name.contains("description")
            })
            .collect();
        assert!(
            !attr_chunks.is_empty(),
            "Should extract attr_accessor, attr_reader, attr_writer"
        );
    }

    #[test]
    fn test_kotlin_class_extraction() {
        let kotlin_code = r#"
class MainActivity : AppCompatActivity() {
    companion object {
        const val TAG = "MainActivity"
        
        fun staticMethod(): String {
            return "Hello from static method"
        }
    }
    
    private val name: String = "GitType"
    
    fun greetUser(username: String): String {
        return "Hello, $username! Welcome to $name."
    }
}

data class User(
    val id: Long,
    val name: String,
    val email: String
) {
    fun getDisplayName(): String = "$name ($email)"
}

object DatabaseHelper {
    fun connect(): Connection {
        return DriverManager.getConnection("jdbc:sqlite:app.db")
    }
}
"#;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(kotlin_code.as_bytes()).unwrap();
        file.flush().unwrap();

        let mut extractor = CodeExtractor::new().unwrap();
        let options = ExtractionOptions::default();
        let chunks = extractor
            .extract_from_file(file.path(), Language::Kotlin, &options)
            .unwrap();

        let class_chunks: Vec<_> = chunks
            .iter()
            .filter(|chunk| chunk.name.contains("MainActivity") || chunk.name.contains("User"))
            .collect();
        assert!(
            !class_chunks.is_empty(),
            "Should extract classes MainActivity and User"
        );

        let function_chunks: Vec<_> = chunks
            .iter()
            .filter(|chunk| {
                chunk.name.contains("greetUser")
                    || chunk.name.contains("getDisplayName")
                    || chunk.name.contains("staticMethod")
                    || chunk.name.contains("connect")
            })
            .collect();
        assert!(
            !function_chunks.is_empty(),
            "Should extract functions from classes and objects"
        );

        let object_chunks: Vec<_> = chunks
            .iter()
            .filter(|chunk| chunk.name.contains("DatabaseHelper"))
            .collect();
        assert!(
            !object_chunks.is_empty(),
            "Should extract object declarations"
        );
    }
}
