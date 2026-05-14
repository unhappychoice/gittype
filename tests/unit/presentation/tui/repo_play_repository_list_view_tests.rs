use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::storage::StoredRepositoryWithLanguages;
use gittype::presentation::tui::views::repo_play::RepositoryListView;
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

fn repository(languages: Vec<String>) -> StoredRepositoryWithLanguages {
    StoredRepositoryWithLanguages {
        id: 1,
        user_name: "owner".to_string(),
        repository_name: "project".to_string(),
        remote_url: "https://github.com/owner/project".to_string(),
        languages,
    }
}

#[test]
fn render_shows_no_challenges_for_repository_without_languages() {
    let colors = default_colors();
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut list_state = ListState::default();
    let repositories = vec![(repository(vec![]), false)];

    terminal
        .draw(|frame| {
            RepositoryListView::render(
                frame,
                Rect::new(0, 0, 80, 8),
                &repositories,
                &mut list_state,
                &colors,
            );
        })
        .unwrap();

    let text = buffer_text(terminal.backend().buffer());

    assert!(text.contains("owner/project"));
    assert!(text.contains("No challenges"));
}
