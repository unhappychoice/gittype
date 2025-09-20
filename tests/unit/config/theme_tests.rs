use gittype::config::{ColorScheme, SerializableColor, Theme, ThemeConfig, ThemeFile, ThemeManager};
use ratatui::style::Color;
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_color_scheme_conversion() {
    let scheme = ColorScheme::ascii_dark();
    let color: Color = scheme.border.into();
    assert_eq!(color, Color::Blue); // Should be named color, not RGB
}

#[test]
fn test_theme_config_default() {
    let config = ThemeConfig::default();
    assert_eq!(config.current_theme, Theme::AsciiDark);
    assert!(config.custom_themes.is_empty());
}

#[test]
fn test_theme_manager_with_temp_config() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("theme.json");

    let mut manager = ThemeManager::with_config_path(config_path.clone()).unwrap();
    assert_eq!(*manager.get_current_theme(), Theme::AsciiDark);

    // Set theme and verify persistence
    manager.set_theme(Theme::AsciiLight).unwrap();
    assert_eq!(*manager.get_current_theme(), Theme::AsciiLight);

    // Create new manager with same config path
    let manager2 = ThemeManager::with_config_path(config_path).unwrap();
    assert_eq!(*manager2.get_current_theme(), Theme::AsciiLight);
}

#[test]
fn test_predefined_themes() {
    let ascii_dark = ColorScheme::ascii_dark();
    let ascii_light = ColorScheme::ascii_light();

    // Test that themes are different by converting to Color
    let ascii_dark_bg: Color = ascii_dark.background.clone().into();
    let ascii_light_bg: Color = ascii_light.background.clone().into();
    assert_ne!(ascii_dark_bg, ascii_light_bg);

    let ascii_dark_text: Color = ascii_dark.text.clone().into();
    let ascii_light_text: Color = ascii_light.text.clone().into();
    assert_ne!(ascii_dark_text, ascii_light_text);

    // ASCII dark should use black background, white text
    assert_eq!(ascii_dark_bg, Color::Black);
    assert_eq!(ascii_dark_text, Color::White);

    // ASCII light theme should use white background, black text
    assert_eq!(ascii_light_bg, Color::White);
    assert_eq!(ascii_light_text, Color::Black);
}

#[test]
fn test_custom_themes() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("theme.json");

    let mut manager = ThemeManager::with_config_path(config_path).unwrap();

    let custom_scheme = ColorScheme::ascii_dark();
    manager.add_custom_theme("my_theme".to_string(), custom_scheme).unwrap();

    let themes = manager.list_themes();
    assert!(themes.contains(&"my_theme".to_string()));

    manager.set_theme(Theme::Custom("my_theme".to_string())).unwrap();
    assert_eq!(*manager.get_current_theme(), Theme::Custom("my_theme".to_string()));
}

#[test]
fn test_theme_list_includes_all_predefined() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("theme.json");
    let manager = ThemeManager::with_config_path(config_path).unwrap();

    let themes = manager.list_themes();
    assert!(themes.contains(&"ascii_dark".to_string()));
    assert!(themes.contains(&"ascii_light".to_string()));
}

#[test]
fn test_theme_file_parsing() {
    // Test that the embedded JSON files can be parsed correctly
    let ascii_dark_json = include_str!("../../../assets/themes/ascii_dark.json");
    let theme_file: ThemeFile = serde_json::from_str(ascii_dark_json).unwrap();

    assert_eq!(theme_file.name, "ASCII Dark");
    assert!(theme_file.description.contains("ASCII"));
    assert!(theme_file.colors.contains_key("border"));
    assert!(theme_file.colors.contains_key("background"));

    let ascii_light_json = include_str!("../../../assets/themes/ascii_light.json");
    let ascii_light_theme: ThemeFile = serde_json::from_str(ascii_light_json).unwrap();

    assert_eq!(ascii_light_theme.name, "ASCII Light");
    assert!(ascii_light_theme.description.contains("ASCII"));
}

#[test]
fn test_color_scheme_from_theme_file() {
    // Create a simple theme file for testing
    let mut colors = HashMap::new();
    colors.insert("border".to_string(), SerializableColor::Name("red".to_string()));
    colors.insert("background".to_string(), SerializableColor::Name("black".to_string()));

    let theme_file = ThemeFile {
        name: "Test Theme".to_string(),
        description: "A test theme".to_string(),
        colors,
    };

    let color_scheme = ColorScheme::from_theme_file(&theme_file);

    // Test that our custom colors are applied
    let border_color: Color = color_scheme.border.into();
    assert_eq!(border_color, Color::Red);

    let bg_color: Color = color_scheme.background.into();
    assert_eq!(bg_color, Color::Black);
}

#[test]
fn test_embedded_themes_load_correctly() {
    // Test that both embedded themes load without panicking
    let ascii_dark_scheme = ColorScheme::ascii_dark();
    let ascii_light_scheme = ColorScheme::ascii_light();

    // Verify they're different
    let ascii_dark_bg: Color = ascii_dark_scheme.background.clone().into();
    let ascii_light_bg: Color = ascii_light_scheme.background.clone().into();
    assert_ne!(ascii_dark_bg, ascii_light_bg);

    // Test some specific colors to ensure JSON loading worked
    let ascii_dark_bg: Color = ascii_dark_scheme.background.into();
    let ascii_light_bg: Color = ascii_light_scheme.background.into();

    assert_eq!(ascii_dark_bg, Color::Black); // ASCII Dark should have black background
    assert_eq!(ascii_light_bg, Color::White); // ASCII Light should have white background
}

#[test]
fn test_color_name_parsing() {
    // Test basic color names
    let red_color = SerializableColor::Name("red".to_string());
    let color: Color = red_color.into();
    assert_eq!(color, Color::Red);

    let blue_color = SerializableColor::Name("blue".to_string());
    let color: Color = blue_color.into();
    assert_eq!(color, Color::Blue);

    // Test case insensitive
    let white_color = SerializableColor::Name("WHITE".to_string());
    let color: Color = white_color.into();
    assert_eq!(color, Color::White);

    // Test underscore variations
    let dark_gray_color = SerializableColor::Name("dark_gray".to_string());
    let color: Color = dark_gray_color.into();
    assert_eq!(color, Color::DarkGray);

    let light_blue_color = SerializableColor::Name("light_blue".to_string());
    let color: Color = light_blue_color.into();
    assert_eq!(color, Color::LightBlue);
}

#[test]
fn test_hex_color_parsing() {
    // Test hex color support
    let hex_color = SerializableColor::Name("#ff0000".to_string());
    let color: Color = hex_color.into();
    assert_eq!(color, Color::Rgb(255, 0, 0));

    let hex_blue = SerializableColor::Name("#0000ff".to_string());
    let color: Color = hex_blue.into();
    assert_eq!(color, Color::Rgb(0, 0, 255));

    // Test invalid hex fallback to white
    let invalid_hex = SerializableColor::Name("#invalid".to_string());
    let color: Color = invalid_hex.into();
    assert_eq!(color, Color::White);
}

#[test]
fn test_rgb_and_name_serialization() {
    // Test that both RGB and name formats work
    let rgb_color = SerializableColor::Rgb { r: 255, g: 128, b: 0 };
    let color: Color = rgb_color.into();
    assert_eq!(color, Color::Rgb(255, 128, 0));

    let name_color = SerializableColor::Name("cyan".to_string());
    let color: Color = name_color.into();
    assert_eq!(color, Color::Cyan);
}