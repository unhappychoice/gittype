use super::LanguageExtractor;
use crate::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct GoExtractor;

impl LanguageExtractor for GoExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_go::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (function_declaration name: (identifier) @name) @function
            (method_declaration receiver: _ name: (field_identifier) @name) @method
            (type_spec name: (type_identifier) @name type: (struct_type)) @struct
            (type_spec name: (type_identifier) @name type: (interface_type)) @interface
            (const_declaration) @const_block
            (var_declaration) @var_block
            (type_spec name: (type_identifier) @name type: (type_identifier)) @type_alias
            (type_spec name: (type_identifier) @name type: (function_type)) @type_alias
            (type_spec name: (type_identifier) @name type: (pointer_type)) @type_alias
            (type_spec name: (type_identifier) @name type: (slice_type)) @type_alias
            (type_spec name: (type_identifier) @name type: (array_type)) @type_alias
            (type_spec name: (type_identifier) @name type: (map_type)) @type_alias
            (type_spec name: (type_identifier) @name type: (channel_type)) @type_alias
        "
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "method" => Some(ChunkType::Method),
            "struct" => Some(ChunkType::Struct),
            "interface" => Some(ChunkType::Interface),
            "const_block" => Some(ChunkType::Const),
            "var_block" => Some(ChunkType::Variable),
            "type_alias" => Some(ChunkType::TypeAlias),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "const_block" => Some(self.extract_const_var_names(node, source_code, "const")),
            "var_block" => Some(self.extract_const_var_names(node, source_code, "var")),
            _ => self.extract_name_from_node(node, source_code),
        }
    }
}

impl GoExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier"
                    || child.kind() == "type_identifier"
                    || child.kind() == "field_identifier"
                {
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

    fn extract_const_var_names(
        &self,
        node: Node,
        source_code: &str,
        declaration_type: &str,
    ) -> String {
        let mut names = Vec::new();
        let mut cursor = node.walk();

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "const_spec" || child.kind() == "var_spec" {
                    let mut spec_cursor = child.walk();
                    if spec_cursor.goto_first_child() {
                        loop {
                            let spec_child = spec_cursor.node();
                            if spec_child.kind() == "identifier" {
                                let start = spec_child.start_byte();
                                let end = spec_child.end_byte();
                                names.push(source_code[start..end].to_string());
                                break;
                            }
                            if !spec_cursor.goto_next_sibling() {
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

        if names.is_empty() {
            format!("{}_block", declaration_type)
        } else if names.len() == 1 {
            names[0].clone()
        } else {
            format!("{} ({})", names.join(", "), names.len())
        }
    }

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_go::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Go language: {}", e))
            })?;
        Ok(parser)
    }
}
