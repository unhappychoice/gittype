use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::help_screen::HelpScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_help_screen_snapshot_cli,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    )
);

screen_snapshot_test!(
    test_help_screen_snapshot_scoring,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

screen_snapshot_test!(
    test_help_screen_snapshot_ranks,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_help_screen_snapshot_game_help,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_help_screen_snapshot_community,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

// Event-producing key tests
use crate::integration::screens::helpers::EmptyMockProvider;

screen_key_event_test!(
    test_help_screen_esc_navigates_back,
    HelpScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_help_screen_ctrl_c_exits,
    HelpScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

// Non-event key tests
screen_key_tests!(
    HelpScreen,
    EmptyMockProvider,
    [
        (
            test_help_screen_left_switches_tab,
            KeyCode::Left,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_h_switches_tab,
            KeyCode::Char('h'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_right_switches_tab,
            KeyCode::Right,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_l_switches_tab,
            KeyCode::Char('l'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_up_scrolls,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_k_scrolls,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_down_scrolls,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_help_screen_j_scrolls,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
        (
            test_help_screen_g_opens_github,
            KeyCode::Char('g'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_help_screen_basic_methods,
    HelpScreen,
    HelpScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::Help,
    false
);
