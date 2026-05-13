use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::c::CExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_c(source: &str) -> tree_sitter::Tree {
    let mut parser = CExtractor::create_parser().unwrap();
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

fn first_leaf(node: Node<'_>) -> Node<'_> {
    if node.child_count() == 0 {
        return node;
    }
    first_leaf(node.child(0).unwrap())
}

#[test]
fn create_parser_succeeds() {
    let result = CExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = CExtractor;
    let _language = extractor.tree_sitter_language();
    // Just verify it doesn't panic
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = CExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
    assert!(patterns.contains("function"));
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = CExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
    assert!(query.contains("comment"));
}

#[test]
fn capture_name_to_chunk_type_function() {
    let extractor = CExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("function.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_struct() {
    let extractor = CExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("struct.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_variable() {
    let extractor = CExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("variable.definition"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_macro() {
    let extractor = CExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("macro.definition"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown() {
    let extractor = CExtractor;
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = CExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
    assert!(query.contains("for_statement") || query.contains("while_statement"));
}

#[test]
fn middle_capture_name_to_chunk_type_loop() {
    let extractor = CExtractor;
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
    let extractor = CExtractor;
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
fn middle_capture_name_to_chunk_type_function_call() {
    let extractor = CExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_code_block() {
    let extractor = CExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown() {
    let extractor = CExtractor;
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn capture_name_to_chunk_type_type_definition_returns_struct() {
    assert_eq!(
        CExtractor.capture_name_to_chunk_type("type.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_enum_definition_returns_struct() {
    assert_eq!(
        CExtractor.capture_name_to_chunk_type("enum.definition"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_named_capture_returns_code_block() {
    let extractor = CExtractor;
    for name in [
        "function.name",
        "struct.name",
        "type.name",
        "enum.name",
        "variable.name",
        "macro.name",
    ] {
        assert_eq!(
            extractor.capture_name_to_chunk_type(name),
            Some(ChunkType::CodeBlock),
            "{} should map to CodeBlock",
            name
        );
    }
}

#[test]
fn extract_name_for_function_definition_returns_identifier() {
    let source = "int add(int a, int b) { return a + b; }\n";
    let tree = parse_c(source);
    let func = find_node(tree.root_node(), "function_definition")
        .expect("expected a function_definition node");

    let name = CExtractor.extract_name(func, source, "function.definition");

    assert_eq!(name.as_deref(), Some("add"));
}

#[test]
fn extract_name_for_struct_definition_returns_type_identifier() {
    let source = "struct Point { int x; int y; };\n";
    let tree = parse_c(source);
    let strukt =
        find_node(tree.root_node(), "struct_specifier").expect("expected a struct_specifier node");

    let name = CExtractor.extract_name(strukt, source, "struct.definition");

    assert_eq!(name.as_deref(), Some("Point"));
}

#[test]
fn extract_name_for_type_definition_returns_type_identifier() {
    let source = "typedef unsigned int MyUInt;\n";
    let tree = parse_c(source);
    let typedef =
        find_node(tree.root_node(), "type_definition").expect("expected a type_definition node");

    let name = CExtractor.extract_name(typedef, source, "type.definition");

    assert_eq!(name.as_deref(), Some("MyUInt"));
}

#[test]
fn extract_name_for_enum_definition_returns_type_identifier() {
    let source = "enum Color { RED, GREEN, BLUE };\n";
    let tree = parse_c(source);
    let enm =
        find_node(tree.root_node(), "enum_specifier").expect("expected an enum_specifier node");

    let name = CExtractor.extract_name(enm, source, "enum.definition");

    assert_eq!(name.as_deref(), Some("Color"));
}

#[test]
fn extract_name_for_variable_definition_with_init_returns_identifier() {
    let source = "int counter = 0;\n";
    let tree = parse_c(source);
    let decl = find_node(tree.root_node(), "declaration").expect("expected a declaration node");

    let name = CExtractor.extract_name(decl, source, "variable.definition");

    assert_eq!(name.as_deref(), Some("counter"));
}

#[test]
fn extract_name_for_macro_definition_returns_identifier() {
    let source = "#define MAX_LEN 256\n";
    let tree = parse_c(source);
    let macr = find_node(tree.root_node(), "preproc_def").expect("expected a preproc_def node");

    let name = CExtractor.extract_name(macr, source, "macro.definition");

    assert_eq!(name.as_deref(), Some("MAX_LEN"));
}

#[test]
fn extract_name_for_unknown_capture_returns_none() {
    let source = "int x;\n";
    let tree = parse_c(source);
    let root = tree.root_node();

    let name = CExtractor.extract_name(root, source, "unknown.capture");

    assert_eq!(name, None);
}

#[test]
fn extract_name_for_leaf_node_returns_none() {
    let source = "42;\n";
    let tree = parse_c(source);
    let leaf = first_leaf(tree.root_node());

    let name = CExtractor.extract_name(leaf, source, "function.definition");

    assert_eq!(name, None);
}
