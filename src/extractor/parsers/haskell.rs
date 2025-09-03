use super::LanguageExtractor;
use crate::models::ChunkType;
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct HaskellExtractor;

impl LanguageExtractor for HaskellExtractor {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_haskell::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        r#"
            ; Function definitions with name extraction
            (function name: (variable) @function.name) @function.definition
            (function (variable) @function.name) @function.named

            ; Type signatures with name extraction  
            (signature name: (variable) @signature.name) @signature.definition
            (signature (variable) @signature.name) @signature.named

            ; Module declarations
            (module) @module.definition

            ; Import declarations (if present)
            (import) @import.definition

            ; General declarations with names
            (declaration name: (variable) @declaration.name) @declaration.definition
            (decl name: (variable) @decl.name) @decl.definition

            ; Type elements
            (type name: (variable) @type.name) @type.definition

            ; Constructors
            (constructor) @constructor.definition

            ; Fallback patterns for completeness
            (function) @function.basic
            (signature) @signature.basic
            (declaration) @declaration.basic
        "#
    }

    fn comment_query(&self) -> &str {
        "[(comment)] @comment"
    }

    fn capture_name_to_chunk_type(&self, capture_name: &str) -> Option<ChunkType> {
        match capture_name {
            "function.definition" | "function.named" | "function.basic" => {
                Some(ChunkType::Function)
            }
            "signature.definition" | "signature.named" | "signature.basic" => {
                Some(ChunkType::Function)
            }
            "module.definition" => Some(ChunkType::Module),
            "import.definition" => Some(ChunkType::Module),
            "declaration.definition" | "declaration.basic" => Some(ChunkType::Function),
            "decl.definition" => Some(ChunkType::Function),
            "type.definition" => Some(ChunkType::TypeAlias),
            "constructor.definition" => Some(ChunkType::Struct),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            name if name.contains("function") => self.extract_function_name(node, source_code),
            name if name.contains("signature") => self.extract_signature_name(node, source_code),
            name if name.contains("declaration") || name.contains("decl") => {
                self.extract_declaration_name(node, source_code)
            }
            name if name.contains("type") => self.extract_type_name(node, source_code),
            name if name.contains("constructor") => {
                self.extract_constructor_name(node, source_code)
            }
            name if name.contains("module") => self.extract_module_name(node, source_code),
            name if name.contains("import") => self.extract_import_name(node, source_code),
            _ => extract_name_from_node(node, source_code),
        }
    }
}

impl HaskellExtractor {
    fn extract_function_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for variable node in function definition
        find_child_by_kind(node, source_code, "variable")
    }

    fn extract_signature_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for variable node in type signature
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
