use std::sync::Arc;

use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::typing::CodeContext;
use gittype::domain::models::ProcessingOptions;
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::typing_core::TypingCore;
use gittype::presentation::tui::views::typing::TypingView;
use gittype::presentation::ui::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;

struct FakeSessionManager;

impl SessionManagerInterface for FakeSessionManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn default_colors() -> Colors {
    let json = include_str!("../../../../assets/themes/default.json");
    let theme: ThemeFile = serde_json::from_str(json).unwrap();
    Colors::new(ColorScheme::from_theme_file(&theme, &ColorMode::Dark))
}

fn buffer_text(buffer: &Buffer) -> String {
    buffer
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn render_empty_typing_view_skips_metrics_for_non_concrete_session_manager() {
    let colors = default_colors();
    let typing_core = TypingCore::new("", &[], ProcessingOptions::default());
    let code_context = CodeContext {
        pre_context: Vec::new(),
        post_context: Vec::new(),
    };
    let session_manager: Arc<dyn SessionManagerInterface> = Arc::new(FakeSessionManager);
    let mut view = TypingView::new();
    let backend = TestBackend::new(80, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            view.render(
                frame,
                None,
                None,
                &typing_core,
                &[],
                &code_context,
                false,
                None,
                0,
                false,
                &session_manager,
                &colors,
            );
        })
        .unwrap();

    let output = buffer_text(terminal.backend().buffer());

    assert!(output.contains("[Challenge]"));
    assert!(output.contains("0%"));
    assert!(!output.contains("Metrics"));
}
