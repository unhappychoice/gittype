use gittype::domain::services::source_code_parser::parsers::scala::ScalaExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;

#[test]
fn create_parser_succeeds() {
    let result = ScalaExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = ScalaExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = ScalaExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = ScalaExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = ScalaExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn unknown_capture_names_return_none() {
    let extractor = ScalaExtractor;

    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn extract_name_returns_none_when_node_has_no_identifier() {
    let extractor = ScalaExtractor;
    let source = "class Sample\n";
    let tree = parse_scala(source);

    assert_eq!(
        extractor.extract_name(tree.root_node(), source, "class"),
        None
    );
}

fn parse_scala(source: &str) -> tree_sitter::Tree {
    let mut parser = ScalaExtractor::create_parser().unwrap();
    parser.parse(source, None).unwrap()
}
