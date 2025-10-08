use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::help_screen::HelpScreen;

screen_snapshot_test!(
    test_help_screen_snapshot_cli,
    HelpScreen,
    HelpScreen::new(EventBus::new())
);

screen_snapshot_test!(
    test_help_screen_snapshot_scoring,
    HelpScreen,
    HelpScreen::new(EventBus::new()),
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

screen_snapshot_test!(
    test_help_screen_snapshot_ranks,
    HelpScreen,
    HelpScreen::new(EventBus::new()),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_help_screen_snapshot_game_help,
    HelpScreen,
    HelpScreen::new(EventBus::new()),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_help_screen_snapshot_community,
    HelpScreen,
    HelpScreen::new(EventBus::new()),
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);
