use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct RubyExtractor;

impl LanguageExtractor for RubyExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_ruby::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        "
            (method name: (identifier) @name) @method
            (singleton_method object: (self) name: (identifier) @name) @class_method
            (singleton_method name: (identifier) @name) @singleton_method
            (class name: (constant) @name) @class
            (module name: (constant) @name) @module
            (call method: (identifier) @method_name (#match? @method_name \"^(attr_accessor|attr_reader|attr_writer)$\") arguments: (argument_list)) @attr_accessor
        "
    }

    fn comment_query(&self) -> &str {
        "(comment) @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "method" => Some(ChunkType::Method),
            "class_method" => Some(ChunkType::Method),
            "singleton_method" => Some(ChunkType::Method),
            "attr_accessor" => Some(ChunkType::Method),
            "class" => Some(ChunkType::Class),
            "module" => Some(ChunkType::Module),
            "name" => Some(ChunkType::CodeBlock),            
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "attr_accessor" => self.extract_attr_accessor_name(node, source_code),
            _ => self.extract_name_from_node(node, source_code),
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for) @for_loop
        (while) @while_loop
        (until) @until_loop
        (if) @if_block
        (unless) @unless_block
        (case) @case_block
        (begin) @begin_block
        (rescue) @rescue_block
        (call) @method_call
        (lambda) @lambda
        (block) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "while_loop" | "until_loop" => Some(ChunkType::Loop),
            "if_block" | "unless_block" | "case_block" => Some(ChunkType::Conditional),
            "begin_block" | "rescue_block" => Some(ChunkType::ErrorHandling),
            "method_call" => Some(ChunkType::FunctionCall),
            "lambda" => Some(ChunkType::Lambda),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}

impl RubyExtractor {
    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" || child.kind() == "constant" {
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

    fn extract_attr_accessor_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        let mut symbols = Vec::new();

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
                    let start = child.start_byte();
                    let end = child.end_byte();
                    let method_name = &source_code[start..end];

                    if method_name == "attr_accessor"
                        || method_name == "attr_reader"
                        || method_name == "attr_writer"
                    {
                        if cursor.goto_next_sibling() {
                            let args_node = cursor.node();
                            if args_node.kind() == "argument_list" {
                                let mut args_cursor = args_node.walk();
                                if args_cursor.goto_first_child() {
                                    loop {
                                        let arg = args_cursor.node();
                                        if arg.kind() == "simple_symbol" {
                                            let start = arg.start_byte();
                                            let end = arg.end_byte();
                                            let symbol = &source_code[start..end];
                                            symbols
                                                .push(symbol.trim_start_matches(':').to_string());
                                        }
                                        if !args_cursor.goto_next_sibling() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        if symbols.is_empty() {
            Some("unknown_attr".to_string())
        } else {
            Some(format!("{} ({})", symbols.join(", "), symbols.len()))
        }
    }

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_ruby::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Ruby language: {}", e))
            })?;
        Ok(parser)
    }
}
