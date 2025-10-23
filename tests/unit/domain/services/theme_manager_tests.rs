use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_manager::ThemeManager;

#[test]
fn get_available_themes_includes_default_themes() {
    let manager = ThemeManager::new_for_test(Theme::default(), ColorMode::Dark);
    let themes = manager.get_available_themes();
    assert!(!themes.is_empty());
    // Should have at least the built-in themes
    assert!(themes.iter().any(|t| t.id == "default"));
}

#[test]
fn init_returns_ok() {
    let result = ThemeManager::init();
    assert!(result.is_ok());
}

#[test]
fn all_themes_have_unique_ids() {
    let manager = ThemeManager::new_for_test(Theme::default(), ColorMode::Dark);
    let themes = manager.get_available_themes();

    let mut ids: Vec<String> = themes.iter().map(|t| t.id.clone()).collect();
    let original_len = ids.len();
    ids.sort();
    ids.dedup();

    assert_eq!(ids.len(), original_len, "Theme IDs should be unique");
}

#[test]
fn all_themes_have_non_empty_names() {
    let manager = ThemeManager::new_for_test(Theme::default(), ColorMode::Dark);
    let themes = manager.get_available_themes();

    for theme in themes {
        assert!(!theme.name.is_empty(), "Theme name should not be empty");
        assert!(!theme.id.is_empty(), "Theme ID should not be empty");
    }
}

#[test]
fn default_theme_exists() {
    let theme = Theme::default();
    assert_eq!(theme.id, "default");
    assert_eq!(theme.name, "Default");
}

#[test]
fn theme_manager_has_default_values() {
    let manager = ThemeManager::new_for_test(Theme::default(), ColorMode::Dark);
    assert_eq!(manager.current_theme.id, "default");
}

#[test]
fn get_available_themes_returns_consistent_count() {
    let manager = ThemeManager::new_for_test(Theme::default(), ColorMode::Dark);
    let themes1 = manager.get_available_themes();
    let themes2 = manager.get_available_themes();

    // Should return same number of themes on multiple calls
    assert_eq!(themes1.len(), themes2.len());
}
