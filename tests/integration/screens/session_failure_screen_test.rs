use crate::integration::screens::mocks::session_failure_screen_mock::MockSessionFailureDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::session_failure_screen::SessionFailureScreen;

screen_snapshot_test!(
    test_session_failure_screen_snapshot,
    SessionFailureScreen,
    SessionFailureScreen::new(EventBus::new()),
    provider = MockSessionFailureDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_failure_screen_r_retries,
    SessionFailureScreen,
    NavigateTo,
    KeyCode::Char('r'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_capital_r_retries,
    SessionFailureScreen,
    NavigateTo,
    KeyCode::Char('R'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_t_goes_to_title,
    SessionFailureScreen,
    NavigateTo,
    KeyCode::Char('t'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_capital_t_goes_to_title,
    SessionFailureScreen,
    NavigateTo,
    KeyCode::Char('T'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_esc_exits,
    SessionFailureScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_ctrl_c_exits,
    SessionFailureScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionFailureDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_failure_screen_basic_methods,
    SessionFailureScreen,
    SessionFailureScreen::new(EventBus::new()),
    gittype::presentation::tui::ScreenType::SessionFailure,
    false,
    MockSessionFailureDataProvider
);
