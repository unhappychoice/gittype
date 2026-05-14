use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::go::GoExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_go(source: &str) -> tree_sitter::Tree {
    let mut parser = GoExtractor::create_parser().unwrap();
    parser.parse(source, None).unwrap()
}

fn find_node<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    if node.kind() == kind {
        return Some(node);
    }
    (0..node.named_child_count())
        .filter_map(|index| node.named_child(index))
        .find_map(|child| find_node(child, kind))
}

fn first_leaf(node: Node) -> Node {
    node.named_child(0).map_or(node, first_leaf)
}

#[test]
fn create_parser_succeeds() {
    let result = GoExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = GoExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = GoExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = GoExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_function() {
    let extractor = GoExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
}

#[test]
fn capture_name_to_chunk_type_method() {
    let extractor = GoExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Method)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown() {
    let extractor = GoExtractor;
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = GoExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_capture_name_to_chunk_type_loop() {
    let extractor = GoExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_conditional() {
    let extractor = GoExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown() {
    let extractor = GoExtractor;
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn extract_name_for_const_block_returns_single_identifier() {
    let source = "package main\nconst answer = 42\n";
    let tree = parse_go(source);
    let declaration =
        find_node(tree.root_node(), "const_declaration").expect("expected const_declaration");

    let name = GoExtractor.extract_name(declaration, source, "const_block");

    assert_eq!(name.as_deref(), Some("answer"));
}

#[test]
fn extract_name_for_const_block_summarizes_multiple_identifiers() {
    let source = "package main\nconst (\nAlpha = 1\nBeta = 2\n)\n";
    let tree = parse_go(source);
    let declaration =
        find_node(tree.root_node(), "const_declaration").expect("expected const_declaration");

    let name = GoExtractor.extract_name(declaration, source, "const_block");

    assert_eq!(name.as_deref(), Some("Alpha, Beta (2)"));
}

#[test]
fn extract_name_for_var_block_returns_single_identifier() {
    let source = "package main\nvar total int\n";
    let tree = parse_go(source);
    let declaration =
        find_node(tree.root_node(), "var_declaration").expect("expected var_declaration");

    let name = GoExtractor.extract_name(declaration, source, "var_block");

    assert_eq!(name.as_deref(), Some("total"));
}

#[test]
fn extract_name_for_const_block_without_specs_returns_fallback_name() {
    let source = "package main\nconst answer = 42\n";
    let tree = parse_go(source);
    let leaf = first_leaf(tree.root_node());

    let name = GoExtractor.extract_name(leaf, source, "const_block");

    assert_eq!(name.as_deref(), Some("const_block"));
}
