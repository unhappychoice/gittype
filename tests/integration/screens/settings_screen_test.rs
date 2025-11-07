use crate::integration::screens::mocks::settings_screen_mock::MockSettingsScreenDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::presentation::tui::screens::settings_screen::SettingsScreen;
use gittype::presentation::tui::Screen;
use gittype::presentation::tui::ScreenDataProvider;
use std::sync::{Arc, Mutex};

screen_snapshot_test!(
    test_settings_screen_snapshot_color_mode,
    SettingsScreen,
    SettingsScreen::new(Arc::new(EventBus::new())),
    provider = MockSettingsScreenDataProvider
);

screen_snapshot_test!(
    test_settings_screen_snapshot_theme,
    SettingsScreen,
    SettingsScreen::new(Arc::new(EventBus::new())),
    provider = MockSettingsScreenDataProvider,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

// Event-producing key tests (manual implementation because SettingsScreen takes only 1 arg)
#[test]
fn test_settings_screen_space_saves_and_navigates_back() {
    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 1);
}

#[test]
fn test_settings_screen_esc_cancels_and_navigates_back() {
    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 1);
}

#[test]
fn test_settings_screen_ctrl_c_exits() {
    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 1);
}

// Non-event key tests (manual implementation)
#[test]
fn test_settings_screen_key_left_switches_tab() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_settings_screen_key_h_switches_tab() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_settings_screen_key_right_switches_tab() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Right, KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_settings_screen_key_l_switches_tab() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_settings_screen_key_up_navigates_list() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_settings_screen_key_k_navigates_list() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_settings_screen_key_down_navigates_list() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_settings_screen_key_j_navigates_list() {
    let event_bus = Arc::new(EventBus::new());
    let screen = SettingsScreen::new(event_bus);
    let data = MockSettingsScreenDataProvider.provide().unwrap();
    let _ = screen.init_with_data(data);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty()))
        .unwrap();
}

// Basic methods test
screen_basic_methods_test!(
    test_settings_screen_basic_methods,
    SettingsScreen,
    SettingsScreen::new(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::Settings,
    false,
    MockSettingsScreenDataProvider
);
