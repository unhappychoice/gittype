use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::loading::StepType;
use gittype::presentation::tui::screens::loading_screen::LoadingScreenState;
use gittype::presentation::tui::views::loading::loading_progress_view::LoadingProgressView;
use gittype::presentation::ui::colors::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;

fn default_colors() -> Colors {
    let json = include_str!("../../../../assets/themes/default.json");
    let theme: ThemeFile = serde_json::from_str(json).unwrap();
    Colors::new(ColorScheme::from_theme_file(&theme, &ColorMode::Dark))
}

fn render_progress(state: &LoadingScreenState) -> Buffer {
    let colors = default_colors();
    let backend = TestBackend::new(80, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| LoadingProgressView::render(frame, frame.area(), state, &colors))
        .unwrap();

    terminal.backend().buffer().clone()
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
fn render_with_poisoned_progress_lock_shows_spinner_text() {
    let state = LoadingScreenState::default();
    let progress = state.step_progress.clone();
    let _ = std::thread::spawn(move || {
        let _guard = progress.write().unwrap();
        panic!("poison step_progress");
    })
    .join();

    assert!(buffer_text(&render_progress(&state)).contains("Working..."));
}

#[test]
fn render_without_progress_for_extracting_leaves_area_blank() {
    let state = LoadingScreenState::default();
    *state.current_step.write().unwrap() = StepType::Extracting;

    assert!(buffer_text(&render_progress(&state)).trim().is_empty());
}
