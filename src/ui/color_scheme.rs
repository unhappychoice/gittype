use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ui::color_mode::ColorMode;

const LANG_DARK: &str = include_str!("../../assets/languages/lang_dark.json");
const LANG_LIGHT: &str = include_str!("../../assets/languages/lang_light.json");
const LANG_ASCII: &str = include_str!("../../assets/languages/lang_ascii.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeFile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub dark: HashMap<String, SerializableColor>,
    pub light: HashMap<String, SerializableColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeFile {
    pub dark: HashMap<String, SerializableColor>,
    pub light: HashMap<String, SerializableColor>,
}

impl CustomThemeFile {
    /// Convert CustomThemeFile to ThemeFile with fixed metadata
    pub fn to_theme_file(&self) -> ThemeFile {
        ThemeFile {
            id: "custom".to_string(),
            name: "Custom".to_string(),
            description: "Your personal custom theme - edit ~/.gittype/custom-theme.json to customize".to_string(),
            dark: self.dark.clone(),
            light: self.light.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SerializableColor {
    Rgb { r: u8, g: u8, b: u8 },
    Name(String),
}

impl From<SerializableColor> for Color {
    fn from(serializable_color: SerializableColor) -> Self {
        match serializable_color {
            SerializableColor::Rgb { r, g, b } => Color::Rgb(r, g, b),
            SerializableColor::Name(name) => match name.as_str() {
                "reset" => Color::Reset,
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" => Color::Magenta,
                "cyan" => Color::Cyan,
                "gray" => Color::Gray,
                "dark_gray" => Color::DarkGray,
                "light_red" => Color::LightRed,
                "light_green" => Color::LightGreen,
                "light_yellow" => Color::LightYellow,
                "light_blue" => Color::LightBlue,
                "light_magenta" => Color::LightMagenta,
                "light_cyan" => Color::LightCyan,
                "white" => Color::White,
                // Add more named colors as needed
                _ => {
                    // Try to parse as hex color (#RRGGBB or #RGB)
                    if let Some(hex) = name.strip_prefix('#') {
                        if hex.len() == 6 {
                            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                                let r = ((rgb >> 16) & 0xFF) as u8;
                                let g = ((rgb >> 8) & 0xFF) as u8;
                                let b = (rgb & 0xFF) as u8;
                                return Color::Rgb(r, g, b);
                            }
                        } else if hex.len() == 3 {
                            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                                let r = (((rgb >> 8) & 0xF) * 0x11) as u8;
                                let g = (((rgb >> 4) & 0xF) * 0x11) as u8;
                                let b = ((rgb & 0xF) * 0x11) as u8;
                                return Color::Rgb(r, g, b);
                            }
                        }
                    }
                    // Fallback to white for unknown color names
                    Color::White
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScheme {
    // Primary colors for main UI elements
    pub border: SerializableColor,
    pub title: SerializableColor,
    pub text: SerializableColor,
    pub text_secondary: SerializableColor,
    pub background: SerializableColor,
    pub background_secondary: SerializableColor,

    // Status and feedback colors
    pub status_success: SerializableColor,
    pub status_error: SerializableColor,
    pub status_warning: SerializableColor,
    pub status_info: SerializableColor,

    // Specific UI element colors
    pub key_action: SerializableColor,
    pub key_navigation: SerializableColor,
    pub key_back: SerializableColor,

    // Metrics and performance colors
    pub metrics_score: SerializableColor,
    pub metrics_cpm_wpm: SerializableColor,
    pub metrics_accuracy: SerializableColor,
    pub metrics_duration: SerializableColor,
    pub metrics_stage_info: SerializableColor,

    // Typing interface colors
    pub typing_typed_text: SerializableColor,
    pub typing_cursor_fg: SerializableColor,
    pub typing_cursor_bg: SerializableColor,
    pub typing_mistake_bg: SerializableColor,
    pub typing_untyped_text: SerializableColor,

    // Programming language colors
    pub lang_rust: SerializableColor,
    pub lang_python: SerializableColor,
    pub lang_javascript: SerializableColor,
    pub lang_typescript: SerializableColor,
    pub lang_go: SerializableColor,
    pub lang_java: SerializableColor,
    pub lang_c: SerializableColor,
    pub lang_cpp: SerializableColor,
    pub lang_csharp: SerializableColor,
    pub lang_php: SerializableColor,
    pub lang_ruby: SerializableColor,
    pub lang_swift: SerializableColor,
    pub lang_kotlin: SerializableColor,
    pub lang_scala: SerializableColor,
    pub lang_haskell: SerializableColor,
    pub lang_dart: SerializableColor,
    pub lang_default: SerializableColor,
}

impl ColorScheme {
    /// Create ColorScheme from ThemeFile for specific color mode
    pub fn from_theme_file(theme_file: &ThemeFile, color_mode: &ColorMode) -> Self {
        let colors = match color_mode {
            ColorMode::Dark => &theme_file.dark,
            ColorMode::Light => &theme_file.light,
        };
        let lang_colors = Self::load_language_colors(theme_file, color_mode);

        Self {
            border: colors.get("border").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            title: colors.get("title").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            text: colors.get("text").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            text_secondary: colors.get("text_secondary").cloned().unwrap_or(SerializableColor::Name("gray".to_string())),
            background: colors.get("background").cloned().unwrap_or(SerializableColor::Name("black".to_string())),
            background_secondary: colors.get("background_secondary").cloned().unwrap_or(SerializableColor::Name("black".to_string())),

            status_success: colors.get("status_success").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            status_error: colors.get("status_error").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            status_warning: colors.get("status_warning").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            status_info: colors.get("status_info").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),

            key_action: colors.get("key_action").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            key_navigation: colors.get("key_navigation").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            key_back: colors.get("key_back").cloned().unwrap_or(SerializableColor::Name("red".to_string())),

            metrics_score: colors.get("metrics_score").cloned().unwrap_or(SerializableColor::Name("magenta".to_string())),
            metrics_cpm_wpm: colors.get("metrics_cpm_wpm").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            metrics_accuracy: colors.get("metrics_accuracy").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            metrics_duration: colors.get("metrics_duration").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),
            metrics_stage_info: colors.get("metrics_stage_info").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),

            typing_typed_text: colors.get("typing_typed_text").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            typing_cursor_fg: colors.get("typing_cursor_fg").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            typing_cursor_bg: colors.get("typing_cursor_bg").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            typing_mistake_bg: colors.get("typing_mistake_bg").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            typing_untyped_text: colors.get("typing_untyped_text").cloned().unwrap_or(SerializableColor::Name("gray".to_string())),

            lang_rust: lang_colors.get("lang_rust").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            lang_python: lang_colors.get("lang_python").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            lang_javascript: lang_colors.get("lang_javascript").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            lang_typescript: lang_colors.get("lang_typescript").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            lang_go: lang_colors.get("lang_go").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),
            lang_java: lang_colors.get("lang_java").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            lang_c: lang_colors.get("lang_c").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            lang_cpp: lang_colors.get("lang_cpp").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            lang_csharp: lang_colors.get("lang_csharp").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            lang_php: lang_colors.get("lang_php").cloned().unwrap_or(SerializableColor::Name("magenta".to_string())),
            lang_ruby: lang_colors.get("lang_ruby").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            lang_swift: lang_colors.get("lang_swift").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            lang_kotlin: lang_colors.get("lang_kotlin").cloned().unwrap_or(SerializableColor::Name("magenta".to_string())),
            lang_scala: lang_colors.get("lang_scala").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            lang_haskell: lang_colors.get("lang_haskell").cloned().unwrap_or(SerializableColor::Name("magenta".to_string())),
            lang_dart: lang_colors.get("lang_dart").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            lang_default: lang_colors.get("lang_default").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
        }
    }


    fn load_language_colors(theme_file: &ThemeFile, color_mode: &ColorMode) -> HashMap<String, SerializableColor> {
        let lang_json = match (theme_file.id.as_str(), color_mode) {
            ("ascii", _) => LANG_ASCII,
            (_, ColorMode::Light) => LANG_LIGHT,
            (_, ColorMode::Dark) => LANG_DARK,
        };
        serde_json::from_str(lang_json).unwrap_or_default()
    }

}
