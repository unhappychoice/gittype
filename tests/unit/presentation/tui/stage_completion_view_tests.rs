use std::time::Duration;

use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::services::scoring::StageResult;
use gittype::presentation::tui::views::stage_summary::StageCompletionView;
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

fn render_stage_completion(metrics: &StageResult, has_next_stage: bool) -> String {
    let colors = default_colors();
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            StageCompletionView::render(frame, metrics, 2, 3, has_next_stage, 42, &colors);
        })
        .unwrap();

    buffer_text(terminal.backend().buffer())
}

#[test]
fn render_failed_stage_uses_failure_labels_without_metrics() {
    let metrics = StageResult {
        completion_time: Duration::from_millis(3250),
        was_failed: true,
        ..StageResult::default()
    };

    let output = render_stage_completion(&metrics, true);

    assert!(output.contains("=== STAGE 2 FAILED ==="));
    assert!(output.contains("FAILED AFTER"));
    assert!(output.contains("Stage 2 of 3"));
    assert!(output.contains("Next stage starting..."));
    assert!(!output.contains("CPM:"));
}

#[test]
fn render_skipped_stage_uses_skipped_labels_without_metrics() {
    let metrics = StageResult {
        was_skipped: true,
        ..StageResult::default()
    };

    let output = render_stage_completion(&metrics, false);

    assert!(output.contains("=== STAGE 2 SKIPPED ==="));
    assert!(output.contains("SKIPPED"));
    assert!(output.contains("Stage 2 of 3"));
    assert!(!output.contains("Next stage starting..."));
    assert!(!output.contains("CPM:"));
}
