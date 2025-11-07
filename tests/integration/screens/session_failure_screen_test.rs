use crate::integration::screens::mocks::session_failure_screen_mock::MockSessionFailureDataProvider;
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
use gittype::presentation::tui::screens::session_failure_screen::SessionFailureScreen;
use std::sync::Arc;

// Helper function to create SessionFailureScreen with all required dependencies
fn create_session_failure_screen(event_bus: Arc<dyn EventBusInterface>) -> SessionFailureScreen {
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
    let session_manager = Arc::new(SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository,
        session_tracker,
        total_tracker,
    )) as Arc<dyn SessionManagerInterface>;

    SessionFailureScreen::new(event_bus, theme_service, session_manager, repository_store)
}

screen_snapshot_test!(
    test_session_failure_screen_snapshot,
    SessionFailureScreen,
    create_session_failure_screen(Arc::new(EventBus::new())),
    provider = MockSessionFailureDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_failure_screen_r_retries,
    SessionFailureScreen,
    create_session_failure_screen,
    NavigateTo,
    KeyCode::Char('r'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_capital_r_retries,
    SessionFailureScreen,
    create_session_failure_screen,
    NavigateTo,
    KeyCode::Char('R'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_t_goes_to_title,
    SessionFailureScreen,
    create_session_failure_screen,
    NavigateTo,
    KeyCode::Char('t'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_capital_t_goes_to_title,
    SessionFailureScreen,
    create_session_failure_screen,
    NavigateTo,
    KeyCode::Char('T'),
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_esc_exits,
    SessionFailureScreen,
    create_session_failure_screen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionFailureDataProvider
);

screen_key_event_test!(
    test_session_failure_screen_ctrl_c_exits,
    SessionFailureScreen,
    create_session_failure_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionFailureDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_failure_screen_basic_methods,
    SessionFailureScreen,
    create_session_failure_screen(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::SessionFailure,
    false,
    MockSessionFailureDataProvider
);
