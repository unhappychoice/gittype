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
