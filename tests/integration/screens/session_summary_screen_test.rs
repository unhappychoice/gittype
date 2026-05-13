use crate::integration::screens::mocks::session_summary_screen_mock::{
    MockCompilerDataProvider, MockLoadBalancerPrimarchDataProvider, MockSessionSummaryDataProvider,
};
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
use gittype::presentation::tui::screens::session_summary_screen::{
    ResultAction, SessionSummaryScreen,
};
use gittype::presentation::tui::Screen;
use std::sync::Arc;

// Helper function to create SessionSummaryScreen with all required dependencies
fn create_session_summary_screen(event_bus: Arc<dyn EventBusInterface>) -> SessionSummaryScreen {
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

    SessionSummaryScreen::new(event_bus, theme_service, session_manager, repository_store)
}

screen_snapshot_test!(
    test_session_summary_screen_snapshot,
    SessionSummaryScreen,
    create_session_summary_screen(Arc::new(EventBus::new())),
    provider = MockSessionSummaryDataProvider
);

screen_snapshot_test!(
    test_session_summary_screen_load_balancer_primarch_snapshot,
    SessionSummaryScreen,
    create_session_summary_screen(Arc::new(EventBus::new())),
    provider = MockLoadBalancerPrimarchDataProvider
);

screen_snapshot_test!(
    test_session_summary_screen_compiler_snapshot,
    SessionSummaryScreen,
    create_session_summary_screen(Arc::new(EventBus::new())),
    provider = MockCompilerDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_summary_screen_d_opens_details,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('d'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_d_opens_details,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('D'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_r_retries,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('r'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_r_retries,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('R'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_s_shares,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_s_shares,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_t_goes_to_title,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('t'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_t_goes_to_title,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('T'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_esc_exits,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_ctrl_c_exits,
    SessionSummaryScreen,
    create_session_summary_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionSummaryDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_summary_screen_basic_methods,
    SessionSummaryScreen,
    create_session_summary_screen(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::SessionSummary,
    false,
    MockSessionSummaryDataProvider
);

#[test]
fn test_session_summary_screen_get_action_result_initially_none() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    assert!(screen.get_action_result().is_none());
}

#[test]
fn test_session_summary_screen_retry_key_sets_action_result_to_retry() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::empty()))
        .unwrap();
    assert!(matches!(
        screen.get_action_result(),
        Some(ResultAction::Retry)
    ));
}

#[test]
fn test_session_summary_screen_share_key_sets_action_result_to_share() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('S'), KeyModifiers::empty()))
        .unwrap();
    assert!(matches!(
        screen.get_action_result(),
        Some(ResultAction::Share)
    ));
}

#[test]
fn test_session_summary_screen_t_key_sets_action_result_to_back_to_title() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('T'), KeyModifiers::empty()))
        .unwrap();
    assert!(matches!(
        screen.get_action_result(),
        Some(ResultAction::BackToTitle)
    ));
}

#[test]
fn test_session_summary_screen_esc_sets_action_result_to_quit() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();
    assert!(matches!(
        screen.get_action_result(),
        Some(ResultAction::Quit)
    ));
}

#[test]
fn test_session_summary_screen_ctrl_c_sets_action_result_to_quit() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        .unwrap();
    assert!(matches!(
        screen.get_action_result(),
        Some(ResultAction::Quit)
    ));
}

#[test]
fn test_session_summary_screen_d_key_does_not_set_action_result() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()))
        .unwrap();
    // Pressing 'd' opens the details dialog but does not record a ResultAction.
    assert!(screen.get_action_result().is_none());
}

#[test]
fn test_session_summary_screen_unknown_key_is_a_noop() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty()))
        .unwrap();
    assert!(screen.get_action_result().is_none());
}

#[test]
fn test_session_summary_screen_init_with_data_falls_back_to_session_manager_when_box_does_not_downcast(
) {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));

    // The default provider returns `Box<dyn Any>` containing `()`, so the
    // downcast to `SessionSummaryScreenData` fails and the fallback branch
    // queries the injected `SessionManager` / `RepositoryStore` instead.
    let data = <SessionSummaryScreen as Screen>::default_provider()
        .provide()
        .unwrap();
    assert!(screen.init_with_data(data).is_ok());
}

#[test]
fn test_session_summary_screen_default_provider_returns_unit_data() {
    let data = <SessionSummaryScreen as Screen>::default_provider()
        .provide()
        .unwrap();
    assert!(data.downcast::<()>().is_ok());
}

#[test]
fn test_session_summary_screen_init_with_data_resets_previous_action_result() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::empty()))
        .unwrap();
    assert!(matches!(
        screen.get_action_result(),
        Some(ResultAction::Retry)
    ));

    let data = <SessionSummaryScreen as Screen>::default_provider()
        .provide()
        .unwrap();
    screen.init_with_data(data).unwrap();

    assert!(screen.get_action_result().is_none());
}

#[test]
fn test_session_summary_screen_as_any_downcasts_to_concrete_type() {
    let screen = create_session_summary_screen(Arc::new(EventBus::new()));
    assert!(screen
        .as_any()
        .downcast_ref::<SessionSummaryScreen>()
        .is_some());
}

#[test]
fn test_session_summary_screen_result_action_variants_are_clonable_and_debuggable() {
    let actions = [
        ResultAction::Restart,
        ResultAction::BackToTitle,
        ResultAction::Quit,
        ResultAction::Retry,
        ResultAction::Share,
    ];
    for action in actions.iter() {
        let cloned = action.clone();
        // Debug + Clone exist on ResultAction; exercise both to lock in the
        // public contract used by callers that log the chosen action.
        assert!(!format!("{:?}", cloned).is_empty());
    }
}
