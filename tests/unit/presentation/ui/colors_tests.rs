use crossterm::style::Color as TerminalColor;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::presentation::ui::colors::Colors;
use ratatui::style::Color;

fn default_colors() -> Colors {
    let json = include_str!("../../../../assets/themes/default.json");
    let theme: ThemeFile = serde_json::from_str(json).unwrap();
    Colors::new(ColorScheme::from_theme_file(&theme, &ColorMode::Dark))
}

// ---------------------------------------------------------------------------
// to_crossterm: all named color variants
// ---------------------------------------------------------------------------
#[test]
fn to_crossterm_maps_named_colors() {
    assert_eq!(Colors::to_crossterm(Color::Red), TerminalColor::Red);
    assert_eq!(
        Colors::to_crossterm(Color::LightBlue),
        TerminalColor::DarkBlue
    );
}

#[test]
fn to_crossterm_preserves_rgb_values() {
    let color = Color::Rgb(10, 20, 30);
    assert_eq!(
        Colors::to_crossterm(color),
        TerminalColor::Rgb {
            r: 10,
            g: 20,
            b: 30
        }
    );
}

#[test]
fn to_crossterm_all_named_variants() {
    let pairs: Vec<(Color, TerminalColor)> = vec![
        (Color::Reset, TerminalColor::Reset),
        (Color::Black, TerminalColor::Black),
        (Color::Red, TerminalColor::Red),
        (Color::Green, TerminalColor::Green),
        (Color::Yellow, TerminalColor::Yellow),
        (Color::Blue, TerminalColor::Blue),
        (Color::Magenta, TerminalColor::Magenta),
        (Color::Cyan, TerminalColor::Cyan),
        (Color::Gray, TerminalColor::Grey),
        (Color::DarkGray, TerminalColor::DarkGrey),
        (Color::LightRed, TerminalColor::DarkRed),
        (Color::LightGreen, TerminalColor::DarkGreen),
        (Color::LightYellow, TerminalColor::DarkYellow),
        (Color::LightBlue, TerminalColor::DarkBlue),
        (Color::LightMagenta, TerminalColor::DarkMagenta),
        (Color::LightCyan, TerminalColor::DarkCyan),
        (Color::White, TerminalColor::White),
    ];
    for (ratatui, crossterm) in pairs {
        assert_eq!(
            Colors::to_crossterm(ratatui),
            crossterm,
            "Failed for {:?}",
            ratatui
        );
    }
}

#[test]
fn to_crossterm_indexed() {
    assert_eq!(
        Colors::to_crossterm(Color::Indexed(42)),
        TerminalColor::AnsiValue(42)
    );
}

// ---------------------------------------------------------------------------
// from_crossterm: all named color variants
// ---------------------------------------------------------------------------
#[test]
fn from_crossterm_all_named_variants() {
    let pairs: Vec<(TerminalColor, Color)> = vec![
        (TerminalColor::Reset, Color::Reset),
        (TerminalColor::Black, Color::Black),
        (TerminalColor::DarkGrey, Color::DarkGray),
        (TerminalColor::Red, Color::Red),
        (TerminalColor::DarkRed, Color::LightRed),
        (TerminalColor::Green, Color::Green),
        (TerminalColor::DarkGreen, Color::LightGreen),
        (TerminalColor::Yellow, Color::Yellow),
        (TerminalColor::DarkYellow, Color::LightYellow),
        (TerminalColor::Blue, Color::Blue),
        (TerminalColor::DarkBlue, Color::LightBlue),
        (TerminalColor::Magenta, Color::Magenta),
        (TerminalColor::DarkMagenta, Color::LightMagenta),
        (TerminalColor::Cyan, Color::Cyan),
        (TerminalColor::DarkCyan, Color::LightCyan),
        (TerminalColor::White, Color::White),
        (TerminalColor::Grey, Color::Gray),
    ];
    for (crossterm, ratatui) in pairs {
        assert_eq!(
            Colors::from_crossterm(crossterm),
            ratatui,
            "Failed for {:?}",
            crossterm
        );
    }
}

#[test]
fn from_crossterm_rgb() {
    assert_eq!(
        Colors::from_crossterm(TerminalColor::Rgb {
            r: 10,
            g: 20,
            b: 30
        }),
        Color::Rgb(10, 20, 30)
    );
}

#[test]
fn from_crossterm_ansi() {
    assert_eq!(
        Colors::from_crossterm(TerminalColor::AnsiValue(99)),
        Color::Indexed(99)
    );
}

// ---------------------------------------------------------------------------
// Countdown colors
// ---------------------------------------------------------------------------
#[test]
fn countdown_colors_use_status_colors() {
    let colors = default_colors();
    assert_eq!(colors.countdown_3(), colors.success());
    assert_eq!(colors.countdown_2(), colors.info());
    assert_eq!(colors.countdown_1(), colors.warning());
    assert_eq!(colors.countdown_go(), colors.error());
}

// ---------------------------------------------------------------------------
// Tier colors are correct constants
// ---------------------------------------------------------------------------
#[test]
fn tier_colors_are_rgb() {
    assert_eq!(Colors::tier_beginner(), Color::Rgb(95, 175, 255));
    assert_eq!(Colors::tier_intermediate(), Color::Rgb(0, 215, 255));
    assert_eq!(Colors::tier_advanced(), Color::Rgb(95, 175, 135));
    assert_eq!(Colors::tier_expert(), Color::Rgb(255, 175, 0));
    assert_eq!(Colors::tier_legendary(), Color::Rgb(215, 95, 95));
}

// ---------------------------------------------------------------------------
// Default static colors
// ---------------------------------------------------------------------------
#[test]
fn default_static_colors_are_rgb() {
    if let Color::Rgb(_, _, _) = Colors::default_text() {
    } else {
        panic!("default_text should be RGB");
    }
    if let Color::Rgb(_, _, _) = Colors::default_success() {
    } else {
        panic!("default_success should be RGB");
    }
    if let Color::Rgb(_, _, _) = Colors::default_info() {
    } else {
        panic!("default_info should be RGB");
    }
    if let Color::Rgb(_, _, _) = Colors::default_error() {
    } else {
        panic!("default_error should be RGB");
    }
    if let Color::Rgb(_, _, _) = Colors::default_warning() {
    } else {
        panic!("default_warning should be RGB");
    }
    if let Color::Rgb(_, _, _) = Colors::default_score() {
    } else {
        panic!("default_score should be RGB");
    }
    if let Color::Rgb(_, _, _) = Colors::default_border() {
    } else {
        panic!("default_border should be RGB");
    }
}

// ---------------------------------------------------------------------------
// Instance color accessors return valid colors
// ---------------------------------------------------------------------------
#[test]
fn instance_color_accessors() {
    let colors = default_colors();
    // Just ensure they don't panic and return something
    let _ = colors.border();
    let _ = colors.title();
    let _ = colors.text();
    let _ = colors.text_secondary();
    let _ = colors.background();
    let _ = colors.background_secondary();
    let _ = colors.success();
    let _ = colors.info();
    let _ = colors.error();
    let _ = colors.warning();
    let _ = colors.key_action();
    let _ = colors.key_navigation();
    let _ = colors.key_back();
    let _ = colors.score();
    let _ = colors.cpm_wpm();
    let _ = colors.accuracy();
    let _ = colors.duration();
    let _ = colors.stage_info();
    let _ = colors.typed_text();
    let _ = colors.current_cursor();
    let _ = colors.cursor_bg();
    let _ = colors.mistake_bg();
    let _ = colors.untyped_text();
}
