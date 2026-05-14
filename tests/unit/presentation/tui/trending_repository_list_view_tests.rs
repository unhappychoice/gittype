use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::repositories::trending_repository::TrendingRepositoryInfo;
use gittype::presentation::tui::views::trending_repository_selection::RepositoryListView;
use gittype::presentation::ui::colors::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use ratatui::Terminal;

fn default_colors() -> Colors {
    let json = include_str!("../../../../assets/themes/default.json");
    let theme: ThemeFile = serde_json::from_str(json).unwrap();
    Colors::new(ColorScheme::from_theme_file(&theme, &ColorMode::Dark))
}

fn repository(
    repo_name: &str,
    primary_language: Option<&str>,
    description: Option<&str>,
) -> TrendingRepositoryInfo {
    TrendingRepositoryInfo {
        repo_name: repo_name.to_string(),
        primary_language: primary_language.map(str::to_string),
        description: description.map(str::to_string),
        stars: "100".to_string(),
        forks: "10".to_string(),
        total_score: "200".to_string(),
    }
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
fn render_truncates_long_fields_and_uses_missing_value_fallbacks() {
    let colors = default_colors();
    let backend = TestBackend::new(120, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut list_state = ListState::default();
    let repositories = vec![
        repository(
            "owner/repository-name-that-is-far-longer-than-the-column",
            Some("Rust"),
            Some("A description that is intentionally longer than the list view allows"),
        ),
        repository("owner/no-metadata", None, None),
    ];

    terminal
        .draw(|frame| {
            RepositoryListView::render(
                frame,
                Rect::new(0, 0, 120, 8),
                &repositories,
                &mut list_state,
                &colors,
            );
        })
        .unwrap();

    let text = buffer_text(terminal.backend().buffer());

    assert!(text.contains("owner/repository-name-that-is-fa..."));
    assert!(text.contains("A description that is intentionally longer than..."));
    assert!(text.contains("(Unknown)"));
    assert!(text.contains("No description"));
}
