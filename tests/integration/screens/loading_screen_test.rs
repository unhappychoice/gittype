use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::infrastructure::browser;
use gittype::presentation::game::events::ExitRequested;
use gittype::presentation::game::game_data::GameData;
use gittype::presentation::tui::screens::loading_screen::LoadingScreen;
use gittype::presentation::game::{Screen, ScreenType};
use std::sync::{Arc, Mutex};

// Note: LoadingScreen requires GameData initialization, so we use manual tests
// instead of macros for better control

#[test]
fn test_loading_screen_ctrl_c_requests_exit() {
    browser::enable_test_mode();
    let _ = GameData::initialize();

    let event_bus = EventBus::new();
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &ExitRequested| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let mut screen = LoadingScreen::new(event_bus);

    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
        ))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 1);
}

#[test]
fn test_loading_screen_char_a_ignored() {
    browser::enable_test_mode();
    let _ = GameData::initialize();

    let event_bus = EventBus::new();
    let mut screen = LoadingScreen::new(event_bus);

    // Should not panic
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
        ))
        .unwrap();
}

#[test]
fn test_loading_screen_enter_ignored() {
    browser::enable_test_mode();
    let _ = GameData::initialize();

    let event_bus = EventBus::new();
    let mut screen = LoadingScreen::new(event_bus);

    // Should not panic
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Enter,
            KeyModifiers::empty(),
        ))
        .unwrap();
}

#[test]
fn test_loading_screen_esc_ignored() {
    browser::enable_test_mode();
    let _ = GameData::initialize();

    let event_bus = EventBus::new();
    let mut screen = LoadingScreen::new(event_bus);

    // Should not panic
    screen
        .handle_key_event(crossterm::event::KeyEvent::new(
            KeyCode::Esc,
            KeyModifiers::empty(),
        ))
        .unwrap();
}

#[test]
fn test_loading_screen_initialization() {
    browser::enable_test_mode();
    let _ = GameData::initialize();

    let event_bus = EventBus::new();
    let screen = LoadingScreen::new(event_bus);

    assert_eq!(screen.get_type(), ScreenType::Loading);
}
