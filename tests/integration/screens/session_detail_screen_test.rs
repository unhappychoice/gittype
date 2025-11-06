use crate::integration::screens::mocks::records_screen_mock::MockRecordsDataProvider;
use crate::integration::screens::mocks::session_repository_mock::MockSessionRepository;
use crate::integration::screens::mocks::session_service_mock::MockSessionService;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::{RecordsScreen, SessionDetailScreen};
use gittype::presentation::tui::Screen;
use gittype::presentation::tui::ScreenDataProvider;
use std::sync::{Arc, Mutex};

// Helper function to create and initialize SessionDetailScreen from RecordsScreen
fn create_initialized_session_detail_screen(
    event_bus: Arc<dyn EventBusInterface>,
) -> SessionDetailScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>;
    let screen = SessionDetailScreen::new(event_bus.clone(), theme_service.clone(), Arc::new(MockSessionRepository::new()));

    let records = RecordsScreen::new(
        Arc::new(EventBus::new()),
        theme_service.clone(),
        Arc::new(MockSessionService::new()),
    );
    let data = MockRecordsDataProvider.provide().unwrap();
    records.init_with_data(data).unwrap();
    records.set_selected_session_from_index(0);
    screen.on_pushed_from(&records).unwrap();

    screen
}

screen_snapshot_test!(
    test_session_detail_screen_snapshot,
    SessionDetailScreen,
    SessionDetailScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>,
        Arc::new(MockSessionRepository::new())
    ),
    pushed_from = {
        let theme_service = Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>;
        let records = RecordsScreen::new(
            Arc::new(EventBus::new()),
            theme_service,
            Arc::new(MockSessionService::new()),
        );
        let data = MockRecordsDataProvider.provide().unwrap();
        records.init_with_data(data).unwrap();
        records.set_selected_session_from_index(0);
        records
    }
);

// Event-producing key tests
#[test]
fn test_session_detail_screen_esc_pops() {
    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = create_initialized_session_detail_screen(event_bus);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 1);
}

#[test]
fn test_session_detail_screen_ctrl_c_exits() {
    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = create_initialized_session_detail_screen(event_bus);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        .unwrap();

    let captured_events = events.lock().unwrap();
    assert_eq!(captured_events.len(), 1);
}

// Non-event key tests
#[test]
fn test_session_detail_screen_up_scrolls() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_initialized_session_detail_screen(event_bus);

    // Should not panic
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_session_detail_screen_down_scrolls() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_initialized_session_detail_screen(event_bus);

    // Should not panic
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();
}
