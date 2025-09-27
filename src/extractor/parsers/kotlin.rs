use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct KotlinExtractor;

impl LanguageExtractor for KotlinExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_kotlin_ng::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_declaration) @function
            (anonymous_function) @function
            (class_declaration) @class
            (object_declaration) @object
            (companion_object) @companion
            (property_declaration) @property
            (enum_entry) @enum_entry
            (type_alias) @type_alias
        "
    }

    fn comment_query(&self) -> &str {
        "
            (line_comment) @comment
            (block_comment) @comment
        "
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "class" => Some(ChunkType::Class),
            "interface" => Some(ChunkType::Class),
            "object" => Some(ChunkType::Class),
            "companion" => Some(ChunkType::Class),
            "property" => Some(ChunkType::Variable),
            "enum_entry" => Some(ChunkType::Const),
            "type_alias" => Some(ChunkType::Class),
            "lambda" => Some(ChunkType::Function),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "property" => self.extract_property_name(node, source_code),
            "companion" => Some("companion object".to_string()),
            _ => self.extract_name_from_node(node, source_code),
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_statement) @for_loop
        (while_statement) @while_loop
        (if_expression) @if_block
        (try_expression) @try_block
        (when_expression) @when_block
        (call_expression) @function_call
        (lambda_literal) @lambda
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "while_loop" => Some(ChunkType::Loop),
            "if_block" | "when_block" => Some(ChunkType::Conditional),
            "try_block" => Some(ChunkType::ErrorHandling),
            "function_call" => Some(ChunkType::FunctionCall),
            "lambda" => Some(ChunkType::Lambda),
            _ => None,
        }
    }
}

impl KotlinExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "simple_identifier" || child.kind() == "type_identifier" {
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

    fn extract_property_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "variable_declaration" {
                    // Look for simple_identifier in variable_declaration
                    let mut var_cursor = child.walk();
                    if var_cursor.goto_first_child() {
                        loop {
                            let var_child = var_cursor.node();
                            if var_child.kind() == "simple_identifier" {
                                let start = var_child.start_byte();
                                let end = var_child.end_byte();
                                return Some(source_code[start..end].to_string());
                            }
                            if !var_cursor.goto_next_sibling() {
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
        // Fall back to general name extraction
        self.extract_name_from_node(node, source_code)
    }

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_kotlin_ng::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Kotlin language: {}", e))
            })?;
        Ok(parser)
    }
}
