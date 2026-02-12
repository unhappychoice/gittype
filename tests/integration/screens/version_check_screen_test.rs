use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::version_check_screen::VersionCheckScreen;
use std::sync::Arc;

// Event-producing key tests
screen_key_event_test!(
    test_version_check_screen_esc_exits,
    VersionCheckScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_version_check_screen_ctrl_c_exits,
    VersionCheckScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_version_check_screen_basic_methods,
    VersionCheckScreen,
    VersionCheckScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::VersionCheck,
    false
);
