use crate::integration::screens::mocks::records_screen_mock::MockRecordsDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::models::ScreenDataProvider;
use gittype::presentation::game::screens::{RecordsScreen, SessionDetailScreen};

screen_snapshot_test!(
    test_session_detail_screen_snapshot,
    SessionDetailScreen,
    SessionDetailScreen::new(EventBus::new()),
    pushed_from = {
        let mut records = RecordsScreen::new(EventBus::new());
        let data = MockRecordsDataProvider.provide().unwrap();
        records.init_with_data(data).unwrap();
        records.set_selected_session_from_index(0);
        records
    }
);
