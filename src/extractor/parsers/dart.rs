use super::LanguageExtractor;
use crate::extractor::models::{ChunkType, Language};
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct DartExtractor;

impl LanguageExtractor for DartExtractor {
    fn language(&self) -> Language {
        Language::Dart
    }

    fn file_extensions(&self) -> &[&str] {
        &["dart"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_dart::language()
    }

    fn query_patterns(&self) -> &str {
        "
            (class_definition (identifier) @name) @class
            (enum_declaration (identifier) @name) @enum
            (mixin_declaration (identifier) @name) @mixin
            (extension_declaration (identifier) @name) @extension
            (lambda_expression (function_signature (identifier) @name)) @function
            (method_signature (function_signature (identifier) @name)) @method
            (local_variable_declaration (initialized_variable_definition (identifier) @name)) @variable
        "
    }

    fn comment_query(&self) -> &str {
        "[(comment) (documentation_comment)] @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "method" => Some(ChunkType::Function),
            "class" => Some(ChunkType::Class),
            "enum" => Some(ChunkType::Enum),
            "mixin" => Some(ChunkType::Class),
            "extension" => Some(ChunkType::Class),
            "variable" => Some(ChunkType::Variable),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
        self.extract_name_from_node(node, source_code)
    }
}

impl DartExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        // First try direct identifier children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
                    let start = child.start_byte();
                    let end = child.end_byte();
                    return Some(source_code[start..end].to_string());
                }

                // For function signatures, recursively look for identifier
                if child.kind() == "function_signature" {
                    if let Some(name) = self.extract_name_from_node(child, source_code) {
                        return Some(name);
                    }
                }

                // For variable declarations, look deeper
                if child.kind() == "initialized_variable_definition" {
                    if let Some(name) = self.extract_name_from_node(child, source_code) {
                        return Some(name);
                    }
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        None
    }

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_dart::language())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Dart language: {}", e))
            })?;
        Ok(parser)
    }
}
