use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::c::CExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;

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
