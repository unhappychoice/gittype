use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct ZigExtractor;

impl LanguageExtractor for ZigExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_zig::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_declaration name: (identifier) @name) @function
            (variable_declaration (identifier) @name \"=\" (struct_declaration)) @struct
            (variable_declaration (identifier) @name \"=\" (enum_declaration)) @enum
            (variable_declaration (identifier) @name \"=\" (union_declaration)) @union
        "
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "struct" => Some(ChunkType::Struct),
            "enum" => Some(ChunkType::Enum),
            "union" => Some(ChunkType::Struct),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
        self.extract_name_from_node(node, source_code)
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_statement) @for_loop
        (while_statement) @while_loop
        (if_statement) @if_block
        (switch_expression) @switch_expr
        (block) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "while_loop" => Some(ChunkType::Loop),
            "if_block" => Some(ChunkType::Conditional),
            "switch_expr" => Some(ChunkType::Conditional),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl ZigExtractor {
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

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_zig::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Zig language: {}", e))
            })?;
        Ok(parser)
    }
}
