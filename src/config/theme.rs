use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    AsciiDark,
    AsciiLight,
    Custom(String),
}

impl Default for Theme {
    fn default() -> Self {
        Theme::AsciiDark
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    // Primary colors for main UI elements
    pub border: SerializableColor,
    pub title: SerializableColor,
    pub text: SerializableColor,
    pub background: SerializableColor,

    // Status and feedback colors
    pub success: SerializableColor,
    pub error: SerializableColor,
    pub warning: SerializableColor,
    pub info: SerializableColor,

    // Specific UI element colors
    pub back_key: SerializableColor,
    pub action_key: SerializableColor,
    pub navigation_key: SerializableColor,
    pub highlight: SerializableColor,

    // Metrics and performance colors
    pub score: SerializableColor,
    pub cpm_wpm: SerializableColor,
    pub accuracy: SerializableColor,
    pub duration: SerializableColor,
    pub stage_info: SerializableColor,

    // Typing interface colors
    pub typed_text: SerializableColor,
    pub current_cursor: SerializableColor,
    pub cursor_bg: SerializableColor,
    pub mistake_bg: SerializableColor,
    pub untyped_text: SerializableColor,
    pub comment_text: SerializableColor,

    // Context and secondary elements
    pub secondary: SerializableColor,
    pub muted: SerializableColor,

    // Status-specific colors
    pub completed: SerializableColor,
    pub skipped: SerializableColor,
    pub failed: SerializableColor,

    // Countdown colors
    pub countdown_3: SerializableColor,
    pub countdown_2: SerializableColor,
    pub countdown_1: SerializableColor,
    pub countdown_go: SerializableColor,

    // Progress bar colors
    pub progress_bar: SerializableColor,
    pub progress_bg: SerializableColor,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializableColor {
    Rgb { r: u8, g: u8, b: u8 },
    Name(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeFile {
    pub name: String,
    pub description: String,
    pub colors: HashMap<String, SerializableColor>,
}

impl From<Color> for SerializableColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb(r, g, b) => SerializableColor::Rgb { r, g, b },
            Color::Black => SerializableColor::Name("black".to_string()),
            Color::Red => SerializableColor::Name("red".to_string()),
            Color::Green => SerializableColor::Name("green".to_string()),
            Color::Yellow => SerializableColor::Name("yellow".to_string()),
            Color::Blue => SerializableColor::Name("blue".to_string()),
            Color::Magenta => SerializableColor::Name("magenta".to_string()),
            Color::Cyan => SerializableColor::Name("cyan".to_string()),
            Color::Gray => SerializableColor::Name("gray".to_string()),
            Color::DarkGray => SerializableColor::Name("dark_gray".to_string()),
            Color::LightRed => SerializableColor::Name("light_red".to_string()),
            Color::LightGreen => SerializableColor::Name("light_green".to_string()),
            Color::LightYellow => SerializableColor::Name("light_yellow".to_string()),
            Color::LightBlue => SerializableColor::Name("light_blue".to_string()),
            Color::LightMagenta => SerializableColor::Name("light_magenta".to_string()),
            Color::LightCyan => SerializableColor::Name("light_cyan".to_string()),
            Color::White => SerializableColor::Name("white".to_string()),
            _ => SerializableColor::Name("white".to_string()), // Default to white
        }
    }
}

impl From<SerializableColor> for Color {
    fn from(color: SerializableColor) -> Self {
        match color {
            SerializableColor::Rgb { r, g, b } => Color::Rgb(r, g, b),
            SerializableColor::Name(name) => match name.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" => Color::Magenta,
                "cyan" => Color::Cyan,
                "gray" | "grey" => Color::Gray,
                "dark_gray" | "dark_grey" | "darkgray" | "darkgrey" => Color::DarkGray,
                "light_red" | "lightred" => Color::LightRed,
                "light_green" | "lightgreen" => Color::LightGreen,
                "light_yellow" | "lightyellow" => Color::LightYellow,
                "light_blue" | "lightblue" => Color::LightBlue,
                "light_magenta" | "lightmagenta" => Color::LightMagenta,
                "light_cyan" | "lightcyan" => Color::LightCyan,
                "white" => Color::White,
                _ => {
                    // Try to parse as RGB hex if it starts with #
                    if name.starts_with('#') && name.len() == 7 {
                        if let (Ok(r), Ok(g), Ok(b)) = (
                            u8::from_str_radix(&name[1..3], 16),
                            u8::from_str_radix(&name[3..5], 16),
                            u8::from_str_radix(&name[5..7], 16),
                        ) {
                            return Color::Rgb(r, g, b);
                        }
                    }
                    // Default to white for unknown color names
                    Color::White
                }
            }
        }
    }
}

impl ColorScheme {
    pub fn from_theme_file(theme_file: &ThemeFile) -> Self {
        ColorScheme {
            // Primary colors for main UI elements
            border: theme_file.colors.get("border").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),
            title: theme_file.colors.get("title").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            text: theme_file.colors.get("text").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            background: theme_file.colors.get("background").cloned().unwrap_or(SerializableColor::Name("black".to_string())),

            // Status and feedback colors
            success: theme_file.colors.get("success").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            error: theme_file.colors.get("error").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            warning: theme_file.colors.get("warning").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            info: theme_file.colors.get("info").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),

            // Specific UI element colors
            back_key: theme_file.colors.get("back_key").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            action_key: theme_file.colors.get("action_key").cloned().unwrap_or(SerializableColor::Name("light_blue".to_string())),
            navigation_key: theme_file.colors.get("navigation_key").cloned().unwrap_or(SerializableColor::Name("light_blue".to_string())),
            highlight: theme_file.colors.get("highlight").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),

            // Metrics and performance colors
            score: theme_file.colors.get("score").cloned().unwrap_or(SerializableColor::Name("magenta".to_string())),
            cpm_wpm: theme_file.colors.get("cpm_wpm").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            accuracy: theme_file.colors.get("accuracy").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            duration: theme_file.colors.get("duration").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),
            stage_info: theme_file.colors.get("stage_info").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),

            // Typing interface colors
            typed_text: theme_file.colors.get("typed_text").cloned().unwrap_or(SerializableColor::Name("light_blue".to_string())),
            current_cursor: theme_file.colors.get("current_cursor").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            cursor_bg: theme_file.colors.get("cursor_bg").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),
            mistake_bg: theme_file.colors.get("mistake_bg").cloned().unwrap_or(SerializableColor::Name("red".to_string())),
            untyped_text: theme_file.colors.get("untyped_text").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            comment_text: theme_file.colors.get("comment_text").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),

            // Context and secondary elements
            secondary: theme_file.colors.get("secondary").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),
            muted: theme_file.colors.get("muted").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),

            // Status-specific colors
            completed: theme_file.colors.get("completed").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            skipped: theme_file.colors.get("skipped").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            failed: theme_file.colors.get("failed").cloned().unwrap_or(SerializableColor::Name("red".to_string())),

            // Countdown colors
            countdown_3: theme_file.colors.get("countdown_3").cloned().unwrap_or(SerializableColor::Name("magenta".to_string())),
            countdown_2: theme_file.colors.get("countdown_2").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),
            countdown_1: theme_file.colors.get("countdown_1").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            countdown_go: theme_file.colors.get("countdown_go").cloned().unwrap_or(SerializableColor::Name("green".to_string())),

            // Progress bar colors
            progress_bar: theme_file.colors.get("progress_bar").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),
            progress_bg: theme_file.colors.get("progress_bg").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),

            // Programming language colors
            lang_rust: theme_file.colors.get("lang_rust").cloned().unwrap_or(SerializableColor::Rgb { r: 222, g: 165, b: 132 }),
            lang_python: theme_file.colors.get("lang_python").cloned().unwrap_or(SerializableColor::Rgb { r: 255, g: 212, b: 59 }),
            lang_javascript: theme_file.colors.get("lang_javascript").cloned().unwrap_or(SerializableColor::Rgb { r: 240, g: 219, b: 79 }),
            lang_typescript: theme_file.colors.get("lang_typescript").cloned().unwrap_or(SerializableColor::Rgb { r: 49, g: 120, b: 198 }),
            lang_go: theme_file.colors.get("lang_go").cloned().unwrap_or(SerializableColor::Rgb { r: 0, g: 173, b: 181 }),
            lang_java: theme_file.colors.get("lang_java").cloned().unwrap_or(SerializableColor::Rgb { r: 237, g: 41, b: 57 }),
            lang_c: theme_file.colors.get("lang_c").cloned().unwrap_or(SerializableColor::Rgb { r: 85, g: 85, b: 85 }),
            lang_cpp: theme_file.colors.get("lang_cpp").cloned().unwrap_or(SerializableColor::Rgb { r: 0, g: 89, b: 156 }),
            lang_csharp: theme_file.colors.get("lang_csharp").cloned().unwrap_or(SerializableColor::Rgb { r: 239, g: 117, b: 27 }),
            lang_php: theme_file.colors.get("lang_php").cloned().unwrap_or(SerializableColor::Rgb { r: 119, g: 123, b: 180 }),
            lang_ruby: theme_file.colors.get("lang_ruby").cloned().unwrap_or(SerializableColor::Rgb { r: 204, g: 52, b: 45 }),
            lang_swift: theme_file.colors.get("lang_swift").cloned().unwrap_or(SerializableColor::Rgb { r: 250, g: 109, b: 63 }),
            lang_kotlin: theme_file.colors.get("lang_kotlin").cloned().unwrap_or(SerializableColor::Rgb { r: 124, g: 75, b: 255 }),
            lang_scala: theme_file.colors.get("lang_scala").cloned().unwrap_or(SerializableColor::Rgb { r: 220, g: 50, b: 47 }),
            lang_haskell: theme_file.colors.get("lang_haskell").cloned().unwrap_or(SerializableColor::Rgb { r: 94, g: 80, b: 134 }),
            lang_dart: theme_file.colors.get("lang_dart").cloned().unwrap_or(SerializableColor::Rgb { r: 0, g: 180, b: 240 }),
            lang_default: theme_file.colors.get("lang_default").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
        }
    }

    pub fn ascii_dark() -> Self {
        let ascii_dark_json = include_str!("../../assets/themes/ascii_dark.json");
        let theme_file: ThemeFile = serde_json::from_str(ascii_dark_json)
            .expect("Failed to parse ascii_dark.json");
        Self::from_theme_file(&theme_file)
    }

    pub fn ascii_light() -> Self {
        let ascii_light_json = include_str!("../../assets/themes/ascii_light.json");
        let theme_file: ThemeFile = serde_json::from_str(ascii_light_json)
            .expect("Failed to parse ascii_light.json");
        Self::from_theme_file(&theme_file)
    }


}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub current_theme: Theme,
    pub custom_themes: HashMap<String, ColorScheme>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            current_theme: Theme::default(),
            custom_themes: HashMap::new(),
        }
    }
}

pub struct ThemeManager {
    config: ThemeConfig,
    config_path: PathBuf,
}

impl ThemeManager {
    pub fn new() -> anyhow::Result<Self> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join(".gittype");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("theme.json");
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            ThemeConfig::default()
        };

        Ok(ThemeManager { config, config_path })
    }

    pub fn with_config_path(config_path: PathBuf) -> anyhow::Result<Self> {
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            ThemeConfig::default()
        };

        Ok(ThemeManager { config, config_path })
    }

    pub fn get_current_theme(&self) -> &Theme {
        &self.config.current_theme
    }

    pub fn set_theme(&mut self, theme: Theme) -> anyhow::Result<()> {
        self.config.current_theme = theme;
        self.save()
    }

    pub fn get_color_scheme(&self) -> ColorScheme {
        match &self.config.current_theme {
            Theme::AsciiDark => ColorScheme::ascii_dark(),
            Theme::AsciiLight => ColorScheme::ascii_light(),
            Theme::Custom(name) => {
                self.config.custom_themes
                    .get(name)
                    .cloned()
                    .unwrap_or_else(ColorScheme::ascii_dark)
            }
        }
    }

    pub fn add_custom_theme(&mut self, name: String, scheme: ColorScheme) -> anyhow::Result<()> {
        self.config.custom_themes.insert(name, scheme);
        self.save()
    }

    pub fn list_themes(&self) -> Vec<String> {
        let mut themes = vec![
            "ascii_dark".to_string(),
            "ascii_light".to_string(),
        ];
        themes.extend(self.config.custom_themes.keys().cloned());
        themes
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }
}