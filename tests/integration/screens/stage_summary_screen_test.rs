use crate::integration::screens::mocks::stage_summary_screen_mock::MockStageSummaryDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::stage_summary_screen::StageSummaryScreen;

screen_snapshot_test!(
    test_stage_summary_screen_snapshot,
    StageSummaryScreen,
    StageSummaryScreen::new(EventBus::new()),
    provider = MockStageSummaryDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_stage_summary_screen_esc_navigates_to_session_failure,
    StageSummaryScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_ctrl_c_navigates_to_session_failure,
    StageSummaryScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_space_continues,
    StageSummaryScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);
