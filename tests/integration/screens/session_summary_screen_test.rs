use crate::integration::screens::mocks::session_summary_screen_mock::MockSessionSummaryDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::session_summary_screen::SessionSummaryScreen;

screen_snapshot_test!(
    test_session_summary_screen_snapshot,
    SessionSummaryScreen,
    SessionSummaryScreen::new(EventBus::new()),
    provider = MockSessionSummaryDataProvider
);
