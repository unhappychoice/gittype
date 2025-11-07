use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::panic_screen::PanicScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_panic_screen_snapshot_with_fixed_timestamp,
    PanicScreen,
    PanicScreen::with_error_message(
        "Test panic message".to_string(),
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Some("SystemTime { tv_sec: 1700000000, tv_nsec: 0 }".to_string())
    )
);

// Event-producing key tests
screen_key_event_test!(
    test_panic_screen_esc_exits,
    PanicScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

#[test]
fn test_panic_screen_other_keys_ignored() {
    use gittype::presentation::tui::Screen;
    use gittype::presentation::tui::ScreenDataProvider;
    use std::sync::{Arc, Mutex};

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
    let screen = PanicScreen::new(event_bus, theme_service);
    let data = EmptyMockProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    // Try various keys that should be ignored
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
        ))
        .unwrap();
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Enter,
            KeyModifiers::empty(),
        ))
        .unwrap();
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
        ))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 0);
}

// Basic methods test
screen_basic_methods_test!(
    test_panic_screen_basic_methods,
    PanicScreen,
    PanicScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::Panic,
    false
);
