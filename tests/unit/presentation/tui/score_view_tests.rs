use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::{Rank, RankTier, SessionResult};
use gittype::domain::repositories::session_repository::BestStatus;
use gittype::presentation::tui::views::session_summary::ScoreView;
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

fn render_score(session_score: f64, best_status: &BestStatus) -> (usize, String) {
    render_score_with_status(session_score, Some(best_status))
}

fn render_score_with_status(
    session_score: f64,
    best_status: Option<&BestStatus>,
) -> (usize, String) {
    let colors = default_colors();
    let backend = TestBackend::new(80, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut result = SessionResult::new();
    result.session_score = session_score;
    let rank = Rank::new("Test Rank", RankTier::Beginner, 0, 999);
    let mut rendered_height = 0;

    terminal
        .draw(|frame| {
            rendered_height = ScoreView::render(
                frame,
                Rect::new(0, 0, 80, 12),
                &result,
                &rank,
                best_status,
                &colors,
            );
        })
        .unwrap();

    (rendered_height, buffer_text(terminal.backend().buffer()))
}

#[test]
fn create_ascii_numbers_ignores_non_digits() {
    let lines = ScoreView::create_ascii_numbers("abc");

    assert_eq!(lines, vec!["", "", "", ""]);
}

#[test]
fn render_shows_all_time_best_with_positive_difference() {
    let mut best_status = BestStatus::new();
    best_status.best_type = Some("ALL TIME".to_string());
    best_status.all_time_best_score = 100.0;

    let (rendered_height, output) = render_score(150.0, &best_status);

    assert_eq!(rendered_height, 7);
    assert!(output.contains("*** ALL TIME BEST ***"));
    assert!(output.contains("(+50)"));
}

#[test]
fn render_shows_weekly_best_with_negative_difference() {
    let mut best_status = BestStatus::new();
    best_status.best_type = Some("WEEKLY".to_string());
    best_status.weekly_best_score = 200.0;

    let (_rendered_height, output) = render_score(150.0, &best_status);

    assert!(output.contains("*** WEEKLY BEST ***"));
    assert!(output.contains("(-50)"));
}

#[test]
fn render_uses_todays_score_for_unknown_best_type() {
    let mut best_status = BestStatus::new();
    best_status.best_type = Some("RECENT".to_string());
    best_status.todays_best_score = 125.0;

    let (_rendered_height, output) = render_score(150.0, &best_status);

    assert!(output.contains("*** RECENT BEST ***"));
    assert!(output.contains("(+25)"));
}

#[test]
fn render_omits_best_label_when_status_has_no_best_type() {
    let mut best_status = BestStatus::new();
    best_status.todays_best_score = 175.0;

    let (rendered_height, output) = render_score(150.0, &best_status);

    assert_eq!(rendered_height, 7);
    assert!(output.contains("SESSION SCORE"));
    assert!(output.contains("(-25)"));
    assert!(!output.contains("BEST"));
}

#[test]
fn render_shows_todays_best_with_equal_score() {
    let mut best_status = BestStatus::new();
    best_status.best_type = Some("TODAY'S".to_string());
    best_status.todays_best_score = 150.0;

    let (rendered_height, output) = render_score(150.0, &best_status);

    assert_eq!(rendered_height, 7);
    assert!(output.contains("*** TODAY'S BEST ***"));
}

#[test]
fn render_without_best_status_uses_today_best_fallback() {
    let (rendered_height, output) = render_score_with_status(150.0, None);

    assert_eq!(rendered_height, 7);
    assert!(output.contains("SESSION SCORE"));
}
