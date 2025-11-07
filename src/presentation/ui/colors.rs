use ratatui::style::Color;

use crate::domain::models::color_scheme::ColorScheme;

/// UI color scheme for gittype application
pub struct Colors {
    pub color_scheme: ColorScheme,
}

impl Colors {
    /// Create Colors from ColorScheme
    pub fn new(color_scheme: ColorScheme) -> Self {
        Self { color_scheme }
    }

    // Primary colors for main UI elements
    pub fn border(&self) -> Color {
        self.color_scheme.border.clone().into()
    }
    pub fn title(&self) -> Color {
        self.color_scheme.title.clone().into()
    }
    pub fn text(&self) -> Color {
        self.color_scheme.text.clone().into()
    }
    pub fn text_secondary(&self) -> Color {
        self.color_scheme.text_secondary.clone().into()
    }
    pub fn background(&self) -> Color {
        self.color_scheme.background.clone().into()
    }
    pub fn background_secondary(&self) -> Color {
        self.color_scheme.background_secondary.clone().into()
    }

    // Status and feedback colors
    pub fn success(&self) -> Color {
        self.color_scheme.status_success.clone().into()
    }
    pub fn info(&self) -> Color {
        self.color_scheme.status_info.clone().into()
    }
    pub fn error(&self) -> Color {
        self.color_scheme.status_error.clone().into()
    }
    pub fn warning(&self) -> Color {
        self.color_scheme.status_warning.clone().into()
    }

    // Specific UI element colors
    pub fn key_action(&self) -> Color {
        self.color_scheme.key_action.clone().into()
    }
    pub fn key_navigation(&self) -> Color {
        self.color_scheme.key_navigation.clone().into()
    }
    pub fn key_back(&self) -> Color {
        self.color_scheme.key_back.clone().into()
    }

    // Metrics and performance colors
    pub fn score(&self) -> Color {
        self.color_scheme.metrics_score.clone().into()
    }
    pub fn cpm_wpm(&self) -> Color {
        self.color_scheme.metrics_cpm_wpm.clone().into()
    }
    pub fn accuracy(&self) -> Color {
        self.color_scheme.metrics_accuracy.clone().into()
    }
    pub fn duration(&self) -> Color {
        self.color_scheme.metrics_duration.clone().into()
    }
    pub fn stage_info(&self) -> Color {
        self.color_scheme.metrics_stage_info.clone().into()
    }

    // Typing interface colors
    pub fn typed_text(&self) -> Color {
        self.color_scheme.typing_typed_text.clone().into()
    }
    pub fn current_cursor(&self) -> Color {
        self.color_scheme.typing_cursor_fg.clone().into()
    }
    pub fn cursor_bg(&self) -> Color {
        self.color_scheme.typing_cursor_bg.clone().into()
    }
    pub fn mistake_bg(&self) -> Color {
        self.color_scheme.typing_mistake_bg.clone().into()
    }
    pub fn untyped_text(&self) -> Color {
        self.color_scheme.typing_untyped_text.clone().into()
    }

    // Countdown colors - using status colors in sequence
    pub fn countdown_3(&self) -> Color {
        self.success()
    }
    pub fn countdown_2(&self) -> Color {
        self.info()
    }
    pub fn countdown_1(&self) -> Color {
        self.warning()
    }
    pub fn countdown_go(&self) -> Color {
        self.error()
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

    // Default colors for static contexts (e.g., rank messages)
    // These use the default theme colors
    pub fn default_text() -> Color {
        Color::Rgb(200, 200, 200) // Light gray for text
    }
    pub fn default_success() -> Color {
        Color::Rgb(80, 250, 123) // Green for success
    }
    pub fn default_info() -> Color {
        Color::Rgb(139, 233, 253) // Cyan for info
    }
    pub fn default_error() -> Color {
        Color::Rgb(255, 85, 85) // Red for error
    }
    pub fn default_warning() -> Color {
        Color::Rgb(241, 250, 140) // Yellow for warning
    }
    pub fn default_score() -> Color {
        Color::Rgb(255, 184, 108) // Orange for score
    }
    pub fn default_border() -> Color {
        Color::Rgb(98, 114, 164) // Purple-ish for border
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
