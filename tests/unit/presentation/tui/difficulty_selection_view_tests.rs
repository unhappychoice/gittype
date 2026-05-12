use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::DifficultyLevel;
use gittype::presentation::tui::views::title::DifficultySelectionView;
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

#[test]
fn render_zero_count_with_error_message() {
    let colors = default_colors();
    let difficulties = [
        ("Easy", DifficultyLevel::Easy),
        ("Normal", DifficultyLevel::Normal),
        ("Hard", DifficultyLevel::Hard),
        ("Wild", DifficultyLevel::Wild),
        ("Zen", DifficultyLevel::Zen),
    ];
    let challenge_counts = [0; 5];
    let error = "No challenges available for Easy".to_string();
    let backend = TestBackend::new(80, 8);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            DifficultySelectionView::render(
                frame,
                frame.area(),
                &difficulties,
                0,
                &challenge_counts,
                Some(&error),
                &colors,
            );
        })
        .unwrap();

    let output = buffer_text(terminal.backend().buffer());

    assert!(output.contains("Difficulty:"));
    assert!(output.contains("Easy"));
    assert!(output.contains("Challenge count will be displayed after loading"));
    assert!(output.contains("No challenges available for Easy"));
    assert!(!output.contains("Short code snippets"));
}
