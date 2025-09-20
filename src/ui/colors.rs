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
            .map(|tm| tm.get_color_scheme())
            .unwrap_or_else(ColorScheme::ascii_dark)
    }

    // Primary colors for main UI elements
    pub fn border() -> Color { Self::get_color_scheme().border.into() }
    pub fn title() -> Color { Self::get_color_scheme().title.into() }
    pub fn text() -> Color { Self::get_color_scheme().text.into() }
    pub fn background() -> Color { Self::get_color_scheme().background.into() }

    // Status and feedback colors
    pub fn success() -> Color { Self::get_color_scheme().success.into() }
    pub fn error() -> Color { Self::get_color_scheme().error.into() }
    pub fn warning() -> Color { Self::get_color_scheme().warning.into() }
    pub fn info() -> Color { Self::get_color_scheme().info.into() }

    // Specific UI element colors
    pub fn back_key() -> Color { Self::get_color_scheme().back_key.into() }
    pub fn action_key() -> Color { Self::get_color_scheme().action_key.into() }
    pub fn navigation_key() -> Color { Self::get_color_scheme().navigation_key.into() }
    pub fn highlight() -> Color { Self::get_color_scheme().highlight.into() }

    // Metrics and performance colors
    pub fn score() -> Color { Self::get_color_scheme().score.into() }
    pub fn cpm_wpm() -> Color { Self::get_color_scheme().cpm_wpm.into() }
    pub fn accuracy() -> Color { Self::get_color_scheme().accuracy.into() }
    pub fn duration() -> Color { Self::get_color_scheme().duration.into() }
    pub fn stage_info() -> Color { Self::get_color_scheme().stage_info.into() }

    // Typing interface colors
    pub fn typed_text() -> Color { Self::get_color_scheme().typed_text.into() }
    pub fn current_cursor() -> Color { Self::get_color_scheme().current_cursor.into() }
    pub fn cursor_bg() -> Color { Self::get_color_scheme().cursor_bg.into() }
    pub fn mistake_bg() -> Color { Self::get_color_scheme().mistake_bg.into() }
    pub fn untyped_text() -> Color { Self::get_color_scheme().untyped_text.into() }
    pub fn comment_text() -> Color { Self::get_color_scheme().comment_text.into() }

    // Context and secondary elements
    pub fn secondary() -> Color { Self::get_color_scheme().secondary.into() }
    pub fn muted() -> Color { Self::get_color_scheme().muted.into() }

    // Status-specific colors
    pub fn completed() -> Color { Self::get_color_scheme().completed.into() }
    pub fn skipped() -> Color { Self::get_color_scheme().skipped.into() }
    pub fn failed() -> Color { Self::get_color_scheme().failed.into() }

    // Countdown colors
    pub fn countdown_3() -> Color { Self::get_color_scheme().countdown_3.into() }
    pub fn countdown_2() -> Color { Self::get_color_scheme().countdown_2.into() }
    pub fn countdown_1() -> Color { Self::get_color_scheme().countdown_1.into() }
    pub fn countdown_go() -> Color { Self::get_color_scheme().countdown_go.into() }

    // Progress bar colors
    pub fn progress_bar() -> Color { Self::get_color_scheme().progress_bar.into() }
    pub fn progress_bg() -> Color { Self::get_color_scheme().progress_bg.into() }

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

    /// Get list of available themes
    pub fn list_themes() -> Vec<String> {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_ref()
            .map(|tm| tm.list_themes())
            .unwrap_or_else(|| vec!["ascii_dark".to_string()])
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
