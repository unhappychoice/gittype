use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::ruby::RubyExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;
use tree_sitter::Node;

fn parse_ruby(source: &str) -> tree_sitter::Tree {
    let mut parser = RubyExtractor::create_parser().unwrap();
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
fn create_parser_succeeds() {
    let result = RubyExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = RubyExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = RubyExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = RubyExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_method() {
    let extractor = RubyExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Method)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    let extractor = RubyExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown() {
    let extractor = RubyExtractor;
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = RubyExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_capture_name_to_chunk_type_loop() {
    let extractor = RubyExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("for_loop"),
        Some(ChunkType::Loop)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_conditional() {
    let extractor = RubyExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("if_block"),
        Some(ChunkType::Conditional)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_unknown() {
    let extractor = RubyExtractor;
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn extract_name_reads_method_identifier() {
    let extractor = RubyExtractor;
    let source = "def greet\n  :ok\nend\n";
    let tree = parse_ruby(source);
    let method = find_first_node(tree.root_node(), "method").unwrap();

    assert_eq!(
        extractor.extract_name(method, source, "method"),
        Some("greet".to_string())
    );
}

#[test]
fn extract_name_reads_attr_accessor_symbols() {
    let extractor = RubyExtractor;
    let source = "class User\n  attr_accessor :name, :email\nend\n";
    let tree = parse_ruby(source);
    let call = find_first_node(tree.root_node(), "call").unwrap();

    assert_eq!(
        extractor.extract_name(call, source, "attr_accessor"),
        Some("name, email (2)".to_string())
    );
}

#[test]
fn extract_name_returns_unknown_attr_when_accessor_has_no_symbols() {
    let extractor = RubyExtractor;
    let source = "class User\n  attr_accessor \"name\"\nend\n";
    let tree = parse_ruby(source);
    let call = find_first_node(tree.root_node(), "call").unwrap();

    assert_eq!(
        extractor.extract_name(call, source, "attr_accessor"),
        Some("unknown_attr".to_string())
    );
}

#[test]
fn extract_name_returns_none_for_leaf_without_identifier() {
    let extractor = RubyExtractor;
    let source = "42\n";
    let tree = parse_ruby(source);
    let integer = find_first_node(tree.root_node(), "integer").unwrap();

    assert_eq!(extractor.extract_name(integer, source, "method"), None);
}
