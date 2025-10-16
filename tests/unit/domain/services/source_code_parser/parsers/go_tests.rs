use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::go::GoExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;

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
