use crate::integration::screens::mocks::total_summary_screen_mock::MockTotalSummaryDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::total_summary_screen::TotalSummaryScreen;

screen_snapshot_test!(
    test_total_summary_screen_snapshot,
    TotalSummaryScreen,
    TotalSummaryScreen::new(EventBus::new()),
    provider = MockTotalSummaryDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_total_summary_screen_s_shares,
    TotalSummaryScreen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockTotalSummaryDataProvider
);

screen_key_event_test!(
    test_total_summary_screen_capital_s_shares,
    TotalSummaryScreen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockTotalSummaryDataProvider
);

screen_key_event_test!(
    test_total_summary_screen_esc_exits,
    TotalSummaryScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTotalSummaryDataProvider
);

screen_key_event_test!(
    test_total_summary_screen_ctrl_c_exits,
    TotalSummaryScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTotalSummaryDataProvider
);
