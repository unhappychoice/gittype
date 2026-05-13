use crate::integration::screens::mocks::records_screen_mock::MockRecordsDataProvider;
use crate::integration::screens::mocks::session_repository_mock::MockSessionRepository;
use crate::integration::screens::mocks::session_service_mock::MockSessionService;
use chrono::{TimeZone, Utc};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::storage::{SessionResultData, StoredSession};
use gittype::domain::models::theme::Theme;
use gittype::domain::services::session_service::SessionDisplayData;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::records_screen::RecordsScreenData;
use gittype::presentation::tui::screens::{RecordsScreen, SessionDetailScreen};
use gittype::presentation::tui::ScreenDataProvider;
use gittype::presentation::tui::{Screen, ScreenType, UpdateStrategy};
use gittype::GitTypeError;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::sync::{Arc, Mutex};

// Helper function to create and initialize SessionDetailScreen from RecordsScreen
fn create_initialized_session_detail_screen(
    event_bus: Arc<dyn EventBusInterface>,
) -> SessionDetailScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let screen = SessionDetailScreen::new(
        event_bus.clone(),
        theme_service.clone(),
        Arc::new(MockSessionRepository::new()),
    );

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
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(MockSessionRepository::new())
    ),
    pushed_from = {
        let theme_service = Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>;
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

#[test]
fn test_session_detail_screen_default_provider_returns_unit_data() {
    let data = <SessionDetailScreen as Screen>::default_provider()
        .provide()
        .unwrap();

    assert!(data.downcast::<()>().is_ok());
}

#[test]
fn test_session_detail_screen_rejects_non_records_source_screen() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_initialized_session_detail_screen(event_bus);

    let result = screen.on_pushed_from(&screen);

    assert!(matches!(
        result,
        Err(GitTypeError::ScreenInitializationError(message))
            if message == "SessionDetail must be pushed from Records screen"
    ));
}

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

fn build_records_screen_with(sessions: Vec<SessionDisplayData>) -> RecordsScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let records = RecordsScreen::new(
        Arc::new(EventBus::new()),
        theme_service,
        Arc::new(MockSessionService::new()),
    );
    let data: Box<dyn std::any::Any> = Box::new(RecordsScreenData {
        sessions,
        repositories: Vec::new(),
    });
    records.init_with_data(data).unwrap();
    records
}

fn session_with_id(id: i64) -> SessionDisplayData {
    SessionDisplayData {
        session: StoredSession {
            id,
            repository_id: None,
            started_at: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            completed_at: None,
            branch: None,
            commit_hash: None,
            is_dirty: false,
            game_mode: "default".to_string(),
            difficulty_level: None,
            max_stages: Some(1),
            time_limit_seconds: None,
        },
        repository: None,
        session_result: Some(SessionResultData {
            keystrokes: 0,
            mistakes: 0,
            duration_ms: 0,
            wpm: 0.0,
            cpm: 0.0,
            accuracy: 0.0,
            stages_completed: 0,
            stages_attempted: 0,
            stages_skipped: 0,
            score: 0.0,
            rank_name: None,
            tier_name: None,
            rank_position: None,
            rank_total: None,
            position: None,
            total: None,
        }),
    }
}

fn make_screen() -> SessionDetailScreen {
    SessionDetailScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(MockSessionRepository::new()),
    )
}

#[test]
fn test_session_detail_screen_rejects_records_without_selected_session() {
    let screen = make_screen();
    let records = build_records_screen_with(vec![session_with_id(1)]);
    // No `set_selected_session_from_index` call → `get_selected_session_for_detail` returns None.

    let result = screen.on_pushed_from(&records);

    assert!(matches!(
        result,
        Err(GitTypeError::ScreenInitializationError(message))
            if message == "SessionDetail requires selected session data from Records screen"
    ));
}

#[test]
fn test_session_detail_screen_rejects_session_with_id_zero() {
    let screen = make_screen();
    let records = build_records_screen_with(vec![session_with_id(0)]);
    records.set_selected_session_from_index(0);

    let result = screen.on_pushed_from(&records);

    assert!(matches!(
        result,
        Err(GitTypeError::ScreenInitializationError(message))
            if message == "SessionDetail: session id cannot be 0"
    ));
}

#[test]
fn test_session_detail_screen_up_after_down_scrolls_back() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_initialized_session_detail_screen(event_bus);

    // Scroll down to a non-zero offset, then back up to exercise both
    // branches of the Up handler (the `*offset > 0` true branch in particular).
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()))
        .unwrap();
}

#[test]
fn test_session_detail_screen_down_stops_at_last_stage() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_initialized_session_detail_screen(event_bus);

    // Mock returns 3 stage results. Pressing Down four times forces the
    // saturating no-op branch (`*offset + 1 < stage_results.len()` is false).
    for _ in 0..4 {
        screen
            .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
            .unwrap();
    }
}

#[test]
fn test_session_detail_screen_ignores_unhandled_key() {
    let event_bus = Arc::new(EventBus::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);

    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = create_initialized_session_detail_screen(event_bus);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty()))
        .unwrap();

    assert!(events.lock().unwrap().is_empty());
}

#[test]
fn test_session_detail_screen_init_with_data_accepts_any_payload() {
    let screen = make_screen();
    assert!(screen.init_with_data(Box::new(())).is_ok());
    assert!(screen.init_with_data(Box::new(42i64)).is_ok());
}

#[test]
fn test_session_detail_screen_basic_screen_methods() {
    let screen = make_screen();

    assert_eq!(screen.get_type(), ScreenType::SessionDetail);
    assert!(matches!(
        screen.get_update_strategy(),
        UpdateStrategy::InputOnly
    ));
    assert!(!screen.update().unwrap());
    assert!(screen.cleanup().is_ok());
    assert!(screen
        .as_any()
        .downcast_ref::<SessionDetailScreen>()
        .is_some());
}

#[test]
fn test_session_detail_screen_render_after_scroll_keys() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_initialized_session_detail_screen(event_bus);

    // Scroll the stage details panel a bit before rendering so the render
    // path picks up a non-zero `stage_scroll_offset`.
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut rendered = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            rendered.push_str(buffer[(x, y)].symbol());
        }
        rendered.push('\n');
    }
    assert!(rendered.contains("Session Details"));
}
