use crate::integration::screens::mocks::title_screen_mock::MockTitleScreenDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::scoring::{
    SessionTracker, SessionTrackerInterface, TotalTracker, TotalTrackerInterface,
};
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::services::SessionManager;
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use gittype::domain::stores::{
    ChallengeStoreInterface, RepositoryStoreInterface, SessionStoreInterface,
};
use gittype::presentation::tui::screens::title_screen::TitleScreen;
use std::sync::Arc;

// Helper function to create TitleScreen with all required dependencies
fn create_title_screen(event_bus: Arc<dyn EventBusInterface>) -> TitleScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let challenge_store =
        Arc::new(ChallengeStore::new_for_test()) as Arc<dyn ChallengeStoreInterface>;
    let repository_store =
        Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;
    let session_store = Arc::new(SessionStore::new_for_test()) as Arc<dyn SessionStoreInterface>;
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store,
        repository_store.clone(),
        session_store,
    )) as Arc<dyn StageRepositoryInterface>;

    let session_tracker: Arc<dyn SessionTrackerInterface> = Arc::new(SessionTracker::default());
    let total_tracker: Arc<dyn TotalTrackerInterface> = Arc::new(TotalTracker::default());
    let session_manager = SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository.clone(),
        session_tracker,
        total_tracker,
    );
    let session_manager: Arc<dyn SessionManagerInterface> = Arc::new(session_manager);

    TitleScreen::new(
        event_bus,
        theme_service,
        stage_repository,
        repository_store,
        session_manager,
    )
}

screen_snapshot_test!(
    test_title_screen_snapshot,
    TitleScreen,
    create_title_screen(Arc::new(EventBus::new())),
    provider = MockTitleScreenDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_title_screen_space_starts_game,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_esc_exits,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_ctrl_c_exits,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_i_opens_help,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('i'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_question_opens_help,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('?'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_r_opens_records,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('r'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_capital_r_opens_records,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('R'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_a_opens_analytics,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('a'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_capital_a_opens_analytics,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('A'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_s_opens_settings,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

screen_key_event_test!(
    test_title_screen_capital_s_opens_settings,
    TitleScreen,
    create_title_screen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockTitleScreenDataProvider
);

// Non-event key tests
screen_key_tests_custom!(
    TitleScreen,
    create_title_screen,
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
    create_title_screen(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::Title,
    false,
    MockTitleScreenDataProvider
);
