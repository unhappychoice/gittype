use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::presentation::tui::views::TypingDialogView;
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

fn render_dialog(skips_remaining: usize) -> String {
    let colors = default_colors();
    let backend = TestBackend::new(64, 16);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| TypingDialogView::render(frame, skips_remaining, &colors))
        .unwrap();

    buffer_text(terminal.backend().buffer())
}

#[test]
fn render_with_no_skips_shows_disabled_skip_option() {
    let output = render_dialog(0);

    assert!(output.contains("No skips remaining"));
    assert!(!output.contains("Skip challenge"));
}
