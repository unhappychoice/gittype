use crate::integration::screens::mocks::title_screen_mock::MockTitleScreenDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::title_screen::TitleScreen;

screen_snapshot_test!(
    test_title_screen_snapshot,
    TitleScreen,
    TitleScreen::new(EventBus::new()),
    provider = MockTitleScreenDataProvider
);
