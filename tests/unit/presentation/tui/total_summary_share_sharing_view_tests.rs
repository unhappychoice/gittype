use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::presentation::sharing::SharingPlatform;
use gittype::presentation::tui::views::total_summary_share::SharingView;
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
fn render_fallback_url_shows_platform_url_and_exit_prompt() {
    let colors = default_colors();
    let backend = TestBackend::new(100, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            SharingView::render_fallback_url(
                frame,
                "https://example.com/share",
                &SharingPlatform::LinkedIn,
                &colors,
            );
        })
        .unwrap();

    let output = buffer_text(terminal.backend().buffer());

    assert!(output.contains("Could not open LinkedIn automatically"));
    assert!(output.contains("Please copy the URL below and open it in your browser:"));
    assert!(output.contains("https://example.com/share"));
    assert!(output.contains("[ESC] Exit"));
}
