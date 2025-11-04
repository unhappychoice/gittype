use crate::integration::screens::mocks::repo_list_screen_mock::MockRepoListDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::RepoListScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_repo_list_screen_snapshot,
    RepoListScreen,
    RepoListScreen::new(Arc::new(EventBus::new())),
    provider = MockRepoListDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_repo_list_screen_esc_exits,
    RepoListScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockRepoListDataProvider
);

screen_key_event_test!(
    test_repo_list_screen_ctrl_c_exits,
    RepoListScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockRepoListDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_repo_list_screen_basic_methods,
    RepoListScreen,
    RepoListScreen::new(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::RepoList,
    true,
    MockRepoListDataProvider
);
