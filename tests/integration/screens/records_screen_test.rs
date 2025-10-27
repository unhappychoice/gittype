use crate::integration::screens::mocks::records_screen_mock::MockRecordsDataProvider;
use crate::integration::screens::mocks::session_service_mock::MockSessionService;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::records_screen::RecordsScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_records_screen_snapshot_with_mock_data,
    RecordsScreen,
    {
        let event_bus = Arc::new(EventBus::new());
        let session_service = Arc::new(MockSessionService::new());
        RecordsScreen::new(event_bus, session_service)
    },
    provider = MockRecordsDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_records_screen_esc_navigates_to_title,
    RecordsScreen,
    |event_bus| RecordsScreen::new(event_bus, Arc::new(MockSessionService::new())),
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockRecordsDataProvider
);

screen_key_event_test!(
    test_records_screen_ctrl_c_exits,
    RecordsScreen,
    |event_bus| RecordsScreen::new(event_bus, Arc::new(MockSessionService::new())),
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockRecordsDataProvider
);

screen_key_event_test!(
    test_records_screen_enter_views_details,
    RecordsScreen,
    |event_bus| RecordsScreen::new(event_bus, Arc::new(MockSessionService::new())),
    NavigateTo,
    KeyCode::Enter,
    KeyModifiers::empty(),
    MockRecordsDataProvider
);

screen_key_event_test!(
    test_records_screen_space_views_details,
    RecordsScreen,
    |event_bus| RecordsScreen::new(event_bus, Arc::new(MockSessionService::new())),
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockRecordsDataProvider
);

// Non-event key tests
screen_key_tests_custom!(
    RecordsScreen,
    |event_bus| RecordsScreen::new(event_bus, Arc::new(MockSessionService::new())),
    MockRecordsDataProvider,
    [
        (
            test_records_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_records_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_records_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_records_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
        (
            test_records_screen_r_refreshes,
            KeyCode::Char('r'),
            KeyModifiers::empty()
        ),
        (
            test_records_screen_s_sorts,
            KeyCode::Char('s'),
            KeyModifiers::empty()
        ),
        (
            test_records_screen_f_filters,
            KeyCode::Char('f'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_records_screen_basic_methods,
    RecordsScreen,
    {
        let event_bus = Arc::new(EventBus::new());
        let session_service = Arc::new(MockSessionService::new());
        RecordsScreen::new(event_bus, session_service)
    },
    gittype::presentation::tui::ScreenType::Records,
    false,
    MockRecordsDataProvider
);
