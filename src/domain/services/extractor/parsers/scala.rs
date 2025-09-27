use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct ScalaExtractor;

impl LanguageExtractor for ScalaExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_scala::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_definition) @function
            (class_definition) @class
            (object_definition) @object
            (trait_definition) @trait
            (enum_definition) @enum
            (type_definition) @type
            (package_object) @package_object
            (given_definition) @given
            (extension_definition) @extension
        "
    }

    fn comment_query(&self) -> &str {
        "
            (comment) @comment
            (block_comment) @comment
        "
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "class" => Some(ChunkType::Class),
            "object" => Some(ChunkType::Class),
            "trait" => Some(ChunkType::Class),
            "enum" => Some(ChunkType::Const),
            "type" => Some(ChunkType::Class),
            "package_object" => Some(ChunkType::Class),
            "given" => Some(ChunkType::Function),
            "extension" => Some(ChunkType::Function),
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
        (if_expression) @if_block
        (try_expression) @try_block
        (match_expression) @match_block
        (call_expression) @function_call
        (lambda_expression) @lambda
        (block) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "while_loop" => Some(ChunkType::Loop),
            "if_block" | "match_block" => Some(ChunkType::Conditional),
            "try_block" => Some(ChunkType::ErrorHandling),
            "function_call" => Some(ChunkType::FunctionCall),
            "lambda" => Some(ChunkType::Lambda),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl ScalaExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for a named field called "name" first
        if let Some(name_node) = node.child_by_field_name("name") {
            let start = name_node.start_byte();
            let end = name_node.end_byte();
            return Some(source_code[start..end].to_string());
        }

        // Fallback: look for the first identifier node
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
            .set_language(&tree_sitter_scala::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Scala language: {}", e))
            })?;
        Ok(parser)
    }
}
