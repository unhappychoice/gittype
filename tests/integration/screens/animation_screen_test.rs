use crate::integration::screens::mocks::animation_screen_mock::MockAnimationDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::animation_screen::AnimationScreen;

screen_snapshot_test!(
    test_animation_screen_snapshot_with_session_result,
    AnimationScreen,
    AnimationScreen::new(EventBus::new()),
    provider = MockAnimationDataProvider
);
