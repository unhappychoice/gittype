use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::csharp::CSharpExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_csharp(source: &str) -> tree_sitter::Tree {
    let mut parser = CSharpExtractor::create_parser().unwrap();
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
    let result = CSharpExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = CSharpExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = CSharpExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = CSharpExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = CSharpExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_event() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("event"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_delegate() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("delegate"),
        Some(ChunkType::Method)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_struct() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("struct"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_interface() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("interface"),
        Some(ChunkType::Interface)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("enum"),
        Some(ChunkType::Enum)
    );
}

#[test]
fn capture_name_to_chunk_type_method() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Method)
    );
}

#[test]
fn capture_name_to_chunk_type_property() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("property"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_field() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("field"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_namespace() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("namespace"),
        Some(ChunkType::Namespace)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown() {
    let extractor = CSharpExtractor;
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_lambda() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("lambda"),
        Some(ChunkType::Lambda)
    );
}

#[test]
fn middle_capture_name_loop_variants() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("foreach_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_conditional() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_error_handling() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("try_block"),
        Some(ChunkType::ErrorHandling)
    );
}

#[test]
fn middle_capture_name_method_call() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("method_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_code_block() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_unknown() {
    let extractor = CSharpExtractor;
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn extract_name_for_class_returns_identifier() {
    let source = "class Foo {}\n";
    let tree = parse_csharp(source);
    let class_node = find_node(tree.root_node(), "class_declaration")
        .expect("expected a class_declaration node");

    let name = CSharpExtractor.extract_name(class_node, source, "class");

    assert_eq!(name.as_deref(), Some("Foo"));
}

#[test]
fn extract_name_for_method_returns_identifier() {
    let source = "class Foo { void Greet() {} }\n";
    let tree = parse_csharp(source);
    let method_node = find_node(tree.root_node(), "method_declaration")
        .expect("expected a method_declaration node");

    let name = CSharpExtractor.extract_name(method_node, source, "method");

    assert_eq!(name.as_deref(), Some("Greet"));
}

#[test]
fn extract_name_for_field_returns_identifier() {
    let source = "class Foo { int counter; }\n";
    let tree = parse_csharp(source);
    let field_node = find_node(tree.root_node(), "field_declaration")
        .expect("expected a field_declaration node");

    let name = CSharpExtractor.extract_name(field_node, source, "field");

    assert_eq!(name.as_deref(), Some("counter"));
}

#[test]
fn extract_name_for_field_with_initializer_returns_identifier() {
    let source = "class Foo { string label = \"hi\"; }\n";
    let tree = parse_csharp(source);
    let field_node = find_node(tree.root_node(), "field_declaration")
        .expect("expected a field_declaration node");

    let name = CSharpExtractor.extract_name(field_node, source, "field");

    assert_eq!(name.as_deref(), Some("label"));
}

#[test]
fn extract_name_for_namespace_with_simple_identifier_returns_name() {
    let source = "namespace Foo { class Bar {} }\n";
    let tree = parse_csharp(source);
    let ns_node = find_node(tree.root_node(), "namespace_declaration")
        .expect("expected a namespace_declaration node");

    let name = CSharpExtractor.extract_name(ns_node, source, "namespace");

    assert_eq!(name.as_deref(), Some("Foo"));
}

#[test]
fn extract_name_for_namespace_with_qualified_name_returns_full_path() {
    let source = "namespace Foo.Bar.Baz { class C {} }\n";
    let tree = parse_csharp(source);
    let ns_node = find_node(tree.root_node(), "namespace_declaration")
        .expect("expected a namespace_declaration node");

    let name = CSharpExtractor.extract_name(ns_node, source, "namespace");

    assert_eq!(name.as_deref(), Some("Foo.Bar.Baz"));
}

#[test]
fn extract_name_for_unknown_capture_returns_none() {
    let source = "class Foo {}\n";
    let tree = parse_csharp(source);

    let name = CSharpExtractor.extract_name(tree.root_node(), source, "unknown.capture");

    // For unknown capture, falls back to extract_name_from_node which searches
    // for an `identifier` child of root — root has no direct identifier child.
    assert_eq!(name, None);
}
