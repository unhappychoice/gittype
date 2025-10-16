use crate::integration::screens::mocks::total_summary_share_screen_mock::MockTotalSummaryShareDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::total_summary_share_screen::TotalSummaryShareScreen;

screen_snapshot_test!(
    test_total_summary_share_screen_snapshot,
    TotalSummaryShareScreen,
    TotalSummaryShareScreen::new(EventBus::new()),
    provider = MockTotalSummaryShareDataProvider
);

// Event-producing key tests (when no fallback)
screen_key_event_test!(
    test_total_summary_share_screen_esc_goes_back,
    TotalSummaryShareScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTotalSummaryShareDataProvider
);

screen_key_event_test!(
    test_total_summary_share_screen_ctrl_c_exits,
    TotalSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTotalSummaryShareDataProvider
);

// Non-event key tests (browser opening will fail in test environment)
screen_key_tests!(
    TotalSummaryShareScreen,
    MockTotalSummaryShareDataProvider,
    [
        (
            test_total_summary_share_screen_1_attempts_share,
            KeyCode::Char('1'),
            KeyModifiers::empty()
        ),
        (
            test_total_summary_share_screen_2_attempts_share,
            KeyCode::Char('2'),
            KeyModifiers::empty()
        ),
        (
            test_total_summary_share_screen_3_attempts_share,
            KeyCode::Char('3'),
            KeyModifiers::empty()
        ),
        (
            test_total_summary_share_screen_4_attempts_share,
            KeyCode::Char('4'),
            KeyModifiers::empty()
        ),
    ]
);
