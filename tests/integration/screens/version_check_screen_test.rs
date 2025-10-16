use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::version_check_screen::VersionCheckScreen;

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
    VersionCheckScreen::new(EventBus::new()),
    gittype::presentation::tui::ScreenType::VersionCheck,
    false
);
