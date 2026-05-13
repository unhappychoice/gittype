use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::cpp::CppExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_cpp(source_code: &str) -> tree_sitter::Tree {
    let mut parser = CppExtractor::create_parser().unwrap();
    parser.parse(source_code, None).unwrap()
}

fn find_node<'tree>(
    node: Node<'tree>,
    source_code: &str,
    kind: &str,
    text: &str,
) -> Option<Node<'tree>> {
    let matches_kind = node.kind() == kind;
    let matches_text = node
        .utf8_text(source_code.as_bytes())
        .map(|node_text| node_text.contains(text))
        .unwrap_or(false);

    matches_kind
        .then_some(node)
        .filter(|_| matches_text)
        .or_else(|| {
            (0..node.child_count())
                .filter_map(|index| node.child(index))
                .find_map(|child| find_node(child, source_code, kind, text))
        })
}

#[test]
fn create_parser_succeeds() {
    let result = CppExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = CppExtractor;
    let _language = extractor.tree_sitter_language();
    // Just verify it doesn't panic
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = CppExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
    assert!(patterns.contains("function"));
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = CppExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
    assert!(query.contains("comment"));
}

#[test]
fn capture_name_to_chunk_type_function() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("function.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_method() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("method.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("class.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_struct() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("struct.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_variable() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("variable.definition"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown() {
    let extractor = CppExtractor;
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = CppExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
    assert!(query.contains("for_statement") || query.contains("while_statement"));
}

#[test]
fn middle_capture_name_to_chunk_type_loop() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_conditional() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_function_call() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_code_block() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown() {
    let extractor = CppExtractor;
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn capture_name_to_chunk_type_namespace() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("namespace.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_type_definition() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("type.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("enum.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_template_class() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("template_class.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_template_function() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("template_function.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_operator() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("operator.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_destructor() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("destructor.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_switch() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_while() {
    let extractor = CppExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("while_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn extract_name_returns_none_for_cpp_namespace_identifier_nodes() {
    let source_code = "namespace tools { int value = 1; }";
    let tree = parse_cpp(source_code);
    let namespace_node = find_node(
        tree.root_node(),
        source_code,
        "namespace_definition",
        "namespace tools",
    )
    .unwrap();

    let name = CppExtractor.extract_name(namespace_node, source_code, "namespace.definition");

    assert_eq!(name, None);
}

#[test]
fn extract_name_reads_constructor_and_handles_destructor_fallback() {
    let source_code = "class Resource { public: Resource() {} ~Resource() {} };";
    let tree = parse_cpp(source_code);
    let constructor_node = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "Resource() {}",
    )
    .unwrap();
    let destructor_node = find_node(
        tree.root_node(),
        source_code,
        "function_definition",
        "~Resource() {}",
    )
    .unwrap();

    let constructor_name =
        CppExtractor.extract_name(constructor_node, source_code, "constructor.definition");
    let destructor_name =
        CppExtractor.extract_name(destructor_node, source_code, "destructor.definition");

    assert_eq!(constructor_name, Some("Resource".to_string()));
    assert_eq!(destructor_name, None);
}

#[test]
fn extract_name_reads_plain_variable_declarations() {
    let source_code = "int plain;";
    let tree = parse_cpp(source_code);
    let declaration_node =
        find_node(tree.root_node(), source_code, "declaration", "plain").unwrap();

    let name = CppExtractor.extract_name(declaration_node, source_code, "variable.definition");

    assert_eq!(name, Some("plain".to_string()));
}
