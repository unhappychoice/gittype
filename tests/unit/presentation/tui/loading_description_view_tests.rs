use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::presentation::tui::screens::loading_screen::LoadingScreenState;
use gittype::presentation::tui::views::loading::loading_description_view::LoadingDescriptionView;
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
fn render_with_poisoned_steps_lock_keeps_base_description() {
    let state = LoadingScreenState::default();
    let steps = state.all_steps.clone();
    let _ = std::thread::spawn(move || {
        let _guard = steps.write().unwrap();
        panic!("poison all_steps");
    })
    .join();
    let colors = default_colors();
    let backend = TestBackend::new(80, 6);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| LoadingDescriptionView::render(frame, frame.area(), &state, &colors))
        .unwrap();

    assert!(buffer_text(terminal.backend().buffer())
        .contains("Analyzing your repository to create typing challenges..."));
}
