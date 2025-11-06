use crate::domain::models::color_mode::ColorMode;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            description:
                "Your personal custom theme - edit ~/.gittype/custom-theme.json to customize"
                    .to_string(),
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
            },
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
}

impl ColorScheme {
    /// Create ColorScheme from ThemeFile for specific color mode
    pub fn from_theme_file(theme_file: &ThemeFile, color_mode: &ColorMode) -> Self {
        let colors = match color_mode {
            ColorMode::Dark => &theme_file.dark,
            ColorMode::Light => &theme_file.light,
        };

        Self {
            border: colors
                .get("border")
                .cloned()
                .unwrap_or(SerializableColor::Name("blue".to_string())),
            title: colors
                .get("title")
                .cloned()
                .unwrap_or(SerializableColor::Name("white".to_string())),
            text: colors
                .get("text")
                .cloned()
                .unwrap_or(SerializableColor::Name("white".to_string())),
            text_secondary: colors
                .get("text_secondary")
                .cloned()
                .unwrap_or(SerializableColor::Name("gray".to_string())),
            background: colors
                .get("background")
                .cloned()
                .unwrap_or(SerializableColor::Name("black".to_string())),
            background_secondary: colors
                .get("background_secondary")
                .cloned()
                .unwrap_or(SerializableColor::Name("black".to_string())),

            status_success: colors
                .get("status_success")
                .cloned()
                .unwrap_or(SerializableColor::Name("green".to_string())),
            status_error: colors
                .get("status_error")
                .cloned()
                .unwrap_or(SerializableColor::Name("red".to_string())),
            status_warning: colors
                .get("status_warning")
                .cloned()
                .unwrap_or(SerializableColor::Name("yellow".to_string())),
            status_info: colors
                .get("status_info")
                .cloned()
                .unwrap_or(SerializableColor::Name("blue".to_string())),

            key_action: colors
                .get("key_action")
                .cloned()
                .unwrap_or(SerializableColor::Name("blue".to_string())),
            key_navigation: colors
                .get("key_navigation")
                .cloned()
                .unwrap_or(SerializableColor::Name("blue".to_string())),
            key_back: colors
                .get("key_back")
                .cloned()
                .unwrap_or(SerializableColor::Name("red".to_string())),

            metrics_score: colors
                .get("metrics_score")
                .cloned()
                .unwrap_or(SerializableColor::Name("magenta".to_string())),
            metrics_cpm_wpm: colors
                .get("metrics_cpm_wpm")
                .cloned()
                .unwrap_or(SerializableColor::Name("green".to_string())),
            metrics_accuracy: colors
                .get("metrics_accuracy")
                .cloned()
                .unwrap_or(SerializableColor::Name("yellow".to_string())),
            metrics_duration: colors
                .get("metrics_duration")
                .cloned()
                .unwrap_or(SerializableColor::Name("cyan".to_string())),
            metrics_stage_info: colors
                .get("metrics_stage_info")
                .cloned()
                .unwrap_or(SerializableColor::Name("blue".to_string())),

            typing_typed_text: colors
                .get("typing_typed_text")
                .cloned()
                .unwrap_or(SerializableColor::Name("green".to_string())),
            typing_cursor_fg: colors
                .get("typing_cursor_fg")
                .cloned()
                .unwrap_or(SerializableColor::Name("white".to_string())),
            typing_cursor_bg: colors
                .get("typing_cursor_bg")
                .cloned()
                .unwrap_or(SerializableColor::Name("blue".to_string())),
            typing_mistake_bg: colors
                .get("typing_mistake_bg")
                .cloned()
                .unwrap_or(SerializableColor::Name("red".to_string())),
            typing_untyped_text: colors
                .get("typing_untyped_text")
                .cloned()
                .unwrap_or(SerializableColor::Name("gray".to_string())),
        }
    }
}
