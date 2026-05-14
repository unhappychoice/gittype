use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::storage::SessionStageResult;
use gittype::presentation::tui::views::StageDetailsView;
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

fn render_stage_details(stage_results: &[SessionStageResult], height: u16) -> String {
    let colors = default_colors();
    let backend = TestBackend::new(100, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            StageDetailsView::render(frame, frame.area(), stage_results, 0, &colors);
        })
        .unwrap();

    buffer_text(terminal.backend().buffer())
}

fn stage(number: i64, was_failed: bool, was_skipped: bool) -> SessionStageResult {
    SessionStageResult {
        stage_number: number,
        wpm: 60.0,
        cpm: 300.0,
        accuracy: 95.0,
        keystrokes: 100,
        mistakes: 5,
        duration_ms: 30000,
        score: 1234.0,
        language: Some("Rust".to_string()),
        difficulty_level: Some("Easy".to_string()),
        rank_name: Some("A".to_string()),
        tier_name: Some("Advanced".to_string()),
        rank_position: 1,
        rank_total: 10,
        position: number as usize,
        total: 10,
        was_skipped,
        was_failed,
        file_path: Some(format!("src/stage_{number}.rs")),
        start_line: Some(10),
        end_line: Some(20),
        code_content: None,
    }
}

#[test]
fn render_empty_stage_results_shows_placeholder() {
    let output = render_stage_details(&[], 6);

    assert!(output.contains("Stage Details"));
    assert!(output.contains("No stage data available"));
}

#[test]
fn render_failed_and_skipped_stages_shows_statuses_and_file_ranges() {
    let output = render_stage_details(&[stage(1, true, false), stage(2, false, true)], 12);

    assert!(output.contains("Stage #1"));
    assert!(output.contains("[FAILED]"));
    assert!(output.contains("src/stage_1.rs:10-20"));
    assert!(output.contains("Stage #2"));
    assert!(output.contains("[SKIPPED]"));
}

#[test]
fn render_many_stages_shows_scroll_hint() {
    let stages = (1..=8)
        .map(|number| stage(number, false, false))
        .collect::<Vec<_>>();

    let output = render_stage_details(&stages, 7);

    assert!(output.contains("stages shown"));
    assert!(output.contains("to scroll"));
}
