use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::storage::SessionResultData;
use gittype::presentation::tui::views::PerformanceMetricsView;
use gittype::presentation::ui::Colors;
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

fn render_metrics(session_result: Option<&SessionResultData>) -> String {
    let colors = default_colors();
    let backend = TestBackend::new(80, 12);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            PerformanceMetricsView::render(frame, frame.area(), session_result, &colors);
        })
        .unwrap();

    buffer_text(terminal.backend().buffer())
}

fn session_result(stages_skipped: usize) -> SessionResultData {
    SessionResultData {
        keystrokes: 120,
        mistakes: 3,
        duration_ms: 125_000,
        wpm: 48.5,
        cpm: 242.5,
        accuracy: 97.5,
        stages_completed: 2,
        stages_attempted: 4,
        stages_skipped,
        score: 3456.7,
        rank_name: None,
        tier_name: None,
        rank_position: None,
        rank_total: None,
        position: None,
        total: None,
    }
}

#[test]
fn render_with_skipped_stages_shows_skipped_count_and_unknown_rank() {
    let result = session_result(2);
    let output = render_metrics(Some(&result));

    assert!(output.contains("Performance"));
    assert!(output.contains("unknown/unknown"));
    assert!(output.contains("Skipped:"));
    assert!(output.contains("2"));
}

#[test]
fn render_without_session_result_shows_placeholder() {
    let output = render_metrics(None);

    assert!(output.contains("Performance"));
    assert!(output.contains("No performance data available"));
}
