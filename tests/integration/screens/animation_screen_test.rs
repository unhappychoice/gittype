use crate::integration::screens::mocks::animation_screen_mock::MockAnimationDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::animation_screen::AnimationScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_animation_screen_snapshot_with_session_result,
    AnimationScreen,
    AnimationScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockAnimationDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_animation_screen_s_skips,
    AnimationScreen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockAnimationDataProvider
);

screen_key_event_test!(
    test_animation_screen_capital_s_skips,
    AnimationScreen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockAnimationDataProvider
);

screen_key_event_test!(
    test_animation_screen_ctrl_c_exits,
    AnimationScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockAnimationDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_animation_screen_basic_methods,
    AnimationScreen,
    AnimationScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    gittype::presentation::tui::ScreenType::Animation,
    false,
    MockAnimationDataProvider
);
