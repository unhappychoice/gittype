use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct ElixirExtractor;

impl LanguageExtractor for ElixirExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_elixir::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        r#"
            (call
                target: (identifier) @def_keyword
                (arguments (identifier) @func_name)
                (#match? @def_keyword "^(def|defp)$")
            ) @function

            (call
                target: (identifier) @defmodule_keyword
                (arguments (alias) @module_name)
                (#match? @defmodule_keyword "^defmodule$")
            ) @module

            (call
                target: (identifier) @defmacro_keyword
                (arguments (identifier) @macro_name)
                (#match? @defmacro_keyword "^(defmacro|defmacrop)$")
            ) @macro_def

            (call
                target: (identifier) @defprotocol_keyword
                (arguments (alias) @protocol_name)
                (#match? @defprotocol_keyword "^defprotocol$")
            ) @protocol

            (call
                target: (identifier) @defimpl_keyword
                (#match? @defimpl_keyword "^defimpl$")
            ) @impl_def

            (call
                target: (identifier) @defstruct_keyword
                (#match? @defstruct_keyword "^defstruct$")
            ) @struct_def

            (call
                target: (identifier) @defguard_keyword
                (arguments (identifier) @guard_name)
                (#match? @defguard_keyword "^(defguard|defguardp)$")
            ) @guard_def
        "#
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "module" => Some(ChunkType::Module),
            "macro_def" => Some(ChunkType::Function),
            "protocol" => Some(ChunkType::Interface),
            "impl_def" => Some(ChunkType::Class),
            "struct_def" => Some(ChunkType::Struct),
            "guard_def" => Some(ChunkType::Function),
            "func_name" | "module_name" | "macro_name" | "protocol_name" | "guard_name" => {
                Some(ChunkType::CodeBlock)
            }
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
        self.extract_elixir_name(node, source_code)
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (call
            target: (identifier) @keyword
            (#match? @keyword \"^(if|unless|cond|case|with|for|try)$\")
        ) @control_flow

        (call
            target: (dot
                left: (_)
                right: (identifier)
            )
        ) @function_call

        (anonymous_function) @anonymous_fn
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "control_flow" => Some(ChunkType::Conditional),
            "function_call" => Some(ChunkType::FunctionCall),
            "anonymous_fn" => Some(ChunkType::Lambda),
            _ => None,
        }
    }
}

impl ElixirExtractor {
    fn extract_elixir_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                match child.kind() {
                    "arguments" => {
                        let mut args_cursor = child.walk();
                        if args_cursor.goto_first_child() {
                            loop {
                                let arg = args_cursor.node();
                                if arg.kind() == "identifier" || arg.kind() == "alias" {
                                    return arg
                                        .utf8_text(source_code.as_bytes())
                                        .ok()
                                        .map(|s| s.to_string());
                                }
                                if !args_cursor.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                    }
                    "identifier" | "alias" => {
                        return child
                            .utf8_text(source_code.as_bytes())
                            .ok()
                            .map(|s| s.to_string());
                    }
                    _ => {}
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
            .set_language(&tree_sitter_elixir::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Elixir language: {}", e))
            })?;
        Ok(parser)
    }
}
