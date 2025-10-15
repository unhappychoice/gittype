use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct JavaScriptExtractor;

impl LanguageExtractor for JavaScriptExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_javascript::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_declaration name: (identifier) @name) @function
            (generator_function_declaration name: (identifier) @name) @generator_function
            (method_definition name: (property_identifier) @name) @method
            (assignment_expression left: (member_expression) right: (function_expression)) @method
            (assignment_expression left: (member_expression) right: (arrow_function)) @method
            (class_declaration name: (identifier) @name) @class
            (variable_declarator name: (identifier) value: (arrow_function)) @arrow_function
            (variable_declarator name: (identifier) value: (function_expression)) @function_expression
            (variable_declarator name: (identifier) value: (generator_function)) @generator_function_expression
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
            "generator_function" => Some(ChunkType::Function),
            "generator_function_expression" => Some(ChunkType::Function),
            "method" => Some(ChunkType::Method),
            "class" => Some(ChunkType::Class),
            "arrow_function" => Some(ChunkType::Function),
            "function_expression" => Some(ChunkType::Function),
            "jsx_element" => Some(ChunkType::Component),
            "jsx_self_closing_element" => Some(ChunkType::Component),
            "variable" => Some(ChunkType::Variable),
            "name" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "arrow_function" | "function_expression" | "generator_function_expression" => {
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
            "method" => {
                if node.kind() == "assignment_expression" {
                    let mut cursor = node.walk();
                    if cursor.goto_first_child() && cursor.node().kind() == "member_expression" {
                        let member_cursor = cursor.node().walk();
                        let mut cursor = member_cursor;
                        while cursor.goto_next_sibling() {
                            let child = cursor.node();
                            if child.kind() == "property_identifier" {
                                let start = child.start_byte();
                                let end = child.end_byte();
                                return Some(source_code[start..end].to_string());
                            }
                        }
                    }
                }
                self.extract_name_from_node(node, source_code)
            }
            _ => self.extract_name_from_node(node, source_code),
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_statement) @for_loop
        (for_in_statement) @for_in_loop
        (while_statement) @while_loop
        (do_statement) @do_while_loop
        (if_statement) @if_block
        (try_statement) @try_block
        (switch_statement) @switch_block
        (function_expression) @function_expr
        (arrow_function) @arrow_lambda
        (call_expression) @function_call
        (statement_block) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "for_in_loop" | "while_loop" | "do_while_loop" => Some(ChunkType::Loop),
            "if_block" => Some(ChunkType::Conditional),
            "try_block" => Some(ChunkType::ErrorHandling),
            "switch_block" => Some(ChunkType::Conditional),
            "function_expr" | "arrow_lambda" => Some(ChunkType::Lambda),
            "function_call" => Some(ChunkType::FunctionCall),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl JavaScriptExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" || child.kind() == "property_identifier" {
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
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set JavaScript language: {}", e))
            })?;
        Ok(parser)
    }
}
