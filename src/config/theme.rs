use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Ascii,
    Custom(String),
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub typing_current_cursor: SerializableColor,
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
            text_secondary: theme_file.colors.get("text_secondary").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),
            background: theme_file.colors.get("background").cloned().unwrap_or(SerializableColor::Name("black".to_string())),
            background_secondary: theme_file.colors.get("background_secondary").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),

            // Status and feedback colors
            status_success: theme_file.colors.get("status_success").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            status_info: theme_file.colors.get("status_info").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),
            status_warning: theme_file.colors.get("status_warning").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),            
            status_error: theme_file.colors.get("status_error").cloned().unwrap_or(SerializableColor::Name("red".to_string())),

            // Specific UI element colors
            key_action: theme_file.colors.get("key_action").cloned().unwrap_or(SerializableColor::Name("light_blue".to_string())),
            key_navigation: theme_file.colors.get("key_navigation").cloned().unwrap_or(SerializableColor::Name("light_blue".to_string())),
            key_back: theme_file.colors.get("key_back").cloned().unwrap_or(SerializableColor::Name("red".to_string())),

            // Metrics and performance colors
            metrics_score: theme_file.colors.get("metrics_score").cloned().unwrap_or(SerializableColor::Name("magenta".to_string())),
            metrics_cpm_wpm: theme_file.colors.get("metrics_cpm_wpm").cloned().unwrap_or(SerializableColor::Name("green".to_string())),
            metrics_accuracy: theme_file.colors.get("metrics_accuracy").cloned().unwrap_or(SerializableColor::Name("yellow".to_string())),
            metrics_duration: theme_file.colors.get("metrics_duration").cloned().unwrap_or(SerializableColor::Name("cyan".to_string())),
            metrics_stage_info: theme_file.colors.get("metrics_stage_info").cloned().unwrap_or(SerializableColor::Name("blue".to_string())),

            // Typing interface colors
            typing_untyped_text: theme_file.colors.get("typing_untyped_text").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            typing_typed_text: theme_file.colors.get("typing_typed_text").cloned().unwrap_or(SerializableColor::Name("light_blue".to_string())),
            typing_current_cursor: theme_file.colors.get("typing_cursor_fg").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
            typing_cursor_bg: theme_file.colors.get("typing_cursor_bg").cloned().unwrap_or(SerializableColor::Name("dark_gray".to_string())),
            typing_mistake_bg: theme_file.colors.get("typing_mistake_bg").cloned().unwrap_or(SerializableColor::Name("red".to_string())),

            // Programming language colors (loaded from separate files)
            ..Self::load_language_colors(&theme_file.name)
        }
    }

    fn load_language_colors(theme_name: &str) -> Self {
        // Determine which language color file to use based on theme name
        let lang_json = match theme_name.to_lowercase().as_str() {
            "dark" => include_str!("../../assets/themes/lang_dark.json"),
            "light" => include_str!("../../assets/themes/lang_light.json"),
            "ascii" => include_str!("../../assets/themes/lang_ascii.json"),
            _ => include_str!("../../assets/themes/lang_dark.json"), // default
        };

        let lang_colors: HashMap<String, SerializableColor> = serde_json::from_str(lang_json)
            .expect("Failed to parse language colors");

        ColorScheme {
            // Dummy values - only language colors will be used
            border: SerializableColor::Name("black".to_string()),
            title: SerializableColor::Name("black".to_string()),
            text: SerializableColor::Name("black".to_string()),
            text_secondary: SerializableColor::Name("black".to_string()),
            background: SerializableColor::Name("black".to_string()),
            background_secondary: SerializableColor::Name("black".to_string()),
            status_success: SerializableColor::Name("black".to_string()),
            status_info: SerializableColor::Name("black".to_string()),
            status_error: SerializableColor::Name("black".to_string()),
            status_warning: SerializableColor::Name("black".to_string()),
            key_action: SerializableColor::Name("black".to_string()),
            key_navigation: SerializableColor::Name("black".to_string()),
            key_back: SerializableColor::Name("black".to_string()),
            metrics_score: SerializableColor::Name("black".to_string()),
            metrics_cpm_wpm: SerializableColor::Name("black".to_string()),
            metrics_accuracy: SerializableColor::Name("black".to_string()),
            metrics_duration: SerializableColor::Name("black".to_string()),
            metrics_stage_info: SerializableColor::Name("black".to_string()),
            typing_typed_text: SerializableColor::Name("black".to_string()),
            typing_current_cursor: SerializableColor::Name("black".to_string()),
            typing_cursor_bg: SerializableColor::Name("black".to_string()),
            typing_mistake_bg: SerializableColor::Name("black".to_string()),
            typing_untyped_text: SerializableColor::Name("black".to_string()),

            // Language colors
            lang_rust: lang_colors.get("lang_rust").cloned().unwrap_or(SerializableColor::Rgb { r: 222, g: 165, b: 132 }),
            lang_python: lang_colors.get("lang_python").cloned().unwrap_or(SerializableColor::Rgb { r: 255, g: 212, b: 59 }),
            lang_javascript: lang_colors.get("lang_javascript").cloned().unwrap_or(SerializableColor::Rgb { r: 240, g: 219, b: 79 }),
            lang_typescript: lang_colors.get("lang_typescript").cloned().unwrap_or(SerializableColor::Rgb { r: 49, g: 120, b: 198 }),
            lang_go: lang_colors.get("lang_go").cloned().unwrap_or(SerializableColor::Rgb { r: 0, g: 173, b: 181 }),
            lang_java: lang_colors.get("lang_java").cloned().unwrap_or(SerializableColor::Rgb { r: 237, g: 41, b: 57 }),
            lang_c: lang_colors.get("lang_c").cloned().unwrap_or(SerializableColor::Rgb { r: 85, g: 85, b: 85 }),
            lang_cpp: lang_colors.get("lang_cpp").cloned().unwrap_or(SerializableColor::Rgb { r: 0, g: 89, b: 156 }),
            lang_csharp: lang_colors.get("lang_csharp").cloned().unwrap_or(SerializableColor::Rgb { r: 239, g: 117, b: 27 }),
            lang_php: lang_colors.get("lang_php").cloned().unwrap_or(SerializableColor::Rgb { r: 119, g: 123, b: 180 }),
            lang_ruby: lang_colors.get("lang_ruby").cloned().unwrap_or(SerializableColor::Rgb { r: 204, g: 52, b: 45 }),
            lang_swift: lang_colors.get("lang_swift").cloned().unwrap_or(SerializableColor::Rgb { r: 250, g: 109, b: 63 }),
            lang_kotlin: lang_colors.get("lang_kotlin").cloned().unwrap_or(SerializableColor::Rgb { r: 124, g: 75, b: 255 }),
            lang_scala: lang_colors.get("lang_scala").cloned().unwrap_or(SerializableColor::Rgb { r: 220, g: 50, b: 47 }),
            lang_haskell: lang_colors.get("lang_haskell").cloned().unwrap_or(SerializableColor::Rgb { r: 94, g: 80, b: 134 }),
            lang_dart: lang_colors.get("lang_dart").cloned().unwrap_or(SerializableColor::Rgb { r: 0, g: 180, b: 240 }),
            lang_default: lang_colors.get("lang_default").cloned().unwrap_or(SerializableColor::Name("white".to_string())),
        }
    }

    pub fn dark() -> Self {
        let dark_json = include_str!("../../assets/themes/dark.json");
        let theme_file: ThemeFile = serde_json::from_str(dark_json)
            .expect("Failed to parse dark.json");
        Self::from_theme_file(&theme_file)
    }

    pub fn light() -> Self {
        let light_json = include_str!("../../assets/themes/light.json");
        let theme_file: ThemeFile = serde_json::from_str(light_json)
            .expect("Failed to parse light.json");
        Self::from_theme_file(&theme_file)
    }

    pub fn ascii() -> Self {
        let ascii_json = include_str!("../../assets/themes/ascii.json");
        let theme_file: ThemeFile = serde_json::from_str(ascii_json)
            .expect("Failed to parse ascii.json");
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
    cached_color_scheme: ColorScheme,
}

impl ThemeManager {
    pub fn new() -> anyhow::Result<Self> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join(".gittype");

        fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("theme.json");
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            ThemeConfig::default()
        };

        let cached_color_scheme = Self::get_color_scheme_for_theme(&config.current_theme, &config);

        Ok(ThemeManager { config, config_path, cached_color_scheme })
    }

    pub fn with_config_path(config_path: PathBuf) -> anyhow::Result<Self> {
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            ThemeConfig::default()
        };

        let cached_color_scheme = Self::get_color_scheme_for_theme(&config.current_theme, &config);

        Ok(ThemeManager { config, config_path, cached_color_scheme })
    }

    pub fn get_current_theme(&self) -> &Theme {
        &self.config.current_theme
    }

    pub fn set_theme(&mut self, theme: Theme) -> anyhow::Result<()> {
        self.config.current_theme = theme.clone();
        self.cached_color_scheme = Self::get_color_scheme_for_theme(&theme, &self.config);
        self.save()
    }

    pub fn get_color_scheme(&self) -> &ColorScheme {
        &self.cached_color_scheme
    }

    fn get_color_scheme_for_theme(theme: &Theme, config: &ThemeConfig) -> ColorScheme {
        match theme {
            Theme::Dark => ColorScheme::dark(),
            Theme::Light => ColorScheme::light(),
            Theme::Ascii => ColorScheme::ascii(),
            Theme::Custom(name) => {
                config.custom_themes
                    .get(name)
                    .cloned()
                    .unwrap_or_else(ColorScheme::dark)
            }
        }
    }

    /// Get list of available theme names for in-game selection
    pub fn get_available_themes(&self) -> Vec<String> {
        let mut themes = vec![
            "dark".to_string(),
            "light".to_string(),
            "ascii".to_string(),
        ];
        themes.extend(self.config.custom_themes.keys().cloned());
        themes
    }

    /// Set theme by name (for in-game use)
    pub fn set_theme_by_name(&mut self, theme_name: &str) -> anyhow::Result<()> {
        let theme = match theme_name.to_lowercase().as_str() {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "ascii" => Theme::Ascii,
            name => {
                if self.config.custom_themes.contains_key(name) {
                    Theme::Custom(name.to_string())
                } else {
                    return Err(anyhow::anyhow!("Unknown theme: {}", name));
                }
            }
        };
        self.set_theme(theme)
    }

    /// Get current theme name as string
    pub fn get_current_theme_name(&self) -> String {
        match &self.config.current_theme {
            Theme::Dark => "dark".to_string(),
            Theme::Light => "light".to_string(),
            Theme::Ascii => "ascii".to_string(),
            Theme::Custom(name) => name.clone(),
        }
    }

    pub fn add_custom_theme(&mut self, name: String, scheme: ColorScheme) -> anyhow::Result<()> {
        self.config.custom_themes.insert(name, scheme);
        self.save()
    }

    pub fn list_themes(&self) -> Vec<String> {
        let mut themes = vec![
            "dark".to_string(),
            "light".to_string(),
            "ascii".to_string(),
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
