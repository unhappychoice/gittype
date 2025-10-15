use gittype::domain::models::Languages;
use gittype::presentation::ui::Colors;

#[test]
fn language_registry_get_color_returns_specific_colors_for_known_languages() {
    assert_eq!(Languages::get_color(Some("rust")), Colors::lang_rust());
    assert_eq!(Languages::get_color(Some("python")), Colors::lang_python());
    assert_eq!(
        Languages::get_color(Some("javascript")),
        Colors::lang_javascript()
    );
    assert_eq!(
        Languages::get_color(Some("typescript")),
        Colors::lang_typescript()
    );
}

#[test]
fn language_registry_get_color_returns_default_for_unknown_languages() {
    assert_eq!(
        Languages::get_color(Some("unknown")),
        Colors::lang_default()
    );
    assert_eq!(Languages::get_color(Some("xyz")), Colors::lang_default());
    assert_eq!(Languages::get_color(Some("")), Colors::lang_default());
}

#[test]
fn language_registry_get_color_returns_default_for_none() {
    assert_eq!(Languages::get_color(None), Colors::lang_default());
}

#[test]
fn language_registry_get_display_name_returns_formal_names() {
    assert_eq!(Languages::get_display_name(Some("rust")), "Rust");
    assert_eq!(Languages::get_display_name(Some("python")), "Python");
    assert_eq!(
        Languages::get_display_name(Some("javascript")),
        "JavaScript"
    );
    assert_eq!(
        Languages::get_display_name(Some("typescript")),
        "TypeScript"
    );
    assert_eq!(Languages::get_display_name(Some("cpp")), "C++");
    assert_eq!(Languages::get_display_name(Some("csharp")), "C#");
}

#[test]
fn language_registry_get_display_name_is_case_insensitive() {
    assert_eq!(Languages::get_display_name(Some("RUST")), "Rust");
    assert_eq!(
        Languages::get_display_name(Some("JavaScript")),
        "JavaScript"
    );
    assert_eq!(Languages::get_display_name(Some("Python")), "Python");
}

#[test]
fn language_registry_get_display_name_preserves_unknown_languages() {
    assert_eq!(Languages::get_display_name(Some("unknown")), "unknown");
    assert_eq!(
        Languages::get_display_name(Some("CustomLang")),
        "CustomLang"
    );
}

#[test]
fn language_registry_get_display_name_returns_unknown_for_none() {
    assert_eq!(Languages::get_display_name(None), "Unknown");
}

#[test]
fn language_registry_get_by_name_finds_known_languages() {
    assert!(Languages::get_by_name("rust").is_some());
    assert!(Languages::get_by_name("python").is_some());
    assert!(Languages::get_by_name("javascript").is_some());
    assert!(Languages::get_by_name("unknown").is_none());
}

#[test]
fn language_trait_methods_work_correctly() {
    if let Some(rust_lang) = Languages::get_by_name("rust") {
        assert_eq!(rust_lang.name(), "rust");
        assert_eq!(rust_lang.display_name(), "Rust");
        assert_eq!(rust_lang.color(), Colors::lang_rust());
    }
}

#[test]
fn all_languages_returns_all_supported_languages() {
    let langs = Languages::all_languages();
    assert!(langs.len() >= 17); // At least 17 languages
}

#[test]
fn all_file_patterns_returns_patterns() {
    let patterns = Languages::all_file_patterns();
    assert!(!patterns.is_empty());
    assert!(patterns.iter().any(|p| p.contains("*.rs")));
    assert!(patterns.iter().any(|p| p.contains("*.py")));
}

#[test]
fn get_supported_languages_includes_main_names_and_aliases() {
    let supported = Languages::get_supported_languages();
    assert!(supported.contains(&"rust"));
    assert!(supported.contains(&"python"));
    assert!(supported.contains(&"javascript"));
}

#[test]
fn validate_languages_accepts_valid_languages() {
    let result = Languages::validate_languages(&vec!["rust".to_string(), "python".to_string()]);
    assert!(result.is_ok());
}

#[test]
fn validate_languages_rejects_invalid_languages() {
    let result = Languages::validate_languages(&vec!["rust".to_string(), "invalid".to_string()]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), vec!["invalid"]);
}

#[test]
fn validate_languages_accepts_empty_list() {
    let result = Languages::validate_languages(&vec![]);
    assert!(result.is_ok());
}

#[test]
fn from_extension_finds_language_by_extension() {
    assert!(Languages::from_extension("rs").is_some());
    assert!(Languages::from_extension("py").is_some());
    assert!(Languages::from_extension("js").is_some());
    assert!(Languages::from_extension("unknown_ext").is_none());
}

#[test]
fn detect_from_path_detects_rust() {
    use std::path::Path;
    assert_eq!(Languages::detect_from_path(Path::new("test.rs")), "rust");
}

#[test]
fn detect_from_path_detects_python() {
    use std::path::Path;
    assert_eq!(Languages::detect_from_path(Path::new("test.py")), "python");
}

#[test]
fn detect_from_path_returns_text_for_unknown() {
    use std::path::Path;
    assert_eq!(Languages::detect_from_path(Path::new("test.xyz")), "text");
}

#[test]
fn detect_from_path_returns_text_for_no_extension() {
    use std::path::Path;
    assert_eq!(Languages::detect_from_path(Path::new("README")), "text");
}

#[test]
fn get_by_name_is_case_insensitive() {
    assert!(Languages::get_by_name("RUST").is_some());
    assert!(Languages::get_by_name("Python").is_some());
    assert!(Languages::get_by_name("JavaScript").is_some());
}

#[test]
fn get_by_name_supports_aliases() {
    // If language has aliases, test them
    assert!(Languages::get_by_name("typescript").is_some());
    assert!(Languages::get_by_name("javascript").is_some());
}

#[test]
fn language_file_patterns_format() {
    if let Some(rust) = Languages::get_by_name("rust") {
        let patterns = rust.file_patterns();
        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.starts_with("**/")));
    }
}

#[test]
fn get_language_by_name_wrapper_works() {
    assert!(Languages::get_language_by_name("rust").is_some());
    assert!(Languages::get_language_by_name("unknown").is_none());
}
