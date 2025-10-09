use crate::integration::screens::mocks::session_failure_screen_mock::MockSessionFailureDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::session_failure_screen::SessionFailureScreen;

screen_snapshot_test!(
    test_session_failure_screen_snapshot,
    SessionFailureScreen,
    SessionFailureScreen::new(EventBus::new()),
    provider = MockSessionFailureDataProvider
);
