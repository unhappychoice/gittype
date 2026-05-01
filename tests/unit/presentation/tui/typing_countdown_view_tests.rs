use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::presentation::tui::views::typing::TypingCountdownView;
use gittype::presentation::ui::colors::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;

fn default_colors() -> Colors {
    let json = include_str!("../../../../assets/themes/default.json");
    let theme: ThemeFile = serde_json::from_str(json).unwrap();
    Colors::new(ColorScheme::from_theme_file(&theme, &ColorMode::Dark))
}

fn render_countdown(count: u8, colors: &Colors) -> Buffer {
    let backend = TestBackend::new(30, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| TypingCountdownView::render(frame, count, colors))
        .unwrap();

    terminal.backend().buffer().clone()
}

fn read_slice(buffer: &Buffer, y: u16, x: u16, width: u16) -> String {
    (x..x + width)
        .map(|column| buffer[(column, y)].symbol().to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn assert_centered_art(buffer: &Buffer, expected: &[&str]) {
    let width = expected.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
    let start_x = buffer.area.width / 2 - width / 2;
    let start_y = buffer.area.height / 2 - 2;

    expected.iter().enumerate().for_each(|(offset, line)| {
        assert_eq!(
            read_slice(buffer, start_y + offset as u16, start_x, width),
            *line
        );
    });
}

#[test]
fn render_active_counts_use_ascii_digits() {
    let colors = default_colors();
    let cases = [
        (3, vec!["  ___ ", " |__ /", "  |_ \\", " |___/"]),
        (2, vec!["  ___ ", " |_  )", "  / / ", " /___|"]),
        (1, vec!["  _ ", " / |", " | |", " |_|"]),
    ];

    cases.into_iter().for_each(|(count, expected)| {
        assert_centered_art(&render_countdown(count, &colors), &expected);
    });
}

#[test]
fn render_zero_uses_go_art() {
    let colors = default_colors();
    let expected = [
        "   ____  ___  ",
        "  / ___|/ _ \\ ",
        " | |  _| | | |",
        " | |_| | |_| |",
        "  \\____|\\___/ ",
    ];

    assert_centered_art(&render_countdown(0, &colors), &expected);
}

#[test]
fn render_unsupported_counts_leave_buffer_blank() {
    let colors = default_colors();
    let buffer = render_countdown(4, &colors);
    let output = (0..buffer.area.height)
        .map(|row| read_slice(&buffer, row, 0, buffer.area.width))
        .collect::<Vec<_>>()
        .join("");

    assert!(output.trim().is_empty());
}
