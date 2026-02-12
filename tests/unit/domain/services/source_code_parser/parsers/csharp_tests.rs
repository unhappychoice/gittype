use gittype::domain::models::ChunkType;
use gittype::domain::services::source_code_parser::parsers::csharp::CSharpExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;

#[test]
fn create_parser_succeeds() {
    let result = CSharpExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = CSharpExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = CSharpExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = CSharpExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = CSharpExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}

#[test]
fn capture_name_to_chunk_type_event() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("event"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_delegate() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("delegate"),
        Some(ChunkType::Method)
    );
}

#[test]
fn capture_name_to_chunk_type_class() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("class"),
        Some(ChunkType::Class)
    );
}

#[test]
fn capture_name_to_chunk_type_struct() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("struct"),
        Some(ChunkType::Struct)
    );
}

#[test]
fn capture_name_to_chunk_type_interface() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("interface"),
        Some(ChunkType::Interface)
    );
}

#[test]
fn capture_name_to_chunk_type_enum() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("enum"),
        Some(ChunkType::Enum)
    );
}

#[test]
fn capture_name_to_chunk_type_method() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("method"),
        Some(ChunkType::Method)
    );
}

#[test]
fn capture_name_to_chunk_type_property() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("property"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_field() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("field"),
        Some(ChunkType::Variable)
    );
}

#[test]
fn capture_name_to_chunk_type_namespace() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.capture_name_to_chunk_type("namespace"),
        Some(ChunkType::Namespace)
    );
}

#[test]
fn capture_name_to_chunk_type_unknown() {
    let extractor = CSharpExtractor;
    assert_eq!(extractor.capture_name_to_chunk_type("unknown"), None);
}

#[test]
fn middle_capture_name_lambda() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("lambda"),
        Some(ChunkType::Lambda)
    );
}

#[test]
fn middle_capture_name_loop_variants() {
    let extractor = CSharpExtractor;
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
}

#[test]
fn middle_capture_name_conditional() {
    let extractor = CSharpExtractor;
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
fn middle_capture_name_error_handling() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("try_block"),
        Some(ChunkType::ErrorHandling)
    );
}

#[test]
fn middle_capture_name_method_call() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("method_call"),
        Some(ChunkType::FunctionCall)
    );
}

#[test]
fn middle_capture_name_code_block() {
    let extractor = CSharpExtractor;
    assert_eq!(
        extractor.middle_capture_name_to_chunk_type("code_block"),
        Some(ChunkType::CodeBlock)
    );
}

#[test]
fn middle_capture_name_unknown() {
    let extractor = CSharpExtractor;
    assert_eq!(extractor.middle_capture_name_to_chunk_type("unknown"), None);
}
