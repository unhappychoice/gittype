use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::java::JavaExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_java(source: &str) -> tree_sitter::Tree {
    let mut parser = JavaExtractor::create_parser().unwrap();
    parser.parse(source, None).unwrap()
}

fn find_node<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    if node.kind() == kind {
        return Some(node);
    }
    (0..node.child_count())
        .filter_map(|index| node.child(index))
        .find_map(|child| find_node(child, kind))
}

#[test]
fn create_parser_succeeds() {
    let result = JavaExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = JavaExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = JavaExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = JavaExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = JavaExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_class() {
    assert_eq!(
        JavaExtractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_interface() {
    assert_eq!(
        JavaExtractor.capture_name_to_chunk_type("interface"),
        Some(ChunkType::Interface)
    );
}

#[test]
fn capture_name_to_chunk_type_method() {
    assert_eq!(
        JavaExtractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Method)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    assert_eq!(
        JavaExtractor.capture_name_to_chunk_type("enum"),
        Some(ChunkType::Enum)
    );
}

#[test]
fn capture_name_to_chunk_type_field_returns_variable() {
    assert_eq!(
        JavaExtractor.capture_name_to_chunk_type("field"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_name_returns_code_block() {
    assert_eq!(
        JavaExtractor.capture_name_to_chunk_type("name"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(JavaExtractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_for_loop_returns_loop() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_enhanced_for_returns_loop() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("enhanced_for"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_while_loop_returns_loop() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_if_block_returns_conditional() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_try_block_returns_error_handling() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("try_block"),
        Some(ChunkType::ErrorHandling)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_switch_block_returns_conditional() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_method_call_returns_function_call() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("method_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_lambda_returns_lambda() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("lambda"),
        Some(ChunkType::Lambda)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_code_block_returns_code_block() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown_returns_none() {
    assert_eq!(
        JavaExtractor.middle_capture_name_to_chunk_type("unknown"),
        None
    );
}

#[test]
fn extract_name_for_class_declaration_returns_identifier() {
    let source = "class Foo {}\n";
    let tree = parse_java(source);
    let class =
        find_node(tree.root_node(), "class_declaration").expect("expected a class_declaration");

    let name = JavaExtractor.extract_name(class, source, "class");

    assert_eq!(name.as_deref(), Some("Foo"));
}

#[test]
fn extract_name_for_interface_declaration_returns_identifier() {
    let source = "interface Bar {}\n";
    let tree = parse_java(source);
    let iface = find_node(tree.root_node(), "interface_declaration")
        .expect("expected an interface_declaration");

    let name = JavaExtractor.extract_name(iface, source, "interface");

    assert_eq!(name.as_deref(), Some("Bar"));
}

#[test]
fn extract_name_for_method_declaration_returns_identifier() {
    let source = "class Foo { void greet() {} }\n";
    let tree = parse_java(source);
    let method =
        find_node(tree.root_node(), "method_declaration").expect("expected a method_declaration");

    let name = JavaExtractor.extract_name(method, source, "method");

    assert_eq!(name.as_deref(), Some("greet"));
}

#[test]
fn extract_name_for_enum_declaration_returns_identifier() {
    let source = "enum Color { RED, GREEN }\n";
    let tree = parse_java(source);
    let enum_node =
        find_node(tree.root_node(), "enum_declaration").expect("expected an enum_declaration");

    let name = JavaExtractor.extract_name(enum_node, source, "enum");

    assert_eq!(name.as_deref(), Some("Color"));
}

#[test]
fn extract_name_for_field_declaration_returns_variable_name() {
    let source = "class Foo { int counter = 0; }\n";
    let tree = parse_java(source);
    let field =
        find_node(tree.root_node(), "field_declaration").expect("expected a field_declaration");

    let name = JavaExtractor.extract_name(field, source, "field");

    assert_eq!(name.as_deref(), Some("counter"));
}

#[test]
fn extract_name_returns_none_for_leaf_node() {
    let source = "1\n";
    let tree = parse_java(source);

    fn first_leaf<'tree>(node: Node<'tree>) -> Node<'tree> {
        if node.child_count() == 0 {
            return node;
        }
        first_leaf(node.child(0).unwrap())
    }

    let leaf = first_leaf(tree.root_node());
    let name = JavaExtractor.extract_name(leaf, source, "method");

    assert_eq!(name, None);
}

#[test]
fn extract_name_for_field_capture_on_node_without_variable_declarator_returns_none() {
    let source = "class Foo { void greet() {} }\n";
    let tree = parse_java(source);
    let method =
        find_node(tree.root_node(), "method_declaration").expect("expected a method_declaration");

    let name = JavaExtractor.extract_name(method, source, "field");

    assert_eq!(name, None);
}
