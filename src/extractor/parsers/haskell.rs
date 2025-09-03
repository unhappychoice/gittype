use super::LanguageExtractor;
use crate::extractor::models::{ChunkType, Language};
use crate::{GitTypeError, Result};
use tree_sitter::{Node, Parser};

pub struct HaskellExtractor;

impl LanguageExtractor for HaskellExtractor {
    fn language(&self) -> Language {
        Language::Haskell
    }

    fn file_extensions(&self) -> &[&str] {
        &["hs", "lhs"]
    }

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
            _ => self.extract_name_from_node(node, source_code),
        }
    }
}

impl HaskellExtractor {
    fn extract_function_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for variable node in function definition
        self.find_child_by_kind(node, source_code, "variable")
    }

    fn extract_signature_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for variable node in type signature
        self.find_child_by_kind(node, source_code, "variable")
    }

    fn extract_type_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for type node in data/newtype declaration
        self.find_child_by_kind(node, source_code, "type")
    }

    fn extract_declaration_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for variable node in general declarations
        self.find_child_by_kind(node, source_code, "variable")
    }

    fn extract_constructor_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for constructor node
        self.find_child_by_kind(node, source_code, "constructor")
    }

    fn extract_module_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for module_name node
        self.find_child_by_kind(node, source_code, "module_name")
    }

    fn extract_import_name(&self, node: Node, source_code: &str) -> Option<String> {
        // Look for module_name node in import
        self.find_child_by_kind(node, source_code, "module_name")
    }

    fn find_child_by_kind(&self, node: Node, source_code: &str, kind: &str) -> Option<String> {
        find_child_by_kind_impl(node, source_code, kind)
    }

    fn extract_name_from_node(&self, node: Node, source_code: &str) -> Option<String> {
        extract_name_from_node_impl(node, source_code)
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

fn find_child_by_kind_impl(node: Node, source_code: &str, kind: &str) -> Option<String> {
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
            if let Some(name) = find_child_by_kind_impl(child, source_code, kind) {
                return Some(name);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    None
}

fn extract_name_from_node_impl(node: Node, source_code: &str) -> Option<String> {
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
                    if let Some(name) = extract_name_from_node_impl(child, source_code) {
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
