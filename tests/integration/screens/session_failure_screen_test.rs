use crate::integration::screens::mocks::session_failure_screen_mock::MockSessionFailureDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
use gittype::presentation::tui::Screen;
use gittype::GitTypeError;
use std::sync::{Arc, Mutex};

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

#[derive(Debug)]
struct NonConcreteSessionManager;

impl SessionManagerInterface for NonConcreteSessionManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn create_session_failure_screen_with_session_manager(
    event_bus: Arc<dyn EventBusInterface>,
    session_manager: Arc<dyn SessionManagerInterface>,
) -> SessionFailureScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let repository_store =
        Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;

    SessionFailureScreen::new(event_bus, theme_service, session_manager, repository_store)
}

#[test]
fn test_session_failure_screen_init_without_data_uses_session_manager_fallback() {
    let screen = create_session_failure_screen(Arc::new(EventBus::new()));

    // Box<()> is not SessionFailureScreenData, so the fallback branch runs.
    // SessionManager has no active session: get_session_result returns None
    // (defaults applied) and get_stage_info returns (0, max_stages).
    screen.init_with_data(Box::new(())).unwrap();
}

#[test]
fn test_session_failure_screen_init_without_data_rejects_non_concrete_session_manager() {
    let screen = create_session_failure_screen_with_session_manager(
        Arc::new(EventBus::new()),
        Arc::new(NonConcreteSessionManager),
    );

    let error = screen.init_with_data(Box::new(())).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::TerminalError(message)
            if message == "Failed to get SessionManager"
    ));
}

#[test]
fn test_session_failure_screen_unhandled_key_does_not_publish_navigation() {
    let event_bus = Arc::new(EventBus::new());
    let published_events = Arc::new(Mutex::new(Vec::<NavigateTo>::new()));
    let observed_events = Arc::clone(&published_events);
    let screen = create_session_failure_screen(event_bus.clone());

    event_bus.subscribe(move |event: &NavigateTo| {
        observed_events.lock().unwrap().push(event.clone());
    });

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty()))
        .unwrap();

    assert!(published_events.lock().unwrap().is_empty());
}

#[test]
fn test_session_failure_screen_default_provider_returns_unit_data() {
    let data = <SessionFailureScreen as Screen>::default_provider()
        .provide()
        .unwrap();

    assert!(data.downcast::<()>().is_ok());
}

#[test]
fn test_session_failure_screen_as_any_downcasts_to_concrete_type() {
    let screen = create_session_failure_screen(Arc::new(EventBus::new()));

    assert!(screen
        .as_any()
        .downcast_ref::<SessionFailureScreen>()
        .is_some());
}
