use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::presentation::tui::views::title::GitRepositoryView;
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
fn render_without_repository_leaves_buffer_blank() {
    let colors = default_colors();
    let backend = TestBackend::new(40, 4);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            GitRepositoryView::render(frame, None, &colors);
        })
        .unwrap();

    assert!(buffer_text(terminal.backend().buffer()).trim().is_empty());
}
