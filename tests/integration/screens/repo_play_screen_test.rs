use crate::integration::screens::mocks::repo_play_screen_mock::MockRepoPlayDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::RepoPlayScreen;

screen_snapshot_test!(
    test_repo_play_screen_snapshot,
    RepoPlayScreen,
    RepoPlayScreen::new(EventBus::new()),
    provider = MockRepoPlayDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_repo_play_screen_esc_exits,
    RepoPlayScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockRepoPlayDataProvider
);

screen_key_event_test!(
    test_repo_play_screen_ctrl_c_exits,
    RepoPlayScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockRepoPlayDataProvider
);

screen_key_event_test!(
    test_repo_play_screen_space_selects,
    RepoPlayScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockRepoPlayDataProvider
);

// Non-event key tests
screen_key_tests!(
    RepoPlayScreen,
    MockRepoPlayDataProvider,
    [
        (
            test_repo_play_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_repo_play_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_repo_play_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_repo_play_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);
