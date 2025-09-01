use super::LanguageExtractor;
use crate::extractor::models::{ChunkType, Language};
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct PhpExtractor;

impl LanguageExtractor for PhpExtractor {
    fn language(&self) -> Language {
        Language::Php
    }

    fn file_extensions(&self) -> &[&str] {
        &["php", "phtml", "php3", "php4", "php5"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_php::language_php()
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
            .set_language(tree_sitter_php::language_php())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set PHP language: {}", e))
            })?;
        Ok(parser)
    }
}
