use crate::integration::screens::mocks::settings_screen_mock::MockSettingsScreenDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::settings_screen::SettingsScreen;

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

// Event-producing key tests
screen_key_event_test!(
    test_settings_screen_space_saves_and_navigates_back,
    SettingsScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockSettingsScreenDataProvider
);

screen_key_event_test!(
    test_settings_screen_esc_cancels_and_navigates_back,
    SettingsScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSettingsScreenDataProvider
);

screen_key_event_test!(
    test_settings_screen_ctrl_c_exits,
    SettingsScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSettingsScreenDataProvider
);

// Non-event key tests
screen_key_tests!(
    SettingsScreen,
    MockSettingsScreenDataProvider,
    [
        (
            test_settings_screen_key_left_switches_tab,
            KeyCode::Left,
            KeyModifiers::empty()
        ),
        (
            test_settings_screen_key_h_switches_tab,
            KeyCode::Char('h'),
            KeyModifiers::empty()
        ),
        (
            test_settings_screen_key_right_switches_tab,
            KeyCode::Right,
            KeyModifiers::empty()
        ),
        (
            test_settings_screen_key_l_switches_tab,
            KeyCode::Char('l'),
            KeyModifiers::empty()
        ),
        (
            test_settings_screen_key_up_navigates_list,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_settings_screen_key_k_navigates_list,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_settings_screen_key_down_navigates_list,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_settings_screen_key_j_navigates_list,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);
