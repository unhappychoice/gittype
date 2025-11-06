use crate::integration::screens::mocks::repo_play_screen_mock::MockRepoPlayDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::RepoPlayScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_repo_play_screen_snapshot,
    RepoPlayScreen,
    RepoPlayScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
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

// Basic methods test
screen_basic_methods_test!(
    test_repo_play_screen_basic_methods,
    RepoPlayScreen,
    RepoPlayScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    gittype::presentation::tui::ScreenType::RepoPlay,
    true,
    MockRepoPlayDataProvider
);
