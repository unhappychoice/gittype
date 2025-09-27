use super::LanguageExtractor;
use crate::domain::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct HaskellExtractor;

impl LanguageExtractor for HaskellExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_haskell::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        r#"
            ; Function definitions
            (function) @function

            ; Type signatures
            (signature) @signature

            ; Data type declarations
            (data_type) @data
            (newtype) @newtype
            (data_family) @data_family
            (data_instance) @data_instance

            ; Data constructors
            (data_constructor) @constructor
            (gadt_constructor) @gadt_constructor
            (newtype_constructor) @newtype_constructor

            ; Type declarations
            (type_family) @type_family
            (type_instance) @type_instance
            (type_synomym) @type_synonym

            ; Module declarations
            (module) @module

            ; Import declarations
            (import) @import

            ; General declarations
            (declaration) @declaration
            (decl) @decl
        "#
    }

    fn comment_query(&self) -> &str {
        "[(comment)] @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function" => Some(ChunkType::Function),
            "signature" => Some(ChunkType::Function),
            "data" | "newtype" | "data_family" | "data_instance" => Some(ChunkType::Class),
            "constructor" | "gadt_constructor" | "newtype_constructor" => Some(ChunkType::Struct),
            "type_family" | "type_instance" | "type_synonym" => Some(ChunkType::TypeAlias),
            "module" => Some(ChunkType::Module),
            "import" => Some(ChunkType::Module),
            "declaration" | "decl" => Some(ChunkType::Function),
            _ => None,
        }
    }

    fn middle_implementation_query(&self) -> &str {
        "
        (apply) @function_call
        (case) @case_expr
        (let_in) @let_expr
        (conditional) @if_expr
        (do) @do_block
        (list_comprehension) @list_comp
        "
    }

    fn middle_capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "case_expr" | "if_expr" => Some(ChunkType::Conditional),
            "function_call" => Some(ChunkType::FunctionCall),
            "let_expr" => Some(ChunkType::Variable),
            "do_block" => Some(ChunkType::SpecialBlock),
            "list_comp" => Some(ChunkType::Comprehension),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "function" | "signature" => self.extract_function_name(node, source_code),
            "declaration" | "decl" => self.extract_declaration_name(node, source_code),
            "data" | "newtype" | "data_family" | "data_instance" => {
                self.extract_type_name(node, source_code)
            }
            "constructor" | "gadt_constructor" | "newtype_constructor" => {
                self.extract_constructor_name(node, source_code)
            }
            "type_family" | "type_instance" | "type_synonym" => {
                self.extract_type_name(node, source_code)
            }
            "module" => self.extract_module_name(node, source_code),
            "import" => self.extract_import_name(node, source_code),
            _ => extract_name_from_node(node, source_code),
        }
    }
}

impl HaskellExtractor {
    fn extract_function_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for variable node in function definition
        find_child_by_kind(node, source_code, "variable")
    }

    fn extract_type_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for type node in data/newtype declaration
        find_child_by_kind(node, source_code, "type")
    }

    fn extract_declaration_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for variable node in general declarations
        find_child_by_kind(node, source_code, "variable")
    }

    fn extract_constructor_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for constructor node
        find_child_by_kind(node, source_code, "constructor")
    }

    fn extract_module_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for module_name node
        find_child_by_kind(node, source_code, "module_name")
    }

    fn extract_import_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for module_name node in import
        find_child_by_kind(node, source_code, "module_name")
    }

    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_haskell::LANGUAGE.into())
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to set Haskell language: {}", e))
            })?;
        Ok(parser)
    }
}

fn find_child_by_kind(node: Node, source_code: &str, kind: &str) -> Option<String> {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            if child.kind() == kind {
                return child
                    .utf8_text(source_code.as_bytes())
                    .ok()
                    .map(|s| s.to_string());
            }
            // Recursively search in child nodes
            if let Some(name) = find_child_by_kind(child, source_code, kind) {
                return Some(name);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}

fn extract_name_from_node(node: Node, source_code: &str) -> Option<String> {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            match child.kind() {
                "variable" | "type" | "module_name" | "constructor" => {
                    return child
                        .utf8_text(source_code.as_bytes())
                        .ok()
                        .map(|s| s.to_string());
                }
                _ => {
                    // Recursively search in child nodes
                    if let Some(name) = extract_name_from_node(child, source_code) {
                        return Some(name);
                    }
                }
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}
