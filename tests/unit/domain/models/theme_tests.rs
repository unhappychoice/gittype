use gittype::domain::models::theme::Theme;

#[test]
fn all_themes_returns_expected_count() {
    let themes = Theme::all_themes();
    // Should match the number of JSON files in THEME_FILES
    assert_eq!(themes.len(), 15);
}

#[test]
fn all_themes_includes_default_theme() {
    let themes = Theme::all_themes();
    let default_theme = themes.iter().find(|t| t.id == "default");

    assert!(default_theme.is_some());
    let theme = default_theme.unwrap();
    assert_eq!(theme.name, "Default");
}

#[test]
fn all_themes_includes_original_theme() {
    let themes = Theme::all_themes();
    let original_theme = themes.iter().find(|t| t.id == "original");

    assert!(original_theme.is_some());
}

#[test]
fn all_themes_includes_ascii_theme() {
    let themes = Theme::all_themes();
    let ascii_theme = themes.iter().find(|t| t.id == "ascii");

    assert!(ascii_theme.is_some());
}

#[test]
fn all_themes_includes_aurora_theme() {
    let themes = Theme::all_themes();
    let aurora_theme = themes.iter().find(|t| t.id == "aurora");

    assert!(aurora_theme.is_some());
}

#[test]
fn all_themes_each_has_required_fields() {
    let themes = Theme::all_themes();

    for theme in themes {
        assert!(!theme.id.is_empty(), "Theme ID should not be empty");
        assert!(!theme.name.is_empty(), "Theme name should not be empty");
        assert!(
            !theme.description.is_empty(),
            "Theme description should not be empty"
        );
        // ColorScheme fields exist (light and dark)
        // Just checking they can be accessed
        let _ = &theme.light;
        let _ = &theme.dark;
    }
}

#[test]
fn default_theme_returns_default_id() {
    let theme = Theme::default();
    assert_eq!(theme.id, "default");
}

#[test]
fn default_theme_has_valid_name_and_description() {
    let theme = Theme::default();
    assert_eq!(theme.name, "Default");
    assert!(!theme.description.is_empty());
}

#[test]
fn default_theme_has_light_and_dark_schemes() {
    let theme = Theme::default();

    // Verify light and dark color schemes exist
    let _ = &theme.light;
    let _ = &theme.dark;
}

#[test]
fn theme_clone_works() {
    let theme = Theme::default();
    let cloned = theme.clone();

    assert_eq!(theme.id, cloned.id);
    assert_eq!(theme.name, cloned.name);
    assert_eq!(theme.description, cloned.description);
}

#[test]
fn theme_partial_eq_works() {
    let theme1 = Theme::default();
    let theme2 = Theme::default();

    assert_eq!(theme1, theme2);
}

#[test]
fn theme_partial_eq_different_themes() {
    let themes = Theme::all_themes();
    let default_theme = themes.iter().find(|t| t.id == "default").unwrap();
    let original_theme = themes.iter().find(|t| t.id == "original").unwrap();

    assert_ne!(default_theme, original_theme);
}

#[test]
fn all_themes_have_unique_ids() {
    let themes = Theme::all_themes();
    let mut ids = themes.iter().map(|t| &t.id).collect::<Vec<_>>();
    ids.sort();
    ids.dedup();

    // If all IDs are unique, length should match original
    assert_eq!(ids.len(), themes.len());
}

#[test]
fn all_themes_include_expected_theme_ids() {
    let themes = Theme::all_themes();
    let ids: Vec<&str> = themes.iter().map(|t| t.id.as_str()).collect();

    assert!(ids.contains(&"default"));
    assert!(ids.contains(&"original"));
    assert!(ids.contains(&"ascii"));
    assert!(ids.contains(&"aurora"));
    assert!(ids.contains(&"blood_oath"));
    assert!(ids.contains(&"cyber_void"));
    assert!(ids.contains(&"eclipse"));
    assert!(ids.contains(&"glacier"));
    assert!(ids.contains(&"inferno"));
    assert!(ids.contains(&"neon_abyss"));
    assert!(ids.contains(&"oblivion"));
    assert!(ids.contains(&"runic"));
    assert!(ids.contains(&"spectral"));
    assert!(ids.contains(&"starforge"));
    assert!(ids.contains(&"venom"));
}

#[test]
fn theme_serialize_deserialize() {
    let theme = Theme::default();
    let serialized = serde_json::to_string(&theme).unwrap();
    let deserialized: Theme = serde_json::from_str(&serialized).unwrap();

    assert_eq!(theme, deserialized);
}

#[test]
fn theme_debug_format_works() {
    let theme = Theme::default();
    let debug_str = format!("{:?}", theme);

    assert!(debug_str.contains("Theme"));
    assert!(debug_str.contains("default"));
}
