use gittype::domain::models::ExtractionOptions;

#[test]
fn test_default_extraction_options() {
    let options = ExtractionOptions::default();

    // Should have include patterns
    assert!(!options.include_patterns.is_empty());

    // Should have exclude patterns
    assert!(!options.exclude_patterns.is_empty());

    // Should exclude common build directories
    assert!(options
        .exclude_patterns
        .contains(&"**/target/**".to_string()));
    assert!(options
        .exclude_patterns
        .contains(&"**/node_modules/**".to_string()));
    assert!(options
        .exclude_patterns
        .contains(&"**/build/**".to_string()));

    // No language filter by default
    assert!(options.languages.is_none());

    // Default max file size should be 1MB
    assert_eq!(options.max_file_size_bytes, 1024 * 1024);
}

#[test]
fn test_custom_extraction_options() {
    let options = ExtractionOptions {
        include_patterns: vec!["**/*.rs".to_string()],
        exclude_patterns: vec!["**/tests/**".to_string()],
        languages: Some(vec!["rust".to_string()]),
        max_file_size_bytes: 2 * 1024 * 1024, // 2MB
    };

    assert_eq!(options.include_patterns.len(), 1);
    assert_eq!(options.exclude_patterns.len(), 1);
    assert_eq!(options.languages, Some(vec!["rust".to_string()]));
    assert_eq!(options.max_file_size_bytes, 2 * 1024 * 1024);
}

#[test]
fn test_apply_language_filter_with_rust() {
    let mut options = ExtractionOptions {
        languages: Some(vec!["rust".to_string()]),
        ..Default::default()
    };

    options.apply_language_filter();

    // Should only have Rust patterns
    assert!(options.include_patterns.iter().any(|p| p.contains(".rs")));
    // Should not have Python patterns
    assert!(!options
        .include_patterns
        .iter()
        .any(|p| p.contains(".py") && !p.contains(".rs")));
}

#[test]
fn test_apply_language_filter_with_multiple_languages() {
    let mut options = ExtractionOptions {
        languages: Some(vec!["rust".to_string(), "python".to_string()]),
        ..Default::default()
    };

    options.apply_language_filter();

    // Should have both Rust and Python patterns
    assert!(options.include_patterns.iter().any(|p| p.contains(".rs")));
    assert!(options.include_patterns.iter().any(|p| p.contains(".py")));
}

#[test]
fn test_apply_language_filter_with_no_languages() {
    let mut options = ExtractionOptions::default();
    let original_patterns = options.include_patterns.clone();

    // No language filter
    options.apply_language_filter();

    // Patterns should remain unchanged
    assert_eq!(options.include_patterns, original_patterns);
}

#[test]
fn test_apply_language_filter_case_insensitive() {
    let mut options = ExtractionOptions {
        languages: Some(vec!["RUST".to_string()]),
        ..Default::default()
    };

    options.apply_language_filter();

    // Should work with uppercase
    assert!(options.include_patterns.iter().any(|p| p.contains(".rs")));
}

#[test]
fn test_clone_extraction_options() {
    let options = ExtractionOptions {
        include_patterns: vec!["**/*.rs".to_string()],
        exclude_patterns: vec!["**/tests/**".to_string()],
        languages: Some(vec!["rust".to_string()]),
        max_file_size_bytes: 2 * 1024 * 1024,
    };

    let cloned = options.clone();

    assert_eq!(options.include_patterns, cloned.include_patterns);
    assert_eq!(options.exclude_patterns, cloned.exclude_patterns);
    assert_eq!(options.languages, cloned.languages);
    assert_eq!(options.max_file_size_bytes, cloned.max_file_size_bytes);
}

#[test]
fn test_default_excludes_python_cache() {
    let options = ExtractionOptions::default();

    assert!(options
        .exclude_patterns
        .contains(&"**/__pycache__/**".to_string()));
    assert!(options.exclude_patterns.contains(&"**/*.pyc".to_string()));
}

#[test]
fn test_default_excludes_git() {
    let options = ExtractionOptions::default();

    assert!(options.exclude_patterns.contains(&"**/.git/**".to_string()));
}

#[test]
fn test_default_excludes_generated_code() {
    let options = ExtractionOptions::default();

    assert!(options
        .exclude_patterns
        .contains(&"**/generated/**".to_string()));
    assert!(options.exclude_patterns.contains(&"**/gen/**".to_string()));
}
