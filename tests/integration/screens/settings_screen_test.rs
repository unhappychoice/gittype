use crate::integration::screens::mocks::settings_screen_mock::MockSettingsScreenDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::settings_screen::SettingsScreen;

screen_snapshot_test!(
    test_settings_screen_snapshot_color_mode,
    SettingsScreen,
    SettingsScreen::new(EventBus::new()),
    provider = MockSettingsScreenDataProvider
);

screen_snapshot_test!(
    test_settings_screen_snapshot_theme,
    SettingsScreen,
    SettingsScreen::new(EventBus::new()),
    provider = MockSettingsScreenDataProvider,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);
