use crate::integration::screens::mocks::records_screen_mock::MockRecordsDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::records_screen::RecordsScreen;

screen_snapshot_test!(
    test_records_screen_snapshot_with_mock_data,
    RecordsScreen,
    RecordsScreen::new(EventBus::new()),
    provider = MockRecordsDataProvider
);
