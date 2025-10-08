use crate::integration::screens::mocks::session_summary_share_screen_mock::MockSessionSummaryShareDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::session_summary_share_screen::SessionSummaryShareScreen;

screen_snapshot_test!(
    test_session_summary_share_screen_snapshot,
    SessionSummaryShareScreen,
    SessionSummaryShareScreen::new(EventBus::new()),
    provider = MockSessionSummaryShareDataProvider
);
