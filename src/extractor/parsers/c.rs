use crate::extractor::parsers::LanguageExtractor;
use crate::models::ChunkType;
use crate::Result;
use tree_sitter::{Node, Parser};

pub struct CExtractor;

impl CExtractor {
    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        let language = tree_sitter_c::LANGUAGE;
        parser.set_language(&language.into())?;
        Ok(parser)
    }
}

impl LanguageExtractor for CExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_c::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        r#"
            ; Function definitions (prioritize full definitions)
            (function_definition
                declarator: (function_declarator
                    declarator: (identifier) @function.name)
                body: (compound_statement)) @function.definition

            ; Struct definitions with body
            (struct_specifier
                name: (type_identifier) @struct.name
                body: (field_declaration_list)) @struct.definition

            ; Type definitions  
            (type_definition
                declarator: (type_identifier) @type.name) @type.definition

            ; Enum definitions
            (enum_specifier
                name: (type_identifier) @enum.name) @enum.definition

            ; Global variable declarations (excluding function parameters)
            (declaration
                declarator: (init_declarator
                    declarator: (identifier) @variable.name)) @variable.definition

            ; Macro definitions
            (preproc_def
                name: (identifier) @macro.name) @macro.definition
        "#
    }

    fn comment_query(&self) -> &str {
        r#"
            (comment) @comment
        "#
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function.definition" => Some(ChunkType::Function),
            "struct.definition" => Some(ChunkType::Struct),
            "variable.definition" => Some(ChunkType::Variable),
            "type.definition" => Some(ChunkType::Struct),
            "enum.definition" => Some(ChunkType::Struct),
            "macro.definition" => Some(ChunkType::Function),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "function.definition" => {
                // For function definitions, find the identifier in the function_declarator
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    while cursor.node().kind() != "function_declarator" {
                        if !cursor.goto_next_sibling() {
                            return None;
                        }
                    }
                    let func_declarator = cursor.node();
                    let mut decl_cursor = func_declarator.walk();
                    if decl_cursor.goto_first_child() {
                        while decl_cursor.node().kind() != "identifier" {
                            if !decl_cursor.goto_next_sibling() {
                                return None;
                            }
                        }
                        let name_node = decl_cursor.node();
                        return name_node
                            .utf8_text(source_code.as_bytes())
                            .ok()
                            .map(|s| s.to_string());
                    }
                }
                None
            }
            "struct.definition" | "type.definition" | "enum.definition" => {
                // Find the type_identifier child
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        if cursor.node().kind() == "type_identifier" {
                            let text = cursor.node().utf8_text(source_code.as_bytes()).ok()?;
                            return Some(text.to_string());
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
                None
            }
            "variable.definition" => {
                // Find the identifier in init_declarator
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.kind() == "init_declarator" {
                            let mut init_cursor = child.walk();
                            if init_cursor.goto_first_child()
                                && init_cursor.node().kind() == "identifier"
                            {
                                let text =
                                    init_cursor.node().utf8_text(source_code.as_bytes()).ok()?;
                                return Some(text.to_string());
                            }
                        } else if child.kind() == "identifier" {
                            let text = child.utf8_text(source_code.as_bytes()).ok()?;
                            return Some(text.to_string());
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
                None
            }
            "macro.definition" => {
                // Find the identifier child
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        if cursor.node().kind() == "identifier" {
                            let text = cursor.node().utf8_text(source_code.as_bytes()).ok()?;
                            return Some(text.to_string());
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (for_statement) @for_loop
        (while_statement) @while_loop
        (if_statement) @if_block
        (switch_statement) @switch_block
        (call_expression) @function_call
        (compound_statement) @code_block
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "for_loop" | "while_loop" => Some(ChunkType::Loop),
            "if_block" | "switch_block" => Some(ChunkType::Conditional),
            "function_call" => Some(ChunkType::FunctionCall),
            "code_block" => Some(ChunkType::CodeBlock),
            _ => None,
        }
    }
}
