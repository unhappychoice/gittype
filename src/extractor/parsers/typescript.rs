use super::LanguageExtractor;
use crate::extractor::models::{ChunkType, Language};
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct TypeScriptExtractor;

impl LanguageExtractor for TypeScriptExtractor {
    fn language(&self) -> Language {
        Language::TypeScript
    }

    fn file_extensions(&self) -> &[&str] {
        &["ts", "tsx"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        // Use TSX parser which supports both TypeScript and JSX syntax
        tree_sitter_typescript::LANGUAGE_TSX.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_declaration name: (identifier) @name) @function
            (method_definition name: (property_identifier) @name) @method
            (class_declaration name: (type_identifier) @name) @class
            (variable_declarator name: (identifier) value: (arrow_function)) @arrow_function
            (variable_declarator name: (identifier) value: (function_expression)) @function_expression
            (interface_declaration name: (type_identifier) @name) @interface
            (type_alias_declaration name: (type_identifier) @name) @type_alias
            (enum_declaration name: (identifier) @name) @enum
            (internal_module name: (identifier) @name) @namespace
            (jsx_element open_tag: (jsx_opening_element name: (identifier) @name)) @jsx_element
            (jsx_self_closing_element name: (identifier) @name) @jsx_self_closing_element
        "
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "method" => Some(ChunkType::Method),
            "class" => Some(ChunkType::Class),
            "arrow_function" => Some(ChunkType::Function),
            "function_expression" => Some(ChunkType::Function),
            "interface" => Some(ChunkType::Interface),
            "type_alias" => Some(ChunkType::TypeAlias),
            "enum" => Some(ChunkType::Enum),
            "namespace" => Some(ChunkType::Module),
            "jsx_element" => Some(ChunkType::Component),
            "jsx_self_closing_element" => Some(ChunkType::Component),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "arrow_function" | "function_expression" => {
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
                None
            }
            "jsx_element" | "jsx_self_closing_element" => {
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.kind() == "identifier" || child.kind() == "jsx_identifier" {
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
            _ => self.extract_name_from_node(node, source_code),
        }
    }
}

impl TypeScriptExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier"
                    || child.kind() == "type_identifier"
                    || child.kind() == "property_identifier"
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

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!(
                    "Failed to set TypeScript/TSX language: {}",
                    e
                ))
            })?;
        Ok(parser)
    }
}
