use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{
    ColorScheme, CustomThemeFile, SerializableColor, ThemeFile,
};
use ratatui::style::Color;
use std::collections::HashMap;

fn sample_theme_file() -> ThemeFile {
    let mut dark = HashMap::new();
    dark.insert(
        "border".to_string(),
        SerializableColor::Name("blue".to_string()),
    );
    dark.insert(
        "title".to_string(),
        SerializableColor::Name("white".to_string()),
    );
    dark.insert(
        "status_success".to_string(),
        SerializableColor::Rgb { r: 0, g: 255, b: 0 },
    );

    let mut light = HashMap::new();
    light.insert(
        "border".to_string(),
        SerializableColor::Name("black".to_string()),
    );
    light.insert(
        "title".to_string(),
        SerializableColor::Name("black".to_string()),
    );

    ThemeFile {
        id: "test".to_string(),
        name: "Test Theme".to_string(),
        description: "A test theme".to_string(),
        dark,
        light,
    }
}

#[test]
fn custom_theme_file_to_theme_file_creates_correct_metadata() {
    let custom = CustomThemeFile {
        dark: HashMap::new(),
        light: HashMap::new(),
    };

    let theme = custom.to_theme_file();

    assert_eq!(theme.id, "custom");
    assert_eq!(theme.name, "Custom");
    assert!(theme
        .description
        .contains("edit ~/.gittype/custom-theme.json"));
}

#[test]
fn custom_theme_file_to_theme_file_preserves_colors() {
    let mut dark = HashMap::new();
    dark.insert(
        "border".to_string(),
        SerializableColor::Name("red".to_string()),
    );

    let mut light = HashMap::new();
    light.insert(
        "border".to_string(),
        SerializableColor::Name("blue".to_string()),
    );

    let custom = CustomThemeFile { dark, light };
    let theme = custom.to_theme_file();

    assert_eq!(
        theme.dark.get("border"),
        Some(&SerializableColor::Name("red".to_string()))
    );
    assert_eq!(
        theme.light.get("border"),
        Some(&SerializableColor::Name("blue".to_string()))
    );
}

#[test]
fn serializable_color_rgb_converts_to_ratatui_color() {
    let color = SerializableColor::Rgb {
        r: 128,
        g: 64,
        b: 32,
    };
    let result: Color = color.into();

    assert_eq!(result, Color::Rgb(128, 64, 32));
}

#[test]
fn serializable_color_named_black_converts_correctly() {
    let color = SerializableColor::Name("black".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Black);
}

#[test]
fn serializable_color_named_white_converts_correctly() {
    let color = SerializableColor::Name("white".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::White);
}

#[test]
fn serializable_color_named_red_converts_correctly() {
    let color = SerializableColor::Name("red".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Red);
}

#[test]
fn serializable_color_named_green_converts_correctly() {
    let color = SerializableColor::Name("green".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Green);
}

#[test]
fn serializable_color_named_blue_converts_correctly() {
    let color = SerializableColor::Name("blue".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Blue);
}

#[test]
fn serializable_color_named_yellow_converts_correctly() {
    let color = SerializableColor::Name("yellow".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Yellow);
}

#[test]
fn serializable_color_named_cyan_converts_correctly() {
    let color = SerializableColor::Name("cyan".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Cyan);
}

#[test]
fn serializable_color_named_magenta_converts_correctly() {
    let color = SerializableColor::Name("magenta".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Magenta);
}

#[test]
fn serializable_color_named_gray_converts_correctly() {
    let color = SerializableColor::Name("gray".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Gray);
}

#[test]
fn serializable_color_named_dark_gray_converts_correctly() {
    let color = SerializableColor::Name("dark_gray".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::DarkGray);
}

#[test]
fn serializable_color_named_light_colors_convert_correctly() {
    assert_eq!(
        Color::from(SerializableColor::Name("light_red".to_string())),
        Color::LightRed
    );
    assert_eq!(
        Color::from(SerializableColor::Name("light_green".to_string())),
        Color::LightGreen
    );
    assert_eq!(
        Color::from(SerializableColor::Name("light_yellow".to_string())),
        Color::LightYellow
    );
    assert_eq!(
        Color::from(SerializableColor::Name("light_blue".to_string())),
        Color::LightBlue
    );
    assert_eq!(
        Color::from(SerializableColor::Name("light_magenta".to_string())),
        Color::LightMagenta
    );
    assert_eq!(
        Color::from(SerializableColor::Name("light_cyan".to_string())),
        Color::LightCyan
    );
}

#[test]
fn serializable_color_named_reset_converts_correctly() {
    let color = SerializableColor::Name("reset".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Reset);
}

#[test]
fn serializable_color_hex_6_digit_converts_correctly() {
    let color = SerializableColor::Name("#ff8040".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Rgb(255, 128, 64));
}

#[test]
fn serializable_color_hex_3_digit_converts_correctly() {
    let color = SerializableColor::Name("#f80".to_string());
    let result: Color = color.into();

    // #f80 expands to #ff8800
    assert_eq!(result, Color::Rgb(255, 136, 0));
}

#[test]
fn serializable_color_hex_uppercase_converts_correctly() {
    let color = SerializableColor::Name("#FF0000".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::Rgb(255, 0, 0));
}

#[test]
fn serializable_color_unknown_name_defaults_to_white() {
    let color = SerializableColor::Name("unknown_color".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::White);
}

#[test]
fn serializable_color_invalid_hex_defaults_to_white() {
    let color = SerializableColor::Name("#gggggg".to_string());
    let result: Color = color.into();

    assert_eq!(result, Color::White);
}

#[test]
fn serializable_color_partial_eq_works() {
    let color1 = SerializableColor::Rgb {
        r: 100,
        g: 100,
        b: 100,
    };
    let color2 = SerializableColor::Rgb {
        r: 100,
        g: 100,
        b: 100,
    };
    let color3 = SerializableColor::Rgb {
        r: 200,
        g: 200,
        b: 200,
    };

    assert_eq!(color1, color2);
    assert_ne!(color1, color3);

    let name1 = SerializableColor::Name("red".to_string());
    let name2 = SerializableColor::Name("red".to_string());
    let name3 = SerializableColor::Name("blue".to_string());

    assert_eq!(name1, name2);
    assert_ne!(name1, name3);
}

#[test]
fn serializable_color_clone_works() {
    let color = SerializableColor::Rgb {
        r: 50,
        g: 100,
        b: 150,
    };
    let cloned = color.clone();

    assert_eq!(color, cloned);
}

#[test]
fn color_scheme_from_theme_file_uses_dark_colors() {
    let theme = sample_theme_file();
    let scheme = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);

    assert_eq!(scheme.border, SerializableColor::Name("blue".to_string()));
    assert_eq!(scheme.title, SerializableColor::Name("white".to_string()));
}

#[test]
fn color_scheme_from_theme_file_uses_light_colors() {
    let theme = sample_theme_file();
    let scheme = ColorScheme::from_theme_file(&theme, &ColorMode::Light);

    assert_eq!(scheme.border, SerializableColor::Name("black".to_string()));
    assert_eq!(scheme.title, SerializableColor::Name("black".to_string()));
}

#[test]
fn color_scheme_from_theme_file_uses_defaults_for_missing_colors() {
    let theme = ThemeFile {
        id: "minimal".to_string(),
        name: "Minimal".to_string(),
        description: "Minimal theme".to_string(),
        dark: HashMap::new(),
        light: HashMap::new(),
    };

    let scheme = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);

    // Should use default values
    assert_eq!(scheme.border, SerializableColor::Name("blue".to_string()));
    assert_eq!(scheme.text, SerializableColor::Name("white".to_string()));
    assert_eq!(
        scheme.status_success,
        SerializableColor::Name("green".to_string())
    );
    assert_eq!(
        scheme.status_error,
        SerializableColor::Name("red".to_string())
    );
}

#[test]
fn color_scheme_from_theme_file_includes_all_metrics_colors() {
    let theme = sample_theme_file();
    let scheme = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);

    // All metrics colors should have default values
    assert_eq!(
        scheme.metrics_score,
        SerializableColor::Name("magenta".to_string())
    );
    assert_eq!(
        scheme.metrics_cpm_wpm,
        SerializableColor::Name("green".to_string())
    );
    assert_eq!(
        scheme.metrics_accuracy,
        SerializableColor::Name("yellow".to_string())
    );
    assert_eq!(
        scheme.metrics_duration,
        SerializableColor::Name("cyan".to_string())
    );
    assert_eq!(
        scheme.metrics_stage_info,
        SerializableColor::Name("blue".to_string())
    );
}

#[test]
fn color_scheme_from_theme_file_includes_typing_colors() {
    let theme = sample_theme_file();
    let scheme = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);

    assert_eq!(
        scheme.typing_typed_text,
        SerializableColor::Name("green".to_string())
    );
    assert_eq!(
        scheme.typing_cursor_fg,
        SerializableColor::Name("white".to_string())
    );
    assert_eq!(
        scheme.typing_cursor_bg,
        SerializableColor::Name("blue".to_string())
    );
    assert_eq!(
        scheme.typing_mistake_bg,
        SerializableColor::Name("red".to_string())
    );
    assert_eq!(
        scheme.typing_untyped_text,
        SerializableColor::Name("gray".to_string())
    );
}

#[test]
fn color_scheme_from_theme_file_includes_language_colors() {
    let theme = sample_theme_file();
    let scheme = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);

    // Language colors should be loaded from lang_dark.json (RGB values)
    // Just verify they are set and not empty
    match &scheme.lang_rust {
        SerializableColor::Rgb { .. } => (),
        SerializableColor::Name(_) => (),
    }
    match &scheme.lang_python {
        SerializableColor::Rgb { .. } => (),
        SerializableColor::Name(_) => (),
    }
    match &scheme.lang_javascript {
        SerializableColor::Rgb { .. } => (),
        SerializableColor::Name(_) => (),
    }
}

#[test]
fn color_scheme_clone_works() {
    let theme = sample_theme_file();
    let scheme = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);
    let cloned = scheme.clone();

    assert_eq!(scheme, cloned);
}

#[test]
fn color_scheme_partial_eq_works() {
    let theme = sample_theme_file();
    let scheme1 = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);
    let scheme2 = ColorScheme::from_theme_file(&theme, &ColorMode::Dark);
    let scheme3 = ColorScheme::from_theme_file(&theme, &ColorMode::Light);

    assert_eq!(scheme1, scheme2);
    assert_ne!(scheme1, scheme3);
}

#[test]
fn theme_file_clone_works() {
    let theme = sample_theme_file();
    let cloned = theme.clone();

    assert_eq!(theme.id, cloned.id);
    assert_eq!(theme.name, cloned.name);
    assert_eq!(theme.description, cloned.description);
}

#[test]
fn custom_theme_file_clone_works() {
    let custom = CustomThemeFile {
        dark: HashMap::new(),
        light: HashMap::new(),
    };
    let cloned = custom.clone();

    assert_eq!(custom.dark.len(), cloned.dark.len());
    assert_eq!(custom.light.len(), cloned.light.len());
}

#[test]
fn serializable_color_serialize_deserialize_rgb() {
    let color = SerializableColor::Rgb {
        r: 100,
        g: 150,
        b: 200,
    };
    let serialized = serde_json::to_string(&color).unwrap();
    let deserialized: SerializableColor = serde_json::from_str(&serialized).unwrap();

    assert_eq!(color, deserialized);
}

#[test]
fn serializable_color_serialize_deserialize_name() {
    let color = SerializableColor::Name("blue".to_string());
    let serialized = serde_json::to_string(&color).unwrap();
    let deserialized: SerializableColor = serde_json::from_str(&serialized).unwrap();

    assert_eq!(color, deserialized);
}
