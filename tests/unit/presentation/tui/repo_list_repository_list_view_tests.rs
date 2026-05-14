use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::storage::StoredRepositoryWithLanguages;
use gittype::presentation::tui::views::repo_list::RepositoryListView;
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

fn repository(name: &str, languages: Vec<&str>) -> StoredRepositoryWithLanguages {
    StoredRepositoryWithLanguages {
        id: 1,
        user_name: "owner".to_string(),
        repository_name: name.to_string(),
        remote_url: format!("https://example.com/owner/{name}"),
        languages: languages.into_iter().map(str::to_string).collect(),
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
fn render_handles_empty_and_overflowing_language_lists() {
    let colors = default_colors();
    let backend = TestBackend::new(120, 8);
    let mut terminal = Terminal::new(backend).unwrap();
    let repositories = vec![
        (repository("empty", vec![]), false),
        (
            repository(
                "ellipsis",
                vec!["abcdefghijklmnopqrst", "language-that-does-not-fit"],
            ),
            true,
        ),
        (
            repository("separator-break", vec!["abcdefghijklmnopqrstuvwx", "rust"]),
            false,
        ),
        (
            repository("language-break", vec!["abcdefghijklmnopQRSTUVW", "rust"]),
            false,
        ),
    ];

    terminal
        .draw(|frame| {
            RepositoryListView::render(frame, Rect::new(0, 0, 120, 8), &repositories, &colors);
        })
        .unwrap();

    let text = buffer_text(terminal.backend().buffer());
    assert!(text.contains("No challenges"));
    assert!(text.contains("abcdefghijklmnopqrst, ..."));
    assert!(text.contains("abcdefghijklmnopqrstuvwx"));
    assert!(!text.contains("abcdefghijklmnopqrstuvwx, Rust"));
    assert!(text.contains("abcdefghijklmnopQRSTUVW,"));
    assert!(!text.contains("abcdefghijklmnopQRSTUVW, Rust"));
    assert!(!text.contains("abcdefghijklmnopQRSTUVW, ..."));
}
