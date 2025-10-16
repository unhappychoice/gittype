use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct PythonExtractor;

impl LanguageExtractor for PythonExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_python::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_definition name: (identifier) @name) @function
            (class_definition name: (identifier) @name) @class
        "
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "class" => Some(ChunkType::Class),
            "name" => Some(ChunkType::CodeBlock),
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
        (try_statement) @try_block
        (with_statement) @with_block
        (function_definition) @nested_function
        (class_definition) @nested_class
        (list_comprehension) @list_comp
        (dictionary_comprehension) @dict_comp
        (call) @function_call
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "while_loop" => Some(ChunkType::Loop),
            "if_block" => Some(ChunkType::Conditional),
            "try_block" => Some(ChunkType::ErrorHandling),
            "with_block" => Some(ChunkType::SpecialBlock),
            "nested_function" => Some(ChunkType::Function),
            "nested_class" => Some(ChunkType::Class),
            "list_comp" | "dict_comp" => Some(ChunkType::Comprehension),
            "function_call" => Some(ChunkType::FunctionCall),
            _ => None,
        }
    }
}

impl PythonExtractor {
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
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Python language: {}", e))
            })?;
        Ok(parser)
    }
}
