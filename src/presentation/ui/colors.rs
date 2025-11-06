use crate::domain::models::color_scheme::ColorScheme;
use crate::domain::services::theme_service::ThemeService;
use ratatui::style::Color;

/// UI color scheme for gittype application
pub struct Colors;

impl Colors {
    /// Get the current color scheme
    fn get_color_scheme() -> ColorScheme {
        ThemeService::get_current_color_scheme()
    }

    // Primary colors for main UI elements
    pub fn border() -> Color {
        Self::get_color_scheme().border.into()
    }
    pub fn title() -> Color {
        Self::get_color_scheme().title.into()
    }
    pub fn text() -> Color {
        Self::get_color_scheme().text.into()
    }
    pub fn text_secondary() -> Color {
        Self::get_color_scheme().text_secondary.into()
    }
    pub fn background() -> Color {
        Self::get_color_scheme().background.into()
    }
    pub fn background_secondary() -> Color {
        Self::get_color_scheme().background_secondary.into()
    }

    // Status and feedback colors
    pub fn success() -> Color {
        Self::get_color_scheme().status_success.into()
    }
    pub fn info() -> Color {
        Self::get_color_scheme().status_info.into()
    }
    pub fn error() -> Color {
        Self::get_color_scheme().status_error.into()
    }
    pub fn warning() -> Color {
        Self::get_color_scheme().status_warning.into()
    }

    // Specific UI element colors
    pub fn key_action() -> Color {
        Self::get_color_scheme().key_action.into()
    }
    pub fn key_navigation() -> Color {
        Self::get_color_scheme().key_navigation.into()
    }
    pub fn key_back() -> Color {
        Self::get_color_scheme().key_back.into()
    }

    // Metrics and performance colors
    pub fn score() -> Color {
        Self::get_color_scheme().metrics_score.into()
    }
    pub fn cpm_wpm() -> Color {
        Self::get_color_scheme().metrics_cpm_wpm.into()
    }
    pub fn accuracy() -> Color {
        Self::get_color_scheme().metrics_accuracy.into()
    }
    pub fn duration() -> Color {
        Self::get_color_scheme().metrics_duration.into()
    }
    pub fn stage_info() -> Color {
        Self::get_color_scheme().metrics_stage_info.into()
    }

    // Typing interface colors
    pub fn typed_text() -> Color {
        Self::get_color_scheme().typing_typed_text.into()
    }
    pub fn current_cursor() -> Color {
        Self::get_color_scheme().typing_cursor_fg.into()
    }
    pub fn cursor_bg() -> Color {
        Self::get_color_scheme().typing_cursor_bg.into()
    }
    pub fn mistake_bg() -> Color {
        Self::get_color_scheme().typing_mistake_bg.into()
    }
    pub fn untyped_text() -> Color {
        Self::get_color_scheme().typing_untyped_text.into()
    }

    // Countdown colors - using status colors in sequence
    pub fn countdown_3() -> Color {
        Self::success()
    }
    pub fn countdown_2() -> Color {
        Self::info()
    }
    pub fn countdown_1() -> Color {
        Self::warning()
    }
    pub fn countdown_go() -> Color {
        Self::error()
    }

    // Programming language colors
    pub fn lang_rust() -> Color {
        Self::get_color_scheme().lang_rust.into()
    }
    pub fn lang_python() -> Color {
        Self::get_color_scheme().lang_python.into()
    }
    pub fn lang_javascript() -> Color {
        Self::get_color_scheme().lang_javascript.into()
    }
    pub fn lang_typescript() -> Color {
        Self::get_color_scheme().lang_typescript.into()
    }
    pub fn lang_go() -> Color {
        Self::get_color_scheme().lang_go.into()
    }
    pub fn lang_java() -> Color {
        Self::get_color_scheme().lang_java.into()
    }
    pub fn lang_c() -> Color {
        Self::get_color_scheme().lang_c.into()
    }
    pub fn lang_cpp() -> Color {
        Self::get_color_scheme().lang_cpp.into()
    }
    pub fn lang_clojure() -> Color {
        Self::get_color_scheme().lang_clojure.into()
    }
    pub fn lang_csharp() -> Color {
        Self::get_color_scheme().lang_csharp.into()
    }
    pub fn lang_php() -> Color {
        Self::get_color_scheme().lang_php.into()
    }
    pub fn lang_ruby() -> Color {
        Self::get_color_scheme().lang_ruby.into()
    }
    pub fn lang_swift() -> Color {
        Self::get_color_scheme().lang_swift.into()
    }
    pub fn lang_kotlin() -> Color {
        Self::get_color_scheme().lang_kotlin.into()
    }
    pub fn lang_scala() -> Color {
        Self::get_color_scheme().lang_scala.into()
    }
    pub fn lang_haskell() -> Color {
        Self::get_color_scheme().lang_haskell.into()
    }
    pub fn lang_dart() -> Color {
        Self::get_color_scheme().lang_dart.into()
    }
    pub fn lang_zig() -> Color {
        Self::get_color_scheme().lang_zig.into()
    }
    pub fn lang_default() -> Color {
        Self::get_color_scheme().lang_default.into()
    }

    // Rank tier colors (from rank_colors.rs)
    pub fn tier_beginner() -> Color {
        Color::Rgb(95, 175, 255) // #5fafff - light blue
    }
    pub fn tier_intermediate() -> Color {
        Color::Rgb(0, 215, 255) // #00d7ff - cyan
    }
    pub fn tier_advanced() -> Color {
        Color::Rgb(95, 175, 135) // #5faf87 - green
    }
    pub fn tier_expert() -> Color {
        Color::Rgb(255, 175, 0) // #ffaf00 - orange/gold
    }
    pub fn tier_legendary() -> Color {
        Color::Rgb(215, 95, 95) // #d75f5f - red
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
