use crate::integration::screens::mocks::session_summary_screen_mock::{
    MockCompilerDataProvider, MockLoadBalancerPrimarchDataProvider, MockSessionSummaryDataProvider,
};
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::game::screens::session_summary_screen::SessionSummaryScreen;

screen_snapshot_test!(
    test_session_summary_screen_snapshot,
    SessionSummaryScreen,
    SessionSummaryScreen::new(EventBus::new()),
    provider = MockSessionSummaryDataProvider
);

screen_snapshot_test!(
    test_session_summary_screen_load_balancer_primarch_snapshot,
    SessionSummaryScreen,
    SessionSummaryScreen::new(EventBus::new()),
    provider = MockLoadBalancerPrimarchDataProvider
);

screen_snapshot_test!(
    test_session_summary_screen_compiler_snapshot,
    SessionSummaryScreen,
    SessionSummaryScreen::new(EventBus::new()),
    provider = MockCompilerDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_summary_screen_d_opens_details,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('d'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_d_opens_details,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('D'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_r_retries,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('r'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_r_retries,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('R'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_s_shares,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_s_shares,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_t_goes_to_title,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('t'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_t_goes_to_title,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('T'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_esc_exits,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_ctrl_c_exits,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionSummaryDataProvider
);
