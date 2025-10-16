use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::game::screens::panic_screen::PanicScreen;

screen_snapshot_test!(
    test_panic_screen_snapshot_with_fixed_timestamp,
    PanicScreen,
    PanicScreen::with_error_message(
        "Test panic message".to_string(),
        EventBus::new(),
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
    use gittype::presentation::game::models::ScreenDataProvider;
    use gittype::presentation::game::Screen;
    use std::sync::{Arc, Mutex};

    let event_bus = EventBus::new();
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let mut screen = PanicScreen::new(event_bus);
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
