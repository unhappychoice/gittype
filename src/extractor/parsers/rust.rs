use super::LanguageExtractor;
use crate::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct RustExtractor;

impl LanguageExtractor for RustExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_item name: (identifier) @name) @function
            (impl_item type: (type_identifier) @name) @impl
            (struct_item name: (type_identifier) @name) @struct
            (enum_item name: (type_identifier) @name) @enum
            (trait_item name: (type_identifier) @name) @trait
            (mod_item name: (identifier) @name) @module
            (type_item name: (type_identifier) @name) @type_alias
        "
    }

    fn comment_query(&self) -> &str {
        "[(line_comment) (block_comment)] @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "impl" => Some(ChunkType::Class),
            "struct" => Some(ChunkType::Struct),
            "enum" => Some(ChunkType::Enum),
            "trait" => Some(ChunkType::Trait),
            "type_alias" => Some(ChunkType::TypeAlias),
            "module" => Some(ChunkType::Module),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
        self.extract_name_from_node(node, source_code)
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_expression) @for_loop
        (while_expression) @while_loop
        (loop_expression) @loop
        (if_expression) @if_block
        (match_expression) @match_expr
        (closure_expression) @closure
        (call_expression) @function_call
        (macro_invocation) @macro_call
        (block) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "while_loop" | "loop" => Some(ChunkType::Loop),
            "if_block" => Some(ChunkType::Conditional),
            "match_expr" => Some(ChunkType::Conditional),
            "function_call" | "macro_call" => Some(ChunkType::FunctionCall),
            "closure" => Some(ChunkType::Lambda),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl RustExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" || child.kind() == "type_identifier" {
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
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Rust language: {}", e))
            })?;
        Ok(parser)
    }
}
