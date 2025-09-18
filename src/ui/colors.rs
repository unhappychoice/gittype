use ratatui::style::Color;

/// UI color scheme for gittype application
pub struct Colors;

impl Colors {
    // Primary colors for main UI elements
    pub const BORDER: Color = Color::Blue;
    pub const TITLE: Color = Color::White;
    pub const TEXT: Color = Color::White;
    pub const BACKGROUND: Color = Color::Black;

    // Status and feedback colors
    pub const SUCCESS: Color = Color::Green;
    pub const ERROR: Color = Color::Red;
    pub const WARNING: Color = Color::Yellow;
    pub const INFO: Color = Color::Cyan;

    // Specific UI element colors
    pub const BACK_KEY: Color = Color::Red;
    pub const ACTION_KEY: Color = Color::LightBlue;
    pub const NAVIGATION_KEY: Color = Color::LightBlue;
    pub const HIGHLIGHT: Color = Color::Cyan;

    // Metrics and performance colors (from session_detail_screen.rs)
    pub const SCORE: Color = Color::Magenta;
    pub const CPM_WPM: Color = Color::Green;
    pub const ACCURACY: Color = Color::Yellow;
    pub const DURATION: Color = Color::Cyan;
    pub const STAGE_INFO: Color = Color::Blue;

    // Typing interface colors
    pub const TYPED_TEXT: Color = Color::LightBlue;
    pub const CURRENT_CURSOR: Color = Color::White;
    pub const CURSOR_BG: Color = Color::DarkGray;
    pub const MISTAKE_BG: Color = Color::Red;
    pub const UNTYPED_TEXT: Color = Color::White;
    pub const COMMENT_TEXT: Color = Color::DarkGray;

    // Context and secondary elements
    pub const SECONDARY: Color = Color::DarkGray;
    pub const MUTED: Color = Color::DarkGray;

    // Status-specific colors
    pub const COMPLETED: Color = Color::Green;
    pub const SKIPPED: Color = Color::Yellow;
    pub const FAILED: Color = Color::Red;

    // Countdown colors
    pub const COUNTDOWN_3: Color = Color::Magenta;
    pub const COUNTDOWN_2: Color = Color::Cyan;
    pub const COUNTDOWN_1: Color = Color::Yellow;
    pub const COUNTDOWN_GO: Color = Color::Green;

    // Progress bar colors
    pub const PROGRESS_BAR: Color = Color::Cyan;
    pub const PROGRESS_BG: Color = Color::DarkGray;

    // Programming language colors - distinct and accessible
    pub const LANG_RUST: Color = Color::Rgb(222, 165, 132); // Rust orange
    pub const LANG_PYTHON: Color = Color::Rgb(255, 212, 59); // Python yellow
    pub const LANG_JAVASCRIPT: Color = Color::Rgb(240, 219, 79); // JS yellow
    pub const LANG_TYPESCRIPT: Color = Color::Rgb(49, 120, 198); // TS blue
    pub const LANG_GO: Color = Color::Rgb(0, 173, 181); // Go cyan
    pub const LANG_JAVA: Color = Color::Rgb(237, 41, 57); // Java red
    pub const LANG_C: Color = Color::Rgb(85, 85, 85); // C gray
    pub const LANG_CPP: Color = Color::Rgb(0, 89, 156); // C++ blue
    pub const LANG_CSHARP: Color = Color::Rgb(239, 117, 27); // C# orange
    pub const LANG_PHP: Color = Color::Rgb(119, 123, 180); // PHP purple
    pub const LANG_RUBY: Color = Color::Rgb(204, 52, 45); // Ruby red
    pub const LANG_SWIFT: Color = Color::Rgb(250, 109, 63); // Swift orange
    pub const LANG_KOTLIN: Color = Color::Rgb(124, 75, 255); // Kotlin purple
    pub const LANG_SCALA: Color = Color::Rgb(220, 50, 47); // Scala red
    pub const LANG_HASKELL: Color = Color::Rgb(94, 80, 134); // Haskell purple
    pub const LANG_DART: Color = Color::Rgb(0, 180, 240); // Dart blue
    pub const LANG_DEFAULT: Color = Color::White; // Default for unknown languages
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
}
