use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::haskell::HaskellExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;

#[test]
fn create_parser_succeeds() {
    let result = HaskellExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = HaskellExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = HaskellExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = HaskellExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = HaskellExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_returns_none_for_unknown_capture() {
    let extractor = HaskellExtractor;

    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_to_chunk_type_maps_special_haskell_blocks() {
    let extractor = HaskellExtractor;

    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("do_block"),
        Some(ChunkType::SpecialBlock)
    );
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("list_comp"),
        Some(ChunkType::Comprehension)
    );
}

#[test]
fn middle_capture_name_to_chunk_type_returns_none_for_unknown_capture() {
    let extractor = HaskellExtractor;

    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}
