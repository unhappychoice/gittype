use crate::integration::screens::mocks::title_screen_mock::MockTitleScreenDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::title_screen::TitleScreen;

screen_snapshot_test!(
    test_title_screen_snapshot,
    TitleScreen,
    TitleScreen::new(EventBus::new()),
    provider = MockTitleScreenDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_title_screen_space_starts_game,
    TitleScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_esc_exits,
    TitleScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_ctrl_c_exits,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_i_opens_help,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('i'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_question_opens_help,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('?'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_r_opens_records,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('r'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_capital_r_opens_records,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('R'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_a_opens_analytics,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('a'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_capital_a_opens_analytics,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('A'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_s_opens_settings,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_capital_s_opens_settings,
    TitleScreen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

// Non-event key tests
screen_key_tests!(
    TitleScreen,
    MockTitleScreenDataProvider,
    [
        (
            test_title_screen_left_changes_difficulty,
            KeyCode::Left,
            KeyModifiers::empty()
        ),
        (
            test_title_screen_h_changes_difficulty,
            KeyCode::Char('h'),
            KeyModifiers::empty()
        ),
        (
            test_title_screen_right_changes_difficulty,
            KeyCode::Right,
            KeyModifiers::empty()
        ),
        (
            test_title_screen_l_changes_difficulty,
            KeyCode::Char('l'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_title_screen_basic_methods,
    TitleScreen,
    TitleScreen::new(EventBus::new()),
    gittype::presentation::tui::ScreenType::Title,
    false,
    MockTitleScreenDataProvider
);
