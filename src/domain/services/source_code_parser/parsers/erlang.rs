use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct ErlangExtractor;

impl LanguageExtractor for ErlangExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_erlang::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        r#"
            (fun_decl
                (function_clause
                    name: (atom) @func_name
                )
            ) @function

            (module_attribute) @module_attr

            (export_attribute) @export_attr

            (record_decl) @record_decl

            (type_alias) @type_alias

            (spec) @spec_decl

            (behaviour_attribute) @behaviour_attr
        "#
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "module_attr" => Some(ChunkType::Module),
            "export_attr" => Some(ChunkType::CodeBlock),
            "record_decl" => Some(ChunkType::Struct),
            "type_alias" => Some(ChunkType::TypeAlias),
            "spec_decl" => Some(ChunkType::CodeBlock),
            "behaviour_attr" => Some(ChunkType::Interface),
            "func_name" => None,
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "function" => self.extract_function_name(node, source_code),
            "module_attr" => self.extract_atom_child(node, source_code),
            "record_decl" => self.extract_atom_child(node, source_code),
            "type_alias" => self.extract_atom_child(node, source_code),
            "spec_decl" => self.extract_atom_child(node, source_code),
            "behaviour_attr" => self.extract_atom_child(node, source_code),
            _ => node
                .utf8_text(source_code.as_bytes())
                .ok()
                .map(|s| s.to_string()),
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (case_expr) @case_expr
        (if_expr) @if_expr
        (receive_expr) @receive_expr
        (try_expr) @try_expr
        (anonymous_fun) @anonymous_fn
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "case_expr" | "if_expr" | "receive_expr" | "try_expr" => Some(ChunkType::Conditional),
            "anonymous_fn" => Some(ChunkType::Lambda),
            _ => None,
        }
    }
}

impl ErlangExtractor {
    fn extract_function_name(&self, node: Node, source_code: &str) -> Option<String> {
        node.child_by_field_name("name")
            .or_else(|| {
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.kind() == "function_clause" {
                            return child.child_by_field_name("name");
                        }
                        if child.kind() == "atom" {
                            return Some(child);
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
                None
            })
            .and_then(|n| n.utf8_text(source_code.as_bytes()).ok())
            .map(|s| s.to_string())
    }

    fn extract_atom_child(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "atom" {
                    return child
                        .utf8_text(source_code.as_bytes())
                        .ok()
                        .map(|s| s.to_string());
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
            .set_language(&tree_sitter_erlang::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Erlang language: {}", e))
            })?;
        Ok(parser)
    }
}
