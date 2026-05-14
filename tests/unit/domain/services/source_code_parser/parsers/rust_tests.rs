use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::rust::RustExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;

#[test]
fn create_parser_succeeds() {
    let result = RustExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = RustExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = RustExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = RustExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = RustExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_returns_none_for_unknown_capture() {
    let extractor = RustExtractor;

    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_returns_none_for_unknown_capture() {
    let extractor = RustExtractor;

    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("unknown"),
        None::<ChunkType>
    );
}
