use gittype::domain::services::source_code_parser::parsers::clojure::ClojureExtractor;
use gittype::domain::services::source_code_parser::parsers::LanguageExtractor;

#[test]
fn create_parser_succeeds() {
    let result = ClojureExtractor::create_parser();
    assert!(result.is_ok());
}

#[test]
fn tree_sitter_language_returns_language() {
    let extractor = ClojureExtractor;
    let _language = extractor.tree_sitter_language();
}

#[test]
fn query_patterns_returns_non_empty() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(!patterns.is_empty());
}

#[test]
fn comment_query_returns_non_empty() {
    let extractor = ClojureExtractor;
    let query = extractor.comment_query();
    assert!(!query.is_empty());
}

#[test]
fn middle_implementation_query_returns_non_empty() {
    let extractor = ClojureExtractor;
    let query = extractor.middle_implementation_query();
    assert!(!query.is_empty());
}
