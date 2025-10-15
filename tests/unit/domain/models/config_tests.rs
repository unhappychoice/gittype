use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, SerializableColor, ThemeFile};
use gittype::domain::models::config::ThemeConfig;
use ratatui::style::Color;

#[test]
fn test_color_scheme_conversion() {
    // Load ascii theme file and create color scheme
    let ascii_json = include_str!("../../../../assets/themes/ascii.json");
    let theme_file: ThemeFile = serde_json::from_str(ascii_json).unwrap();
    let scheme = ColorScheme::from_theme_file(&theme_file, &ColorMode::Dark);
    let color: Color = scheme.border.into();
    // ASCII theme uses named colors like "blue"
    assert_eq!(color, Color::Blue);
}

#[test]
fn test_theme_config_default() {
    let config = ThemeConfig::default();
    assert_eq!(config.current_theme_id, "default");
    // ThemeConfig no longer has custom_themes field
}

#[test]
fn test_predefined_themes() {
    // Load ascii theme file and create color schemes
    let ascii_json = include_str!("../../../../assets/themes/ascii.json");
    let theme_file: ThemeFile = serde_json::from_str(ascii_json).unwrap();
    let ascii = ColorScheme::from_theme_file(&theme_file, &ColorMode::Dark);

    // Test that color conversion works
    let ascii_bg: Color = ascii.background.clone().into();
    let ascii_text: Color = ascii.text.clone().into();

    // ASCII theme uses named colors
    assert_eq!(ascii_bg, Color::Black);
    assert_eq!(ascii_text, Color::White);
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
    let ascii_json = include_str!("../../../../assets/themes/ascii.json");
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
    // Test that embedded themes load without panicking
    let ascii_json = include_str!("../../../../assets/themes/ascii.json");
    let theme_file: ThemeFile = serde_json::from_str(ascii_json).unwrap();
    let ascii_scheme = ColorScheme::from_theme_file(&theme_file, &ColorMode::Dark);

    // Verify color loading
    let ascii_bg: Color = ascii_scheme.background.clone().into();
    assert_eq!(ascii_bg, Color::Black);

    // Test some specific colors to ensure JSON loading worked
    let ascii_bg: Color = ascii_scheme.background.into();
    let ascii_text: Color = ascii_scheme.text.into();

    assert_eq!(ascii_bg, Color::Black); // Should be black
    assert_eq!(ascii_text, Color::White); // Should be white
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
    let rgb_color = SerializableColor::Rgb {
        r: 255,
        g: 128,
        b: 0,
    };
    let color: Color = rgb_color.into();
    assert_eq!(color, Color::Rgb(255, 128, 0));

    let name_color = SerializableColor::Name("cyan".to_string());
    let color: Color = name_color.into();
    assert_eq!(color, Color::Cyan);
}

#[test]
fn test_config_default() {
    use gittype::domain::models::config::Config;

    let config = Config::default();
    assert_eq!(config.theme.current_theme_id, "default");
    assert_eq!(config.theme.current_color_mode, ColorMode::Dark);
}

#[test]
fn test_config_serialize_deserialize() {
    use gittype::domain::models::config::Config;

    let config = Config::default();
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: Config = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.theme.current_theme_id, deserialized.theme.current_theme_id);
    assert_eq!(config.theme.current_color_mode, deserialized.theme.current_color_mode);
}

#[test]
fn test_theme_config_clone() {
    let config = ThemeConfig::default();
    let cloned = config.clone();

    assert_eq!(config.current_theme_id, cloned.current_theme_id);
    assert_eq!(config.current_color_mode, cloned.current_color_mode);
}
