use crate::integration::screens::mocks::session_summary_share_screen_mock::MockSessionSummaryShareDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::game::screens::session_summary_share_screen::SessionSummaryShareScreen;

screen_snapshot_test!(
    test_session_summary_share_screen_snapshot,
    SessionSummaryShareScreen,
    SessionSummaryShareScreen::new(EventBus::new()),
    provider = MockSessionSummaryShareDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_summary_share_screen_1_shares_to_x,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('1'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_2_shares_to_reddit,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('2'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_3_shares_to_linkedin,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('3'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_4_shares_to_facebook,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('4'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_esc_goes_back,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_ctrl_c_exits,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionSummaryShareDataProvider
);
