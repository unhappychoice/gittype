use crate::integration::screens::mocks::session_summary_screen_mock::{
    MockCompilerDataProvider, MockLoadBalancerPrimarchDataProvider, MockSessionSummaryDataProvider,
};
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::services::session_manager_service::SessionManagerInterface;
use gittype::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use gittype::domain::services::SessionManager;
use gittype::domain::stores::{ChallengeStore, RepositoryStore, SessionStore};
use gittype::domain::stores::{ChallengeStoreInterface, RepositoryStoreInterface, SessionStoreInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::session_summary_screen::SessionSummaryScreen;
use std::sync::Arc;

// Helper function to create SessionSummaryScreen with all required dependencies
fn create_session_summary_screen(event_bus: Arc<dyn EventBusInterface>) -> SessionSummaryScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>;
    let challenge_store = Arc::new(ChallengeStore::new_for_test()) as Arc<dyn ChallengeStoreInterface>;
    let repository_store = Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;
    let session_store = Arc::new(SessionStore::new_for_test()) as Arc<dyn SessionStoreInterface>;
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store,
        repository_store.clone(),
        session_store,
    )) as Arc<dyn StageRepositoryInterface>;
    let session_manager = Arc::new(SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository,
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
