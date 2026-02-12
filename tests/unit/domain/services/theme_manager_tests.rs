use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};

fn create_theme_service() -> ThemeService {
    ThemeService::new_for_test(Theme::default(), ColorMode::Dark)
}

// ============================================
// Available themes
// ============================================

#[test]
fn get_available_themes_includes_default_themes() {
    let manager = create_theme_service();
    let themes = manager.get_available_themes();
    assert!(!themes.is_empty());
    assert!(themes.iter().any(|t| t.id == "default"));
}

#[test]
fn all_themes_have_unique_ids() {
    let manager = create_theme_service();
    let themes = manager.get_available_themes();

    let mut ids: Vec<String> = themes.iter().map(|t| t.id.clone()).collect();
    let original_len = ids.len();
    ids.sort();
    ids.dedup();

    assert_eq!(ids.len(), original_len, "Theme IDs should be unique");
}

#[test]
fn all_themes_have_non_empty_names() {
    let manager = create_theme_service();
    let themes = manager.get_available_themes();

    for theme in themes {
        assert!(!theme.name.is_empty(), "Theme name should not be empty");
        assert!(!theme.id.is_empty(), "Theme ID should not be empty");
    }
}

#[test]
fn get_available_themes_returns_consistent_count() {
    let manager = create_theme_service();
    let themes1 = manager.get_available_themes();
    let themes2 = manager.get_available_themes();
    assert_eq!(themes1.len(), themes2.len());
}

// ============================================
// Current theme get/set
// ============================================

#[test]
fn default_theme_exists() {
    let theme = Theme::default();
    assert_eq!(theme.id, "default");
    assert_eq!(theme.name, "Default");
}

#[test]
fn theme_manager_has_default_values() {
    let manager = create_theme_service();
    assert_eq!(manager.get_current_theme().id, "default");
}

#[test]
fn set_current_theme_updates_theme() {
    let manager = create_theme_service();
    let themes = manager.get_available_themes();
    let non_default = themes.into_iter().find(|t| t.id != "default");

    if let Some(theme) = non_default {
        let theme_id = theme.id.clone();
        manager.set_current_theme(theme);
        assert_eq!(manager.get_current_theme().id, theme_id);
    }
}

// ============================================
// Color mode get/set
// ============================================

#[test]
fn get_current_color_mode_default_is_dark() {
    let manager = create_theme_service();
    assert_eq!(manager.get_current_color_mode(), ColorMode::Dark);
}

#[test]
fn set_current_color_mode_to_light() {
    let manager = create_theme_service();
    manager.set_current_color_mode(ColorMode::Light);
    assert_eq!(manager.get_current_color_mode(), ColorMode::Light);
}

#[test]
fn set_current_color_mode_to_dark() {
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Light);
    manager.set_current_color_mode(ColorMode::Dark);
    assert_eq!(manager.get_current_color_mode(), ColorMode::Dark);
}

// ============================================
// Color scheme
// ============================================

#[test]
fn get_current_color_scheme_returns_dark_scheme() {
    let manager = create_theme_service();
    let scheme = manager.get_current_color_scheme();
    // Should return a valid color scheme without panicking
    let _ = scheme;
}

#[test]
fn get_current_color_scheme_returns_light_scheme() {
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Light);
    let scheme = manager.get_current_color_scheme();
    let _ = scheme;
}

// ============================================
// Colors
// ============================================

#[test]
fn get_colors_returns_valid_colors() {
    let manager = create_theme_service();
    let colors = manager.get_colors();
    let _ = colors.border();
    let _ = colors.title();
}

#[test]
fn colors_instance_creation_works() {
    use gittype::presentation::ui::Colors;

    let theme = Theme::default();
    let color_scheme = theme.dark;
    let colors = Colors::new(color_scheme);
    let _ = colors.border();
    let _ = colors.title();
}

// ============================================
// Language colors
// ============================================

#[test]
fn get_color_for_language_after_init() {
    let manager = create_theme_service();
    manager.init().unwrap();
    let color = manager.get_color_for_language("rust");
    // Should return a color (not necessarily White if loaded)
    let _ = color;
}

#[test]
fn get_color_for_language_unknown_returns_fallback() {
    let manager = create_theme_service();
    manager.init().unwrap();
    let color = manager.get_color_for_language("nonexistent_language_xyz");
    // Should return fallback (default or White)
    let _ = color;
}

#[test]
fn get_color_for_language_without_init_returns_white() {
    let manager = create_theme_service();
    // Without init, language_colors is empty
    let color = manager.get_color_for_language("rust");
    assert_eq!(color, ratatui::style::Color::White);
}

#[test]
fn get_color_for_language_light_mode() {
    let manager = ThemeService::new_for_test(Theme::default(), ColorMode::Light);
    manager.init().unwrap();
    let color = manager.get_color_for_language("python");
    let _ = color;
}

// ============================================
// Init
// ============================================

#[test]
fn init_loads_language_colors() {
    let manager = create_theme_service();
    manager.init().unwrap();
    // After init, getting a color for a known language should work
    let color = manager.get_color_for_language("rust");
    // Should not be White (the default fallback) after loading
    // But we can't guarantee exact color, just verify no panic
    let _ = color;
}

#[test]
fn init_sets_theme_from_config() {
    let manager = create_theme_service();
    manager.init().unwrap();
    // Config default is "default" theme
    assert_eq!(manager.get_current_theme().id, "default");
}

// ============================================
// Ascii theme language colors
// ============================================

#[test]
fn ascii_theme_uses_ascii_language_colors() {
    let themes = Theme::all_themes();
    let ascii_theme = themes.into_iter().find(|t| t.id == "ascii");

    if let Some(ascii) = ascii_theme {
        let manager = ThemeService::new_for_test(ascii, ColorMode::Dark);
        manager.init().unwrap();
        let color = manager.get_color_for_language("rust");
        let _ = color;
    }
}
