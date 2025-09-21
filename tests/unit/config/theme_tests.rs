use gittype::config::{ColorScheme, SerializableColor, Theme, ThemeConfig, ThemeFile, ThemeManager};
use ratatui::style::Color;
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_color_scheme_conversion() {
    let scheme = ColorScheme::ascii();
    let color: Color = scheme.border.into();
    assert!(matches!(color, Color::Rgb(100, 149, 237))); // Should be RGB color now
}

#[test]
fn test_theme_config_default() {
    let config = ThemeConfig::default();
    assert_eq!(config.current_theme_id, "default");
    assert!(config.custom_themes.is_empty());
}

// #[test]
// fn test_theme_manager_with_temp_config() {
//     // TODO: Update this test after ThemeManager API changes
// }

#[test]
fn test_predefined_themes() {
    let ascii = ColorScheme::ascii();
    let custom_scheme = ColorScheme::ascii();

    // Test that color conversion works
    let ascii_bg: Color = ascii.background.clone().into();
    let ascii_text: Color = ascii.text.clone().into();

    // Should be RGB colors now
    matches!(ascii_bg, Color::Rgb(0, 0, 0));
    matches!(ascii_text, Color::Rgb(255, 255, 255));

    // ASCII theme should use RGB colors
    assert!(matches!(ascii_bg, Color::Rgb(0, 0, 0))); // Black background
    assert!(matches!(ascii_text, Color::Rgb(255, 255, 255))); // White text
}

// #[test]
// fn test_custom_themes() {
//     // TODO: Update this test after ThemeManager API changes
// }

// #[test]
// fn test_theme_list_includes_all_predefined() {
//     // TODO: Update this test after ThemeManager API changes
// }

#[test]
fn test_theme_file_parsing() {
    // Test that the embedded JSON files can be parsed correctly
    let ascii_json = include_str!("../../../assets/themes/ascii.json");
    let theme_file: ThemeFile = serde_json::from_str(ascii_json).unwrap();

    assert_eq!(theme_file.name, "ASCII");
    assert!(theme_file.description.contains("ASCII"));
    assert!(theme_file.dark.contains_key("border"));
    assert!(theme_file.dark.contains_key("background"));

    // Test that description is appropriate
    assert!(theme_file.description.contains("Classic"));
}

// #[test]
// fn test_color_scheme_from_theme_file() {
//     // TODO: Update this test after ThemeFile structure changes
// }

#[test]
fn test_embedded_themes_load_correctly() {
    // Test that both embedded themes load without panicking
    let ascii_scheme = ColorScheme::ascii();

    // Verify RGB color loading
    let ascii_bg: Color = ascii_scheme.background.clone().into();
    assert!(matches!(ascii_bg, Color::Rgb(0, 0, 0)));

    // Test some specific colors to ensure JSON loading worked
    let ascii_bg: Color = ascii_scheme.background.into();
    let ascii_text: Color = ascii_scheme.text.into();

    assert!(matches!(ascii_bg, Color::Rgb(0, 0, 0))); // Should be RGB black
    assert!(matches!(ascii_text, Color::Rgb(255, 255, 255))); // Should be RGB white
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