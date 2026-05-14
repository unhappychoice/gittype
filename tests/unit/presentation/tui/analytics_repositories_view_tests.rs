use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::services::analytics_service::AnalyticsData;
use gittype::presentation::tui::views::analytics::RepositoriesView;
use gittype::presentation::ui::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{ListState, ScrollbarState};
use ratatui::Terminal;
use std::collections::HashMap;

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

fn analytics_data_with_repository_summary() -> AnalyticsData {
    AnalyticsData {
        total_sessions: 1,
        avg_cpm: 240.0,
        avg_accuracy: 98.0,
        total_time_hours: 0.1,
        cpm_trend: Vec::new(),
        accuracy_trend: Vec::new(),
        top_repositories: vec![("owner/repo".to_string(), 240.0)],
        top_languages: Vec::new(),
        daily_sessions: HashMap::new(),
        best_cpm: 240.0,
        total_mistakes: 0,
        avg_session_duration: 1.0,
        current_streak: 0,
        repository_stats: HashMap::new(),
        language_stats: HashMap::new(),
        reference_date: None,
    }
}

#[test]
fn render_selected_repository_without_detailed_stats_uses_summary_metrics() {
    let colors = default_colors();
    let data = analytics_data_with_repository_summary();
    let backend = TestBackend::new(100, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut list_state = ListState::default().with_selected(Some(0));
    let mut scroll_state = ScrollbarState::default();

    terminal
        .draw(|frame| {
            RepositoriesView::render_with_state(
                frame,
                Rect::new(0, 0, 100, 24),
                &data,
                &mut list_state,
                &mut scroll_state,
                &colors,
            );
        })
        .unwrap();

    let output = buffer_text(terminal.backend().buffer());
    assert!(output.contains("Repository:"));
    assert!(output.contains("owner/repo"));
    assert!(output.contains("Average CPM:"));
    assert!(output.contains("240.0"));
    assert!(output.contains("WPM Equivalent:"));
    assert!(output.contains("48.0"));
}
