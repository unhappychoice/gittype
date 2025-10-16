use crate::integration::screens::mocks::records_screen_mock::MockRecordsDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::records_screen::RecordsScreen;

screen_snapshot_test!(
    test_records_screen_snapshot_with_mock_data,
    RecordsScreen,
    RecordsScreen::new(EventBus::new()),
    provider = MockRecordsDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_records_screen_esc_navigates_to_title,
    RecordsScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockRecordsDataProvider
);

screen_key_event_test!(
    test_records_screen_ctrl_c_exits,
    RecordsScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockRecordsDataProvider
);

screen_key_event_test!(
    test_records_screen_enter_views_details,
    RecordsScreen,
    NavigateTo,
    KeyCode::Enter,
    KeyModifiers::empty(),
    MockRecordsDataProvider
);

screen_key_event_test!(
    test_records_screen_space_views_details,
    RecordsScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockRecordsDataProvider
);

// Non-event key tests
screen_key_tests!(
    RecordsScreen,
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
    RecordsScreen::new(EventBus::new()),
    gittype::presentation::tui::ScreenType::Records,
    false,
    MockRecordsDataProvider
);
