use crate::integration::screens::mocks::total_summary_screen_mock::MockTotalSummaryDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::total_summary_screen::TotalSummaryScreen;

screen_snapshot_test!(
    test_total_summary_screen_snapshot,
    TotalSummaryScreen,
    TotalSummaryScreen::new(EventBus::new()),
    provider = MockTotalSummaryDataProvider
);
