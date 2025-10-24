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

#[test]
fn query_patterns_matches_defn() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defn"));
}

#[test]
fn query_patterns_matches_defmacro() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defmacro"));
}

#[test]
fn query_patterns_matches_defn_dash() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defn-"));
}

#[test]
fn query_patterns_matches_deftype() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("deftype"));
}

#[test]
fn query_patterns_matches_defprotocol() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defprotocol"));
}

#[test]
fn query_patterns_matches_defrecord() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    assert!(patterns.contains("defrecord"));
}

#[test]
fn query_uses_single_match_predicate() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    let match_count = patterns.matches("#match?").count();
    assert_eq!(match_count, 1, "Should use exactly one #match? predicate");
}

#[test]
fn query_does_not_use_multiple_eq_predicates() {
    let extractor = ClojureExtractor;
    let patterns = extractor.query_patterns();
    let eq_count = patterns.matches("#eq?").count();
    assert_eq!(eq_count, 0, "Should not use #eq? predicates");
}
