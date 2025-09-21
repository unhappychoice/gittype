use ratatui::style::Color;
use crate::config::{ColorScheme, ThemeManager};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static THEME_MANAGER: Lazy<Mutex<Option<ThemeManager>>> = Lazy::new(|| Mutex::new(None));

/// UI color scheme for gittype application
pub struct Colors;

impl Colors {
    /// Initialize the theme manager with optional config path
    pub fn init_theme_manager(config_path: Option<std::path::PathBuf>) -> anyhow::Result<()> {
        let mut theme_manager = THEME_MANAGER.lock().unwrap();
        *theme_manager = Some(if let Some(path) = config_path {
            ThemeManager::with_config_path(path)?
        } else {
            ThemeManager::new()?
        });
        Ok(())
    }

    /// Get the current color scheme
    fn get_color_scheme() -> ColorScheme {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_ref()
            .map(|tm| tm.get_color_scheme().clone())
            .unwrap_or_else(|| ColorScheme::default_theme(&crate::config::ColorMode::Dark))
    }

    // Primary colors for main UI elements
    pub fn border() -> Color { Self::get_color_scheme().border.into() }
    pub fn title() -> Color { Self::get_color_scheme().title.into() }
    pub fn text() -> Color { Self::get_color_scheme().text.into() }
    pub fn text_secondary() -> Color { Self::get_color_scheme().text_secondary.into() }
    pub fn background() -> Color { Self::get_color_scheme().background.into() }
    pub fn background_secondary() -> Color { Self::get_color_scheme().background_secondary.into() }

    // Status and feedback colors
    pub fn success() -> Color { Self::get_color_scheme().status_success.into() }
    pub fn info() -> Color { Self::get_color_scheme().status_info.into() }
    pub fn error() -> Color { Self::get_color_scheme().status_error.into() }
    pub fn warning() -> Color { Self::get_color_scheme().status_warning.into() }

    // Specific UI element colors
    pub fn key_action() -> Color { Self::get_color_scheme().key_action.into() }
    pub fn key_navigation() -> Color { Self::get_color_scheme().key_navigation.into() }
    pub fn key_back() -> Color { Self::get_color_scheme().key_back.into() }

    // Metrics and performance colors
    pub fn score() -> Color { Self::get_color_scheme().metrics_score.into() }
    pub fn cpm_wpm() -> Color { Self::get_color_scheme().metrics_cpm_wpm.into() }
    pub fn accuracy() -> Color { Self::get_color_scheme().metrics_accuracy.into() }
    pub fn duration() -> Color { Self::get_color_scheme().metrics_duration.into() }
    pub fn stage_info() -> Color { Self::get_color_scheme().metrics_stage_info.into() }

    // Typing interface colors
    pub fn typed_text() -> Color { Self::get_color_scheme().typing_typed_text.into() }
    pub fn current_cursor() -> Color { Self::get_color_scheme().typing_current_cursor.into() }
    pub fn cursor_bg() -> Color { Self::get_color_scheme().typing_cursor_bg.into() }
    pub fn mistake_bg() -> Color { Self::get_color_scheme().typing_mistake_bg.into() }
    pub fn untyped_text() -> Color { Self::get_color_scheme().typing_untyped_text.into() }

    // Countdown colors - using status colors in sequence
    pub fn countdown_3() -> Color { Self::success() }
    pub fn countdown_2() -> Color { Self::info() }
    pub fn countdown_1() -> Color { Self::warning() }
    pub fn countdown_go() -> Color { Self::error() }

    // Programming language colors
    pub fn lang_rust() -> Color { Self::get_color_scheme().lang_rust.into() }
    pub fn lang_python() -> Color { Self::get_color_scheme().lang_python.into() }
    pub fn lang_javascript() -> Color { Self::get_color_scheme().lang_javascript.into() }
    pub fn lang_typescript() -> Color { Self::get_color_scheme().lang_typescript.into() }
    pub fn lang_go() -> Color { Self::get_color_scheme().lang_go.into() }
    pub fn lang_java() -> Color { Self::get_color_scheme().lang_java.into() }
    pub fn lang_c() -> Color { Self::get_color_scheme().lang_c.into() }
    pub fn lang_cpp() -> Color { Self::get_color_scheme().lang_cpp.into() }
    pub fn lang_csharp() -> Color { Self::get_color_scheme().lang_csharp.into() }
    pub fn lang_php() -> Color { Self::get_color_scheme().lang_php.into() }
    pub fn lang_ruby() -> Color { Self::get_color_scheme().lang_ruby.into() }
    pub fn lang_swift() -> Color { Self::get_color_scheme().lang_swift.into() }
    pub fn lang_kotlin() -> Color { Self::get_color_scheme().lang_kotlin.into() }
    pub fn lang_scala() -> Color { Self::get_color_scheme().lang_scala.into() }
    pub fn lang_haskell() -> Color { Self::get_color_scheme().lang_haskell.into() }
    pub fn lang_dart() -> Color { Self::get_color_scheme().lang_dart.into() }
    pub fn lang_default() -> Color { Self::get_color_scheme().lang_default.into() }

    /// Set the current theme
    pub fn set_theme(theme: crate::config::Theme) -> anyhow::Result<()> {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Theme manager not initialized"))?
            .set_theme(theme)
    }

    /// Set theme by name (for in-game use)
    pub fn set_theme_by_name(theme_name: &str) -> anyhow::Result<()> {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Theme manager not initialized"))?
            .set_theme_by_name(theme_name)
    }

    /// Get current theme name
    pub fn get_current_theme_name() -> String {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_ref()
            .map(|tm| tm.get_current_theme_name())
            .unwrap_or_else(|| "default".to_string())
    }

    /// Get list of available themes
    pub fn list_themes() -> Vec<String> {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_ref()
            .map(|tm| tm.get_available_themes())
            .unwrap_or_else(|| vec!["default".to_string()])
    }

    /// Get current color mode
    pub fn current_color_mode() -> String {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_ref()
            .map(|tm| match tm.get_current_color_mode() {
                crate::config::ColorMode::Dark => "dark".to_string(),
                crate::config::ColorMode::Light => "light".to_string(),
            })
            .unwrap_or_else(|| "dark".to_string())
    }

    /// Toggle color mode
    pub fn toggle_color_mode() -> Result<(), String> {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_mut()
            .ok_or("Theme manager not initialized".to_string())?
            .toggle_color_mode()
            .map_err(|e| e.to_string())
    }

    /// Set color mode
    pub fn set_color_mode(mode: &str) -> Result<(), String> {
        let color_mode = match mode.to_lowercase().as_str() {
            "dark" => crate::config::ColorMode::Dark,
            "light" => crate::config::ColorMode::Light,
            _ => return Err(format!("Invalid color mode: {}. Use 'dark' or 'light'", mode)),
        };

        THEME_MANAGER
            .lock()
            .unwrap()
            .as_mut()
            .ok_or("Theme manager not initialized".to_string())?
            .set_color_mode(color_mode)
            .map_err(|e| e.to_string())
    }
}

impl Colors {
    /// Convert ratatui Color to crossterm Color
    pub fn to_crossterm(color: Color) -> crossterm::style::Color {
        match color {
            Color::Reset => crossterm::style::Color::Reset,
            Color::Black => crossterm::style::Color::Black,
            Color::Red => crossterm::style::Color::Red,
            Color::Green => crossterm::style::Color::Green,
            Color::Yellow => crossterm::style::Color::Yellow,
            Color::Blue => crossterm::style::Color::Blue,
            Color::Magenta => crossterm::style::Color::Magenta,
            Color::Cyan => crossterm::style::Color::Cyan,
            Color::Gray => crossterm::style::Color::Grey,
            Color::DarkGray => crossterm::style::Color::DarkGrey,
            Color::LightRed => crossterm::style::Color::DarkRed,
            Color::LightGreen => crossterm::style::Color::DarkGreen,
            Color::LightYellow => crossterm::style::Color::DarkYellow,
            Color::LightBlue => crossterm::style::Color::DarkBlue,
            Color::LightMagenta => crossterm::style::Color::DarkMagenta,
            Color::LightCyan => crossterm::style::Color::DarkCyan,
            Color::White => crossterm::style::Color::White,
            Color::Rgb(r, g, b) => crossterm::style::Color::Rgb { r, g, b },
            Color::Indexed(i) => crossterm::style::Color::AnsiValue(i),
        }
    }

    /// Convert crossterm Color to ratatui Color
    pub fn from_crossterm(color: crossterm::style::Color) -> Color {
        match color {
            crossterm::style::Color::Reset => Color::Reset,
            crossterm::style::Color::Black => Color::Black,
            crossterm::style::Color::DarkGrey => Color::DarkGray,
            crossterm::style::Color::Red => Color::Red,
            crossterm::style::Color::DarkRed => Color::LightRed,
            crossterm::style::Color::Green => Color::Green,
            crossterm::style::Color::DarkGreen => Color::LightGreen,
            crossterm::style::Color::Yellow => Color::Yellow,
            crossterm::style::Color::DarkYellow => Color::LightYellow,
            crossterm::style::Color::Blue => Color::Blue,
            crossterm::style::Color::DarkBlue => Color::LightBlue,
            crossterm::style::Color::Magenta => Color::Magenta,
            crossterm::style::Color::DarkMagenta => Color::LightMagenta,
            crossterm::style::Color::Cyan => Color::Cyan,
            crossterm::style::Color::DarkCyan => Color::LightCyan,
            crossterm::style::Color::White => Color::White,
            crossterm::style::Color::Grey => Color::Gray,
            crossterm::style::Color::Rgb { r, g, b } => Color::Rgb(r, g, b),
            crossterm::style::Color::AnsiValue(i) => Color::Indexed(i),
        }
    }
}
