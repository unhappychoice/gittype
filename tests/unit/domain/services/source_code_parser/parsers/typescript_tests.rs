use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::typescript::TypeScriptExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_typescript(source: &str) -> tree_sitter::Tree {
    let mut parser = TypeScriptExtractor::create_parser().unwrap();
    parser.parse(source, None).unwrap()
}

fn find_first_node<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    if node.kind() == kind {
        return Some(node);
    }

    (0..node.named_child_count()).find_map(|index| {
        node.named_child(index)
            .and_then(|child| find_first_node(child, kind))
    })
}

#[test]
fn capture_name_to_chunk_type_maps_supported_captures() {
    let extractor = TypeScriptExtractor;
    [
        ("function", ChunkType::Function),
        ("method", ChunkType::Method),
        ("class", ChunkType::Class),
        ("arrow_function", ChunkType::Function),
        ("function_expression", ChunkType::Function),
        ("interface", ChunkType::Interface),
        ("type_alias", ChunkType::TypeAlias),
        ("enum", ChunkType::Enum),
        ("namespace", ChunkType::Module),
        ("jsx_element", ChunkType::Component),
        ("jsx_self_closing_element", ChunkType::Component),
        ("name", ChunkType::CodeBlock),
    ]
    .into_iter()
    .for_each(|(capture_name, chunk_type)| {
        assert_eq!(
            extractor.capture_name_to_chunk_type(capture_name),
            Some(chunk_type)
        );
    });
}

#[test]
fn capture_name_to_chunk_type_returns_none_for_unknown_capture() {
    let extractor = TypeScriptExtractor;

    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn create_parser_succeeds() {
    let result = TypeScriptExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = TypeScriptExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = TypeScriptExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = TypeScriptExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = TypeScriptExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_capture_name_to_chunk_type_returns_none_for_unknown_capture() {
    let extractor = TypeScriptExtractor;

    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn extract_name_reads_arrow_function_variable_name() {
    let extractor = TypeScriptExtractor;
    let source = "const greet = () => 1;\n";
    let tree = parse_typescript(source);
    let declarator = find_first_node(tree.root_node(), "variable_declarator").unwrap();

    assert_eq!(
        extractor.extract_name(declarator, source, "arrow_function"),
        Some("greet".to_string())
    );
}

#[test]
fn extract_name_returns_none_for_destructured_arrow_function_name() {
    let extractor = TypeScriptExtractor;
    let source = "const { greet } = () => 1;\n";
    let tree = parse_typescript(source);
    let declarator = find_first_node(tree.root_node(), "variable_declarator").unwrap();

    assert_eq!(
        extractor.extract_name(declarator, source, "arrow_function"),
        None
    );
}

#[test]
fn extract_name_reads_jsx_self_closing_element_name() {
    let extractor = TypeScriptExtractor;
    let source = "const view = <Widget />;\n";
    let tree = parse_typescript(source);
    let element = find_first_node(tree.root_node(), "jsx_self_closing_element").unwrap();

    assert_eq!(
        extractor.extract_name(element, source, "jsx_self_closing_element"),
        Some("Widget".to_string())
    );
}

#[test]
fn extract_name_reads_class_type_identifier() {
    let extractor = TypeScriptExtractor;
    let source = "class User {}\n";
    let tree = parse_typescript(source);
    let class_node = find_first_node(tree.root_node(), "class_declaration").unwrap();

    assert_eq!(
        extractor.extract_name(class_node, source, "class"),
        Some("User".to_string())
    );
}

#[test]
fn extract_name_returns_none_when_expected_node_shape_is_missing() {
    let extractor = TypeScriptExtractor;
    let source = "class User {}\n";
    let tree = parse_typescript(source);
    let class_node = find_first_node(tree.root_node(), "class_declaration").unwrap();

    assert_eq!(
        extractor.extract_name(class_node, source, "arrow_function"),
        None
    );
}
