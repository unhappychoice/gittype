use crate::extractor::models::{ChunkType, Language};
use crate::extractor::parsers::LanguageExtractor;
use crate::Result;
use tree_sitter::{Node, Parser};

pub struct CppExtractor;

impl CppExtractor {
    pub fn create_parser() -> Result<Parser> {
        let mut parser = Parser::new();
        let language = tree_sitter_cpp::LANGUAGE;
        parser.set_language(&language.into())?;
        Ok(parser)
    }
}

impl LanguageExtractor for CppExtractor {
    fn language(&self) -> Language {
        Language::Cpp
    }

    fn file_extensions(&self) -> &[&str] {
        &["cpp", "cc", "cxx", "hpp", "h"]
    }

    fn tree_sitter_language(&self) -> tree_sitter::Language {
        tree_sitter_cpp::LANGUAGE.into()
    }

    fn query_patterns(&self) -> &str {
        r#"
            ; Function definitions (global functions)
            (function_definition
                declarator: (function_declarator
                    declarator: (identifier) @function.name)
                body: (compound_statement)) @function.definition

            ; Method definitions in class (using field_identifier)
            (function_definition
                declarator: (function_declarator
                    declarator: (field_identifier) @method.name)
                body: (compound_statement)) @method.definition

            ; Class definitions
            (class_specifier
                name: (type_identifier) @class.name
                body: (field_declaration_list)) @class.definition

            ; Struct definitions
            (struct_specifier
                name: (type_identifier) @struct.name
                body: (field_declaration_list)) @struct.definition

            ; Namespace definitions (commented out - may not be supported by tree-sitter-cpp)
            ; (namespace_definition
            ;     name: (identifier) @namespace.name) @namespace.definition

            ; Template class declarations
            (template_declaration
                (class_specifier
                    name: (type_identifier) @template_class.name)) @template_class.definition

            ; Template function declarations
            (template_declaration
                (function_definition
                    declarator: (function_declarator
                        declarator: (identifier) @template_function.name))) @template_function.definition

            ; Type definitions
            (type_definition
                declarator: (type_identifier) @type.name) @type.definition

            ; Enum definitions
            (enum_specifier
                name: (type_identifier) @enum.name) @enum.definition

            ; Global variable declarations
            (declaration
                declarator: (init_declarator
                    declarator: (identifier) @variable.name)) @variable.definition
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
            "method.definition" => Some(ChunkType::Function),
            "class.definition" => Some(ChunkType::Struct),
            "struct.definition" => Some(ChunkType::Struct),
            "namespace.definition" => Some(ChunkType::Struct),
            "template_class.definition" => Some(ChunkType::Struct),
            "template_function.definition" => Some(ChunkType::Function),
            "type.definition" => Some(ChunkType::Struct),
            "enum.definition" => Some(ChunkType::Struct),
            "variable.definition" => Some(ChunkType::Variable),
            _ => None,
        }
    }

    fn extract_name(&self, node: Node, source_code: &str, capture_name: &str) -> Option<String> {
        match capture_name {
            "function.definition" | "template_function.definition" => {
                self.extract_function_name(node, source_code)
            }
            "method.definition" => self.extract_function_name(node, source_code),
            "class.definition" | "struct.definition" | "template_class.definition" => {
                self.extract_type_name(node, source_code)
            }
            "namespace.definition" => self.extract_namespace_name(node, source_code),
            "constructor.definition" | "destructor.definition" => {
                self.extract_constructor_destructor_name(node, source_code)
            }
            "operator.definition" => self.extract_operator_name(node, source_code),
            "type.definition" | "enum.definition" => self.extract_type_name(node, source_code),
            "variable.definition" => self.extract_variable_name(node, source_code),
            _ => None,
        }
    }
}

impl CppExtractor {
    fn extract_function_name(&self, node: Node, source_code: &str) -> Option<String> {
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
                // Look for either identifier or field_identifier
                while decl_cursor.node().kind() != "identifier"
                    && decl_cursor.node().kind() != "field_identifier"
                {
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

    fn extract_type_name(&self, node: Node, source_code: &str) -> Option<String> {
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

    fn extract_namespace_name(&self, node: Node, source_code: &str) -> Option<String> {
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

    fn extract_constructor_destructor_name(&self, node: Node, source_code: &str) -> Option<String> {
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
                if decl_cursor.node().kind() == "identifier" {
                    let name_node = decl_cursor.node();
                    return name_node
                        .utf8_text(source_code.as_bytes())
                        .ok()
                        .map(|s| s.to_string());
                } else if decl_cursor.node().kind() == "destructor_name" {
                    let destructor = decl_cursor.node();
                    let mut dest_cursor = destructor.walk();
                    if dest_cursor.goto_first_child() && dest_cursor.node().kind() == "identifier" {
                        let name_node = dest_cursor.node();
                        let name = name_node
                            .utf8_text(source_code.as_bytes())
                            .ok()
                            .map(|s| format!("~{}", s))?;
                        return Some(name);
                    }
                }
            }
        }
        None
    }

    fn extract_operator_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            while cursor.node().kind() != "function_declarator" {
                if !cursor.goto_next_sibling() {
                    return None;
                }
            }
            let func_declarator = cursor.node();
            let mut decl_cursor = func_declarator.walk();
            if decl_cursor.goto_first_child() && decl_cursor.node().kind() == "operator_name" {
                let operator_node = decl_cursor.node();
                return operator_node
                    .utf8_text(source_code.as_bytes())
                    .ok()
                    .map(|s| s.to_string());
            }
        }
        None
    }

    fn extract_variable_name(&self, node: Node, source_code: &str) -> Option<String> {
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "init_declarator" {
                    let mut init_cursor = child.walk();
                    if init_cursor.goto_first_child() && init_cursor.node().kind() == "identifier" {
                        let text = init_cursor.node().utf8_text(source_code.as_bytes()).ok()?;
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
}
