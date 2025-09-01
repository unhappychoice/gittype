use super::LanguageExtractor;
use crate::extractor::models::{ChunkType, Language};
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct KotlinExtractor;

impl LanguageExtractor for KotlinExtractor {
    fn language(&self) -> Language {
        Language::Kotlin
    }

    fn file_extensions(&self) -> &[&str] {
        &["kt", "kts"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_kotlin::language()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_declaration (simple_identifier) @name) @function
            (class_declaration (type_identifier) @name) @class
            (object_declaration (type_identifier) @name) @object
            (property_declaration (variable_declaration (simple_identifier) @name)) @property
            (companion_object) @companion
            (enum_entry (simple_identifier) @name) @enum_entry
        "
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "class" => Some(ChunkType::Class),
            "object" => Some(ChunkType::Class),
            "property" => Some(ChunkType::Variable),
            "companion" => Some(ChunkType::Class),
            "enum_entry" => Some(ChunkType::Const),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
        self.extract_name_from_node(node, source_code)
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

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_kotlin::language())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Kotlin language: {}", e))
            })?;
        Ok(parser)
    }
}
