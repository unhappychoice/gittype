use crate::integration::screens::mocks::session_summary_share_screen_mock::MockSessionSummaryShareDataProvider;
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
use gittype::presentation::tui::screens::session_summary_share_screen::SessionSummaryShareScreen;
use gittype::presentation::tui::Screen;
use gittype::GitTypeError;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::sync::{Arc, Mutex};

// Helper function to create SessionSummaryShareScreen with all required dependencies
fn create_session_summary_share_screen(
    event_bus: Arc<dyn EventBusInterface>,
) -> SessionSummaryShareScreen {
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

    SessionSummaryShareScreen::new(event_bus, theme_service, session_manager, repository_store)
}

screen_snapshot_test!(
    test_session_summary_share_screen_snapshot,
    SessionSummaryShareScreen,
    create_session_summary_share_screen(Arc::new(EventBus::new())),
    provider = MockSessionSummaryShareDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_summary_share_screen_1_shares_to_x,
    SessionSummaryShareScreen,
    create_session_summary_share_screen,
    NavigateTo,
    KeyCode::Char('1'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_2_shares_to_reddit,
    SessionSummaryShareScreen,
    create_session_summary_share_screen,
    NavigateTo,
    KeyCode::Char('2'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_3_shares_to_linkedin,
    SessionSummaryShareScreen,
    create_session_summary_share_screen,
    NavigateTo,
    KeyCode::Char('3'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_4_shares_to_facebook,
    SessionSummaryShareScreen,
    create_session_summary_share_screen,
    NavigateTo,
    KeyCode::Char('4'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_esc_goes_back,
    SessionSummaryShareScreen,
    create_session_summary_share_screen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_ctrl_c_exits,
    SessionSummaryShareScreen,
    create_session_summary_share_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionSummaryShareDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_summary_share_screen_basic_methods,
    SessionSummaryShareScreen,
    create_session_summary_share_screen(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::SessionSharing,
    false,
    MockSessionSummaryShareDataProvider
);

#[derive(Debug)]
struct NonConcreteSessionManager;

impl SessionManagerInterface for NonConcreteSessionManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn create_session_summary_share_screen_with_session_manager(
    event_bus: Arc<dyn EventBusInterface>,
    session_manager: Arc<dyn SessionManagerInterface>,
) -> SessionSummaryShareScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let repository_store =
        Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;

    SessionSummaryShareScreen::new(event_bus, theme_service, session_manager, repository_store)
}

#[test]
fn test_session_summary_share_screen_init_without_data_uses_session_manager_fallback() {
    let screen = create_session_summary_share_screen(Arc::new(EventBus::new()));

    // Box<()> is not SessionSummaryShareData, so the fallback branch runs.
    // SessionManager has no active session: get_session_result returns None,
    // and the repository_store returns None.
    screen.init_with_data(Box::new(())).unwrap();
}

#[test]
fn test_session_summary_share_screen_init_without_data_rejects_non_concrete_session_manager() {
    let screen = create_session_summary_share_screen_with_session_manager(
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
fn test_session_summary_share_screen_unhandled_key_does_not_publish_navigation() {
    let event_bus = Arc::new(EventBus::new());
    let published_events = Arc::new(Mutex::new(Vec::<NavigateTo>::new()));
    let observed_events = Arc::clone(&published_events);
    let screen = create_session_summary_share_screen(event_bus.clone());

    event_bus.subscribe(move |event: &NavigateTo| {
        observed_events.lock().unwrap().push(event.clone());
    });

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty()))
        .unwrap();

    assert!(published_events.lock().unwrap().is_empty());
}

#[test]
fn test_session_summary_share_screen_default_provider_returns_unit_data() {
    let data = <SessionSummaryShareScreen as Screen>::default_provider()
        .provide()
        .unwrap();

    assert!(data.downcast::<()>().is_ok());
}

#[test]
fn test_session_summary_share_screen_as_any_downcasts_to_concrete_type() {
    let screen = create_session_summary_share_screen(Arc::new(EventBus::new()));

    assert!(screen
        .as_any()
        .downcast_ref::<SessionSummaryShareScreen>()
        .is_some());
}

#[test]
fn test_session_summary_share_screen_render_without_data_is_noop() {
    let screen = create_session_summary_share_screen(Arc::new(EventBus::new()));
    // No init -> session_result stays None, render path takes the early-return branch.

    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn test_session_summary_share_screen_share_keys_without_session_result_still_publish_pop() {
    // Cover the `if let Some(session_result) = ...` is-None branch for each share key:
    // when no SessionResult is loaded, the share call is skipped but Pop is still published.
    for key in ['1', '2', '3', '4'] {
        let event_bus = Arc::new(EventBus::new());
        let observed = Arc::new(Mutex::new(Vec::<NavigateTo>::new()));
        let observed_clone = Arc::clone(&observed);
        let screen = create_session_summary_share_screen(event_bus.clone());

        event_bus.subscribe(move |event: &NavigateTo| {
            observed_clone.lock().unwrap().push(event.clone());
        });

        // Intentionally skip init_with_data: session_result stays None.
        screen
            .handle_key_event(KeyEvent::new(KeyCode::Char(key), KeyModifiers::empty()))
            .unwrap();

        let events = observed.lock().unwrap();
        assert_eq!(events.len(), 1, "expected one NavigateTo for key '{}'", key);
        assert!(matches!(events[0], NavigateTo::Pop));
    }
}
