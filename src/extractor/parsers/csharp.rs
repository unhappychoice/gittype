use super::LanguageExtractor;
use crate::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct CSharpExtractor;

impl LanguageExtractor for CSharpExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_c_sharp::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (class_declaration name: (identifier) @name) @class
            (struct_declaration name: (identifier) @name) @struct
            (interface_declaration name: (identifier) @name) @interface
            (enum_declaration name: (identifier) @name) @enum
            (record_declaration name: (identifier) @name) @class
            (method_declaration name: (identifier) @name) @method
            (constructor_declaration name: (identifier) @name) @method
            (destructor_declaration name: (identifier) @name) @method
            (property_declaration name: (identifier) @name) @property
            (event_declaration name: (identifier) @name) @event
            (delegate_declaration name: (identifier) @name) @delegate
            (namespace_declaration name: (_) @name) @namespace
        "
    }

    fn comment_query(&self) -> &str {
        "
            (comment) @comment
        "
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "class" => Some(ChunkType::Class),
            "struct" => Some(ChunkType::Struct),
            "interface" => Some(ChunkType::Interface),
            "enum" => Some(ChunkType::Enum),
            "method" => Some(ChunkType::Method),
            "property" => Some(ChunkType::Variable),
            "event" => Some(ChunkType::Variable),
            "field" => Some(ChunkType::Variable),
            "delegate" => Some(ChunkType::Method),
            "namespace" => Some(ChunkType::Namespace),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "field" => self.extract_field_name(node, source_code),
            "namespace" => self.extract_namespace_name(node, source_code),
            _ => self.extract_name_from_node(node, source_code),
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_statement) @for_loop
        (foreach_statement) @foreach_loop
        (while_statement) @while_loop
        (if_statement) @if_block
        (try_statement) @try_block
        (switch_statement) @switch_block
        (invocation_expression) @method_call
        (lambda_expression) @lambda
        (block) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "foreach_loop" | "while_loop" => Some(ChunkType::Loop),
            "if_block" | "switch_block" => Some(ChunkType::Conditional),
            "try_block" => Some(ChunkType::ErrorHandling),
            "method_call" => Some(ChunkType::FunctionCall),
            "lambda" => Some(ChunkType::Lambda),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl CSharpExtractor {
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
                if child.kind() == "variable_declaration" {
                    let mut var_cursor = child.walk();
                    if var_cursor.goto_first_child() {
                        loop {
                            let var_child = var_cursor.node();
                            if var_child.kind() == "variable_declarator" {
                                let mut decl_cursor = var_child.walk();
                                if decl_cursor.goto_first_child() {
                                    loop {
                                        let decl_child = decl_cursor.node();
                                        if decl_child.kind() == "identifier" {
                                            let start = decl_child.start_byte();
                                            let end = decl_child.end_byte();
                                            return Some(source_code[start..end].to_string());
                                        }
                                        if !decl_cursor.goto_next_sibling() {
                                            break;
                                        }
                                    }
                                }
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
        None
    }

    fn extract_namespace_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "qualified_name" || child.kind() == "identifier" {
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
            .set_language(&tree_sitter_c_sharp::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set C# language: {}", e))
            })?;
        Ok(parser)
    }
}
