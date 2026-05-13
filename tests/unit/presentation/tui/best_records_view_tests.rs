use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::storage::SessionResultData;
use gittype::domain::models::SessionResult;
use gittype::domain::repositories::session_repository::{BestRecords, BestStatus};
use gittype::presentation::tui::views::session_detail_dialog::BestRecordsView;
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

fn sample_result(score: f64, cpm: f64, accuracy: f64) -> SessionResultData {
    SessionResultData {
        keystrokes: 100,
        mistakes: 0,
        duration_ms: 30_000,
        wpm: cpm / 5.0,
        cpm,
        accuracy,
        stages_completed: 1,
        stages_attempted: 1,
        stages_skipped: 0,
        score,
        rank_name: None,
        tier_name: None,
        rank_position: None,
        rank_total: None,
        position: None,
        total: None,
    }
}

fn render_to_string(
    session_result: &SessionResult,
    best_status: Option<&BestStatus>,
    best_records: Option<&BestRecords>,
) -> String {
    let colors = default_colors();
    let backend = TestBackend::new(120, 12);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            BestRecordsView::render(
                frame,
                Rect::new(0, 0, 120, 12),
                session_result,
                best_status,
                best_records,
                &colors,
            );
        })
        .unwrap();

    buffer_text(terminal.backend().buffer())
}

#[test]
fn render_without_best_records_draws_nothing() {
    let session = SessionResult::new();

    let output = render_to_string(&session, None, None);

    assert!(!output.contains("BEST RECORDS"));
    assert!(!output.contains("NEW PB"));
}

#[test]
fn render_uses_best_status_flags_for_new_pb_markers() {
    let mut session = SessionResult::new();
    session.session_score = 200.0;

    let best_status = BestStatus {
        is_todays_best: true,
        is_weekly_best: false,
        is_all_time_best: true,
        best_type: None,
        todays_best_score: 0.0,
        weekly_best_score: 0.0,
        all_time_best_score: 0.0,
    };

    let best_records = BestRecords {
        todays_best: Some(sample_result(150.0, 300.0, 95.0)),
        weekly_best: Some(sample_result(220.0, 320.0, 96.0)),
        all_time_best: Some(sample_result(180.0, 340.0, 97.0)),
    };

    let output = render_to_string(&session, Some(&best_status), Some(&best_records));

    assert!(output.contains("BEST RECORDS"));
    assert!(output.contains("Today's Best"));
    assert!(output.contains("Weekly Best"));
    assert!(output.contains("All time Best"));
    assert!(output.contains("NEW PB"));
    assert!(output.contains("(+50)"));
    assert!(output.contains("(-20)"));
    assert!(output.contains("(+20)"));
}

#[test]
fn render_falls_back_to_score_comparison_when_no_status() {
    let mut session = SessionResult::new();
    session.session_score = 250.0;

    let best_records = BestRecords {
        todays_best: Some(sample_result(100.0, 300.0, 95.0)),
        weekly_best: Some(sample_result(250.0, 320.0, 96.0)),
        all_time_best: Some(sample_result(400.0, 340.0, 97.0)),
    };

    let output = render_to_string(&session, None, Some(&best_records));

    assert!(output.contains("BEST RECORDS"));
    assert!(output.contains("NEW PB"));
    assert!(output.contains("(+150)"));
    assert!(output.contains("(-150)"));
}

#[test]
fn render_handles_missing_records_with_placeholder() {
    let session = SessionResult::new();

    let best_records = BestRecords {
        todays_best: None,
        weekly_best: None,
        all_time_best: None,
    };

    let output = render_to_string(&session, None, Some(&best_records));

    assert!(output.contains("BEST RECORDS"));
    assert!(output.contains("No previous record"));
}
