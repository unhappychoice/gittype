use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::php::PhpExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

#[test]
fn create_parser_succeeds() {
    let result = PhpExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = PhpExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = PhpExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = PhpExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = PhpExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_maps_php_constructs() {
    let extractor = PhpExtractor;

    assert_eq!(
        extractor.capture_name_to_chunk_type("function"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("interface"),
        Some(ChunkType::Class)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("trait"),
        Some(ChunkType::Class)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("namespace"),
        Some(ChunkType::Function)
    );
    assert_eq!(
        extractor.capture_name_to_chunk_type("name"),
        Some(ChunkType::CodeBlock)
    );
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_maps_php_blocks() {
    let extractor = PhpExtractor;

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
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("switch_block"),
        Some(ChunkType::Conditional)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("try_block"),
        Some(ChunkType::ErrorHandling)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("function_call"),
        Some(ChunkType::FunctionCall)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn extract_name_reads_name_capture_directly() {
    let extractor = PhpExtractor;
    let source = "<?php function greet() { return true; }";
    let tree = PhpExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();
    let name_node = find_node(tree.root_node(), "name").unwrap();

    let name = extractor.extract_name(name_node, source, "name");

    assert_eq!(name, Some("greet".to_string()));
}

#[test]
fn extract_name_falls_back_to_name_child() {
    let extractor = PhpExtractor;
    let source = "<?php class Greeter { public function hello() {} }";
    let tree = PhpExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();
    let class_node = find_node(tree.root_node(), "class_declaration").unwrap();

    let name = extractor.extract_name(class_node, source, "class");

    assert_eq!(name, Some("Greeter".to_string()));
}

#[test]
fn extract_name_returns_none_for_leaf_node_without_children() {
    let extractor = PhpExtractor;
    let source = "<?php 42;";
    let tree = PhpExtractor::create_parser()
        .unwrap()
        .parse(source, None)
        .unwrap();
    let leaf = find_leaf(tree.root_node());

    let name = extractor.extract_name(leaf, source, "function");

    assert_eq!(name, None);
}

fn find_node<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    (node.kind() == kind).then_some(node).or_else(|| {
        let mut cursor = node.walk();
        let result = node
            .children(&mut cursor)
            .find_map(|child| find_node(child, kind));
        result
    })
}

fn find_leaf(node: Node<'_>) -> Node<'_> {
    if node.child_count() == 0 {
        return node;
    }

    (0..node.child_count())
        .filter_map(|index| node.child(index))
        .map(find_leaf)
        .next()
        .unwrap()
}
