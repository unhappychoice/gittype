use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::color_scheme::{ColorScheme, ThemeFile};
use gittype::domain::models::typing::CodeContext;
use gittype::domain::models::{Challenge, ProcessingOptions};
use gittype::domain::services::typing_core::TypingCore;
use gittype::presentation::tui::views::typing::typing_content_view::TypingContentView;
use gittype::presentation::ui::Colors;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::Terminal;

fn test_colors() -> Colors {
    let json = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/themes/default.json"
    ));
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
fn test_calculate_scroll_offset() {
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 10), 0);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 15), 5);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 30), 20);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 25, 50), 5);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 30, 15), 5);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 5), 0);
}

#[test]
fn render_includes_context_lines_and_reuses_cached_content() {
    let code = "\tlet value = 1;\nvalue\u{0007}\n";
    let challenge = Challenge::new("context-render".to_string(), code.to_string())
        .with_source_info("src/lib.rs".to_string(), 10, 11);
    let typing_core = TypingCore::from_challenge(&challenge, Some(ProcessingOptions::default()));
    let chars = code.chars().collect::<Vec<_>>();
    let context = CodeContext {
        pre_context: vec!["before_one".to_string(), "before_two".to_string()],
        post_context: vec!["after_one".to_string(), "after_two".to_string()],
    };
    let colors = test_colors();
    let mut view = TypingContentView::new();
    let backend = TestBackend::new(90, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    for _ in 0..2 {
        terminal
            .draw(|frame| {
                view.render(
                    frame,
                    Rect::new(0, 0, 90, 24),
                    true,
                    Some(&challenge),
                    &typing_core,
                    &chars,
                    &context,
                    &colors,
                );
            })
            .unwrap();
    }

    let output = buffer_text(terminal.backend().buffer());
    assert!(output.contains("before_one"));
    assert!(output.contains("before_two"));
    assert!(output.contains("after_one"));
    assert!(output.contains("after_two"));
    assert!(output.contains("    let value = 1;"));
    assert!(output.contains("value?"));
}
