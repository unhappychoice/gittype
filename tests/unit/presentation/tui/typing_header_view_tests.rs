use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::Challenge;
use gittype::presentation::tui::views::TypingHeaderView;
use gittype::presentation::ui::colors::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;

fn default_colors() -> Colors {
    let json = include_str!("../../../../assets/themes/default.json");
    let theme: ThemeFile = serde_json::from_str(json).unwrap();
    Colors::new(ColorScheme::from_theme_file(&theme, &ColorMode::Dark))
}

fn buffer_text(buffer: &Buffer) -> String {
    (0..buffer.area.height)
        .map(|row| {
            (0..buffer.area.width)
                .map(|column| buffer[(column, row)].symbol().to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_header(challenge: Option<&Challenge>) -> String {
    let colors = default_colors();
    let backend = TestBackend::new(80, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            TypingHeaderView::render(frame, frame.area(), challenge, None, &colors);
        })
        .unwrap();

    buffer_text(terminal.backend().buffer())
}

#[test]
fn render_without_challenge_shows_placeholder() {
    let output = render_header(None);

    assert!(output.contains("[Challenge]"));
}

#[test]
fn render_challenge_without_difficulty_shows_unknown() {
    let challenge = Challenge::new("one".to_string(), "fn main() {}".to_string());
    let output = render_header(Some(&challenge));

    assert!(output.contains("Challenge one"));
    assert!(output.contains("[Unknown]"));
}
