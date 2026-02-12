use gittype::domain::models::Languages;
use ratatui::style::Color;
use std::hash::{Hash, Hasher};

#[test]
fn language_color_returns_specific_colors_for_known_languages() {
    let rust = Languages::get_by_name("rust").unwrap();
    assert_eq!(rust.color(), Color::Red);
    let python = Languages::get_by_name("python").unwrap();
    assert_eq!(python.color(), Color::Blue);
    let js = Languages::get_by_name("javascript").unwrap();
    assert_eq!(js.color(), Color::Yellow);
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
    let rust_lang = Languages::get_by_name("rust").unwrap();
    assert_eq!(rust_lang.name(), "rust");
    assert_eq!(rust_lang.display_name(), "Rust");
    assert_eq!(rust_lang.color(), Color::Red);
}

#[test]
fn all_languages_returns_all_supported_languages() {
    let langs = Languages::all_languages();
    assert!(langs.len() >= 17);
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
    let result = Languages::validate_languages(&["rust".to_string(), "python".to_string()]);
    assert!(result.is_ok());
}

#[test]
fn validate_languages_rejects_invalid_languages() {
    let result = Languages::validate_languages(&["rust".to_string(), "invalid".to_string()]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), vec!["invalid"]);
}

#[test]
fn validate_languages_accepts_empty_list() {
    let result = Languages::validate_languages(&[]);
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
    assert!(Languages::get_by_name("typescript").is_some());
    assert!(Languages::get_by_name("javascript").is_some());
}

#[test]
fn language_file_patterns_format() {
    let rust = Languages::get_by_name("rust").unwrap();
    let patterns = rust.file_patterns();
    assert!(!patterns.is_empty());
    assert!(patterns.iter().any(|p| p.starts_with("**/")));
}

#[test]
fn get_language_by_name_wrapper_works() {
    assert!(Languages::get_language_by_name("rust").is_some());
    assert!(Languages::get_language_by_name("unknown").is_none());
}

#[test]
fn language_as_hash_key_returns_name() {
    let rust = Languages::get_by_name("rust").unwrap();
    assert_eq!(rust.as_hash_key(), "rust");
}

#[test]
fn dyn_language_hash_and_eq() {
    let rust = Languages::get_by_name("rust").unwrap();
    let python = Languages::get_by_name("python").unwrap();
    let rust2 = Languages::get_by_name("rust").unwrap();

    // PartialEq
    assert_eq!(
        rust.as_ref() as &dyn gittype::domain::models::Language,
        rust2.as_ref() as &dyn gittype::domain::models::Language
    );
    assert_ne!(
        rust.as_ref() as &dyn gittype::domain::models::Language,
        python.as_ref() as &dyn gittype::domain::models::Language
    );

    // Hash
    let mut h1 = std::collections::hash_map::DefaultHasher::new();
    (rust.as_ref() as &dyn gittype::domain::models::Language).hash(&mut h1);
    let hash1 = h1.finish();

    let mut h2 = std::collections::hash_map::DefaultHasher::new();
    (rust2.as_ref() as &dyn gittype::domain::models::Language).hash(&mut h2);
    let hash2 = h2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
fn all_languages_have_display_name() {
    for lang in Languages::all_languages() {
        let dn = lang.display_name();
        assert!(!dn.is_empty(), "{} has empty display_name", lang.name());
    }
}

#[test]
fn all_languages_have_color() {
    for lang in Languages::all_languages() {
        let _ = lang.color(); // should not panic
    }
}

#[test]
fn all_languages_have_extensions() {
    for lang in Languages::all_languages() {
        assert!(
            !lang.extensions().is_empty(),
            "{} has no extensions",
            lang.name()
        );
    }
}
