use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct JavaExtractor;

impl LanguageExtractor for JavaExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_java::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (class_declaration name: (identifier) @name) @class
            (interface_declaration name: (identifier) @name) @interface
            (method_declaration name: (identifier) @name) @method
            (constructor_declaration name: (identifier) @name) @method
            (enum_declaration name: (identifier) @name) @enum
            (record_declaration name: (identifier) @name) @class
            (annotation_type_declaration name: (identifier) @name) @interface
            (field_declaration declarator: (variable_declarator name: (identifier) @name)) @field
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
            "class" => Some(ChunkType::Class),
            "interface" => Some(ChunkType::Interface),
            "method" => Some(ChunkType::Method),
            "enum" => Some(ChunkType::Enum),
            "field" => Some(ChunkType::Variable),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "field" => self.extract_field_name(node, source_code),
            _ => self.extract_name_from_node(node, source_code),
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_statement) @for_loop
        (enhanced_for_statement) @enhanced_for
        (while_statement) @while_loop
        (if_statement) @if_block
        (try_statement) @try_block
        (switch_expression) @switch_block
        (method_invocation) @method_call
        (lambda_expression) @lambda
        (block) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "enhanced_for" | "while_loop" => Some(ChunkType::Loop),
            "if_block" => Some(ChunkType::Conditional),
            "try_block" => Some(ChunkType::ErrorHandling),
            "switch_block" => Some(ChunkType::Conditional),
            "method_call" => Some(ChunkType::FunctionCall),
            "lambda" => Some(ChunkType::Lambda),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl JavaExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
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

    fn extract_field_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "variable_declarator" {
                    let mut declarator_cursor = child.walk();
                    if declarator_cursor.goto_first_child() {
                        loop {
                            let declarator_child = declarator_cursor.node();
                            if declarator_child.kind() == "identifier" {
                                let start = declarator_child.start_byte();
                                let end = declarator_child.end_byte();
                                return Some(source_code[start..end].to_string());
                            }
                            if !declarator_cursor.goto_next_sibling() {
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
        None
    }

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_java::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Java language: {}", e))
            })?;
        Ok(parser)
    }
}
