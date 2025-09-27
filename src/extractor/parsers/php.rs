use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct PhpExtractor;

impl LanguageExtractor for PhpExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_php::LANGUAGE_PHP.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_definition name: (name) @name) @function
            (method_declaration name: (name) @name) @method
            (class_declaration name: (name) @name) @class
            (interface_declaration name: (name) @name) @interface
            (trait_declaration name: (name) @name) @trait
            (namespace_definition name: (namespace_name (name) @name)) @namespace
        "
    }

    fn comment_query(&self) -> &str {
        "
            (comment) @comment
        "
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "method" => Some(ChunkType::Function),
            "class" => Some(ChunkType::Class),
            "interface" => Some(ChunkType::Class),
            "trait" => Some(ChunkType::Class),
            "namespace" => Some(ChunkType::Function),
            "name" => None, // name captures are not chunks themselves
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        // For @name captures, the node is already the name node
        if capture_name == "name" {
            let start = node.start_byte();
            let end = node.end_byte();
            return Some(source_code[start..end].to_string());
        }

        // Fallback to searching for name child
        self.extract_name_from_node(node, source_code)
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_statement) @for_loop
        (foreach_statement) @foreach_loop
        (while_statement) @while_loop
        (if_statement) @if_block
        (try_statement) @try_block
        (switch_statement) @switch_block
        (function_call_expression) @function_call
        (compound_statement) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "foreach_loop" | "while_loop" => Some(ChunkType::Loop),
            "if_block" | "switch_block" => Some(ChunkType::Conditional),
            "try_block" => Some(ChunkType::ErrorHandling),
            "function_call" => Some(ChunkType::FunctionCall),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl PhpExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "name" {
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

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_php::LANGUAGE_PHP.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set PHP language: {}", e))
            })?;
        Ok(parser)
    }
}
