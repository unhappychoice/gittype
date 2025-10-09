use crate::integration::screens::mocks::total_summary_share_screen_mock::MockTotalSummaryShareDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::total_summary_share_screen::TotalSummaryShareScreen;

screen_snapshot_test!(
    test_total_summary_share_screen_snapshot,
    TotalSummaryShareScreen,
    TotalSummaryShareScreen::new(EventBus::new()),
    provider = MockTotalSummaryShareDataProvider
);
