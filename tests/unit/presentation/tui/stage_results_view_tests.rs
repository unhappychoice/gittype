use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::{SessionResult, StageResult};
use gittype::presentation::tui::views::session_detail_dialog::StageResultsView;
use gittype::presentation::ui::colors::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
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
fn render_without_repo_uses_stage_number_for_missing_path() {
    let colors = default_colors();
    let mut session = SessionResult::new();
    session.stage_results = vec![StageResult {
        challenge_score: 123.0,
        cpm: 456.0,
        accuracy: 98.7,
        ..StageResult::default()
    }];
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            StageResultsView::render(frame, Rect::new(0, 0, 80, 8), &session, &None, &colors);
        })
        .unwrap();

    let output = buffer_text(terminal.backend().buffer());

    assert!(output.contains("Stage Results:"));
    assert!(output.contains("Stage 1:"));
    assert!(output.contains("Score: 123"));
    assert!(!output.contains("["));
}
