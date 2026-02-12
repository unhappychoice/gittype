use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::info_dialog::InfoDialogScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_info_dialog_snapshot_default,
    InfoDialogScreen,
    InfoDialogScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    )
);

// Event-producing key tests (Menu state)
screen_key_event_test!(
    test_info_dialog_esc_closes,
    InfoDialogScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_info_dialog_ctrl_c_exits,
    InfoDialogScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

// Non-event key tests (Menu state)
screen_key_tests!(
    InfoDialogScreen,
    EmptyMockProvider,
    [
        (
            test_info_dialog_space_selects,
            KeyCode::Char(' '),
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_info_dialog_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_info_dialog_basic_methods,
    InfoDialogScreen,
    InfoDialogScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::InfoDialog,
    false
);

// Snapshot test with fallback state
screen_snapshot_test!(
    test_info_dialog_snapshot_fallback,
    InfoDialogScreen,
    InfoDialogScreen::new_fallback(
        "GitHub Repository".to_string(),
        "https://github.com/unhappychoice/gittype".to_string(),
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    )
);

// Snapshot test after navigating down (selected_option = 1)
screen_snapshot_test!(
    test_info_dialog_snapshot_second_option,
    InfoDialogScreen,
    InfoDialogScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [crossterm::event::KeyEvent::new(
        KeyCode::Down,
        KeyModifiers::empty()
    )]
);

// Snapshot test navigating to third option (Close)
screen_snapshot_test!(
    test_info_dialog_snapshot_third_option,
    InfoDialogScreen,
    InfoDialogScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [
        crossterm::event::KeyEvent::new(KeyCode::Down, KeyModifiers::empty()),
        crossterm::event::KeyEvent::new(KeyCode::Down, KeyModifiers::empty())
    ]
);

// Test wrap-around navigation: Up from first item goes to last
screen_snapshot_test!(
    test_info_dialog_snapshot_wrap_up,
    InfoDialogScreen,
    InfoDialogScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [crossterm::event::KeyEvent::new(
        KeyCode::Up,
        KeyModifiers::empty()
    )]
);

// Test: select GitHub option (index 0) - browser opens (no-op in test-mocks), publishes Pop
#[test]
fn test_info_dialog_select_github_option() {
    use gittype::presentation::tui::Screen;
    use std::sync::Mutex;

    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;

    let screen = InfoDialogScreen::new(event_bus, theme_service);

    // Select GitHub option (already at index 0)
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char(' '),
            KeyModifiers::empty(),
        ))
        .unwrap();

    let captured = events.lock().unwrap();
    assert_eq!(captured.len(), 1);
}

// Test: select X option (index 1)
#[test]
fn test_info_dialog_select_x_option() {
    use gittype::presentation::tui::Screen;
    use std::sync::Mutex;

    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;

    let screen = InfoDialogScreen::new(event_bus, theme_service);

    // Navigate to X option (index 1) then select
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::empty(),
        ))
        .unwrap();
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char(' '),
            KeyModifiers::empty(),
        ))
        .unwrap();

    let captured = events.lock().unwrap();
    assert_eq!(captured.len(), 1);
}

// Test: select Close option (index 2)
#[test]
fn test_info_dialog_select_close_option() {
    use gittype::presentation::tui::Screen;
    use std::sync::Mutex;

    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;

    let screen = InfoDialogScreen::new(event_bus, theme_service);

    // Navigate to Close option (index 2) then select
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::empty(),
        ))
        .unwrap();
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::empty(),
        ))
        .unwrap();
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char(' '),
            KeyModifiers::empty(),
        ))
        .unwrap();

    let captured = events.lock().unwrap();
    assert_eq!(captured.len(), 1);
}

// Test: Fallback state - Esc returns to Menu
#[test]
fn test_info_dialog_fallback_esc_returns_to_menu() {
    use gittype::presentation::tui::Screen;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    let screen = InfoDialogScreen::new_fallback(
        "Test".to_string(),
        "https://example.com".to_string(),
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    // Render fallback state
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let before = terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
    let before_buffer = before.buffer.clone();

    // Press Esc to return to Menu state
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Esc,
            KeyModifiers::empty(),
        ))
        .unwrap();

    // Render again - should now be in Menu state with different content
    let backend2 = TestBackend::new(120, 40);
    let mut terminal2 = Terminal::new(backend2).unwrap();
    let after = terminal2
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
    let after_buffer = after.buffer.clone();

    assert_ne!(
        before_buffer, after_buffer,
        "Rendered content should change after Esc transitions from Fallback to Menu"
    );
}

// Test: Fallback state - Ctrl+C exits
#[test]
fn test_info_dialog_fallback_ctrl_c_exits() {
    use gittype::presentation::tui::Screen;
    use std::sync::Mutex;

    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = InfoDialogScreen::new_fallback(
        "Test".to_string(),
        "https://example.com".to_string(),
        event_bus,
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
        ))
        .unwrap();

    let captured = events.lock().unwrap();
    assert_eq!(captured.len(), 1);
}

// Test: Fallback state - other key ignored
#[test]
fn test_info_dialog_fallback_other_key_ignored() {
    use gittype::presentation::tui::Screen;

    let screen = InfoDialogScreen::new_fallback(
        "Test".to_string(),
        "https://example.com".to_string(),
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    // Should not panic
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
        ))
        .unwrap();
}

// Test: Menu state - other key ignored
#[test]
fn test_info_dialog_menu_other_key_ignored() {
    use gittype::presentation::tui::Screen;

    let screen = InfoDialogScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('z'),
            KeyModifiers::empty(),
        ))
        .unwrap();
}
