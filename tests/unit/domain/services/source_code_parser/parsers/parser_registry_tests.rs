use gittype::domain::services::source_code_parser::parsers::{
    get_parser_registry, parse_with_thread_local,
};

#[test]
fn supported_languages_returns_all_18() {
    let registry = get_parser_registry();
    let languages = registry.supported_languages();
    assert!(
        languages.len() >= 18,
        "Expected at least 18 languages, got {}",
        languages.len()
    );
    assert!(languages.contains(&"rust".to_string()));
    assert!(languages.contains(&"python".to_string()));
    assert!(languages.contains(&"go".to_string()));
    assert!(languages.contains(&"typescript".to_string()));
}

#[test]
fn create_parser_unsupported_language_returns_error() {
    let registry = get_parser_registry();
    let result = registry.create_parser("brainfuck");
    assert!(result.is_err());
    let err = result.err().unwrap().to_string();
    assert!(err.contains("Unsupported language"), "Error: {}", err);
}

#[test]
fn get_extractor_unsupported_language_returns_error() {
    let registry = get_parser_registry();
    let result = registry.get_extractor("brainfuck");
    assert!(result.is_err());
}

#[test]
fn create_query_unsupported_language_returns_error() {
    let registry = get_parser_registry();
    let result = registry.create_query("brainfuck");
    assert!(result.is_err());
}

#[test]
fn create_comment_query_unsupported_language_returns_error() {
    let registry = get_parser_registry();
    let result = registry.create_comment_query("brainfuck");
    assert!(result.is_err());
}

#[test]
fn create_middle_implementation_query_unsupported_language_returns_error() {
    let registry = get_parser_registry();
    let result = registry.create_middle_implementation_query("brainfuck");
    assert!(result.is_err());
}

#[test]
fn create_parser_for_all_supported_languages_succeeds() {
    let registry = get_parser_registry();
    for lang in registry.supported_languages() {
        let result = registry.create_parser(&lang);
        assert!(
            result.is_ok(),
            "create_parser failed for '{}': {:?}",
            lang,
            result.err()
        );
    }
}

#[test]
fn create_query_for_all_supported_languages_succeeds() {
    let registry = get_parser_registry();
    for lang in registry.supported_languages() {
        let result = registry.create_query(&lang);
        assert!(
            result.is_ok(),
            "create_query failed for '{}': {:?}",
            lang,
            result.err()
        );
    }
}

#[test]
fn create_comment_query_for_all_supported_languages_succeeds() {
    let registry = get_parser_registry();
    for lang in registry.supported_languages() {
        let result = registry.create_comment_query(&lang);
        assert!(
            result.is_ok(),
            "create_comment_query failed for '{}': {:?}",
            lang,
            result.err()
        );
    }
}

#[test]
fn create_middle_implementation_query_for_all_supported_languages_succeeds() {
    let registry = get_parser_registry();
    for lang in registry.supported_languages() {
        let result = registry.create_middle_implementation_query(&lang);
        assert!(
            result.is_ok(),
            "create_middle_implementation_query failed for '{}': {:?}",
            lang,
            result.err()
        );
    }
}

// ---------------------------------------------------------------------------
// parse_with_thread_local
// ---------------------------------------------------------------------------

#[test]
fn parse_with_thread_local_valid_rust() {
    let tree = parse_with_thread_local("rust", "fn main() {}");
    assert!(tree.is_some());
}

#[test]
fn parse_with_thread_local_valid_python() {
    let tree = parse_with_thread_local("python", "def hello():\n    pass");
    assert!(tree.is_some());
}

#[test]
fn parse_with_thread_local_unsupported_returns_none() {
    let tree = parse_with_thread_local("brainfuck", "+++>+++");
    assert!(tree.is_none());
}

#[test]
fn parse_with_thread_local_reuses_cached_parser() {
    // First call caches the parser
    let tree1 = parse_with_thread_local("rust", "fn a() {}");
    assert!(tree1.is_some());
    // Second call should reuse the cached parser (exercises Some(p) branch)
    let tree2 = parse_with_thread_local("rust", "fn b() {}");
    assert!(tree2.is_some());
}

#[test]
fn parse_with_thread_local_empty_content() {
    let tree = parse_with_thread_local("rust", "");
    assert!(tree.is_some()); // tree-sitter can parse empty input
}
