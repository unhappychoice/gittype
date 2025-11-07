use crate::integration::screens::mocks::repo_list_screen_mock::MockRepoListDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::RepoListScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_repo_list_screen_snapshot,
    RepoListScreen,
    RepoListScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
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
    RepoListScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::RepoList,
    true,
    MockRepoListDataProvider
);
