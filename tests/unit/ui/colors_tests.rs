use crossterm::style::Color as TerminalColor;
use gittype::ui::colors::Colors;
use ratatui::style::Color;

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
