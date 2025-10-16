use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::ScreenDataProvider;
use gittype::presentation::tui::screens::typing_screen::TypingScreen;
use gittype::presentation::tui::Screen;
use std::sync::{Arc, Mutex};

// Note: TypingScreen has complex state management (waiting_to_start, countdown, dialog_shown)
// Full coverage of all state combinations is not practical for integration tests
// These tests cover the basic Ctrl+C exit functionality

#[test]
fn test_typing_screen_ctrl_c_exits() {
    use crate::integration::screens::helpers::EmptyMockProvider;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    let event_bus = EventBus::new();
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let mut screen = TypingScreen::new(event_bus);
    let data = EmptyMockProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        .unwrap();

    let captured_events = events.lock().unwrap();
    // Ctrl+C should produce NavigateTo::PopTo(ScreenType::Title)
    assert_eq!(captured_events.len(), 1);
}
