use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct ClojureExtractor;

impl LanguageExtractor for ClojureExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_clojure::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (list_lit
                (sym_lit) @var_keyword (#match? @var_keyword \"^def$\")
                (sym_lit) @var_name
            ) @variable_def

            (list_lit
                (sym_lit) @ns_keyword (#match? @ns_keyword \"^ns$\")
                (sym_lit) @ns_name
            ) @namespace_def

            (list_lit
                (sym_lit) @func_keyword (#match? @func_keyword \"^(defn|defmacro|defn-)$\")
                (sym_lit) @func_name
            ) @function_def

            (list_lit
                (sym_lit) @class_keyword (#match? @class_keyword \"^(deftype|defrecord)$\")
                (sym_lit) @class_name
            ) @class_def

            (list_lit
                (sym_lit) @interface_keyword (#match? @interface_keyword \"^defprotocol$\")
                (sym_lit) @interface_name
            ) @interface_def
        "
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function_def" => Some(ChunkType::Function),
            "variable_def" => Some(ChunkType::Variable),
            "namespace_def" => Some(ChunkType::Namespace),
            "class_def" => Some(ChunkType::Class),
            "interface_def" => Some(ChunkType::Interface),
            "func_name" | "var_name" | "ns_name" | "class_name" | "interface_name" => {
                Some(ChunkType::CodeBlock)
            }
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, _capture_name: &str) -> Option<String> {
        self.extract_clojure_name(node, source_code)
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (list_lit (sym_lit) @keyword) @expr
        (#match? @keyword \"^(let|let\\*|loop|if|cond|do|try|for|doseq|when|when-not|if-not)$\")
        (list_lit
            (sym_lit) @call_keyword
        ) @function_call
        (#match? @call_keyword \"^[a-zA-Z].*\")
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "expr" => Some(ChunkType::Conditional),
            "function_call" => Some(ChunkType::FunctionCall),
            _ => None,
        }
    }
}

impl ClojureExtractor {
    fn extract_clojure_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        let mut sym_lit_count = 0;

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();

                if child.kind() == "sym_lit" {
                    sym_lit_count += 1;

                    // The second sym_lit is the function/macro/type name
                    if sym_lit_count == 2 {
                        let start = child.start_byte();
                        let end = child.end_byte();
                        return Some(source_code[start..end].to_string());
                    }
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
            .set_language(&tree_sitter_clojure::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Clojure language: {}", e))
            })?;
        Ok(parser)
    }
}
