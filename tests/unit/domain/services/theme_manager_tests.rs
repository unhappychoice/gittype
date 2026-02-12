use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};

#[test]
fn get_available_themes_includes_default_themes() {
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Dark);
    let themes = manager.get_available_themes();
    assert!(!themes.is_empty());
    // Should have at least the built-in themes
    assert!(themes.iter().any(|t| t.id == "default"));
}

#[test]
fn colors_instance_creation_works() {
    use gittype::presentation::ui::Colors;

    let theme = Theme::default();
    let color_scheme = theme.dark;
    let colors = Colors::new(color_scheme);
    // If no panic, the test passes
    // Verify we can call color methods
    let _ = colors.border();
    let _ = colors.title();
}

#[test]
fn all_themes_have_unique_ids() {
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Dark);
    let themes = manager.get_available_themes();

    let mut ids: Vec<String> = themes.iter().map(|t| t.id.clone()).collect();
    let original_len = ids.len();
    ids.sort();
    ids.dedup();

    assert_eq!(ids.len(), original_len, "Theme IDs should be unique");
}

#[test]
fn all_themes_have_non_empty_names() {
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Dark);
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
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Dark);
    assert_eq!(manager.get_current_theme().id, "default");
}

#[test]
fn get_available_themes_returns_consistent_count() {
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Dark);
    let themes1 = manager.get_available_themes();
    let themes2 = manager.get_available_themes();

    // Should return same number of themes on multiple calls
    assert_eq!(themes1.len(), themes2.len());
}
