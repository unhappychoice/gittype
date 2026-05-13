use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::presentation::tui::views::AsciiScoreView;
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

#[test]
fn render_negative_score_ignores_minus_sign() {
    let colors = default_colors();
    let backend = TestBackend::new(32, 4);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            AsciiScoreView::render(frame, frame.area(), -12.0, &colors);
        })
        .unwrap();

    let output = buffer_text(terminal.backend().buffer());

    assert!(output.contains("  _"));
    assert!(output.contains("___"));
    assert!(!output.contains("-"));
}
