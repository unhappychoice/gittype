use crate::integration::screens::mocks::stage_summary_screen_mock::MockStageSummaryDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::stage_summary_screen::StageSummaryScreen;

screen_snapshot_test!(
    test_stage_summary_screen_snapshot,
    StageSummaryScreen,
    StageSummaryScreen::new(EventBus::new()),
    provider = MockStageSummaryDataProvider
);
