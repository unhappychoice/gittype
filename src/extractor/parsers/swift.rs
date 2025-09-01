use super::LanguageExtractor;
use crate::extractor::models::{ChunkType, Language};
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct SwiftExtractor;

impl LanguageExtractor for SwiftExtractor {
    fn language(&self) -> Language {
        Language::Swift
    }

    fn file_extensions(&self) -> &[&str] {
        &["swift"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_swift::language()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_declaration name: (simple_identifier) @name) @function
            (class_declaration name: (type_identifier) @name) @class
            (protocol_declaration name: (type_identifier) @name) @protocol
            (_ 
              \"struct\" 
              name: (type_identifier) @name) @struct
            (_ 
              \"enum\" 
              name: (type_identifier) @name) @enum
        "
    }

    fn comment_query(&self) -> &str {
        "[(comment) (multiline_comment)] @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "class" => Some(ChunkType::Class),
            "struct" => Some(ChunkType::Struct),
            "enum" => Some(ChunkType::Enum),
            "protocol" => Some(ChunkType::Interface),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
        self.extract_name_from_node(node, source_code)
    }
}

impl SwiftExtractor {
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
            .set_language(tree_sitter_swift::language())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Swift language: {}", e))
            })?;
        Ok(parser)
    }
}
