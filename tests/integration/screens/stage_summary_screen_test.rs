use crate::integration::screens::mocks::stage_summary_screen_mock::MockStageSummaryDataProvider;
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
use gittype::presentation::tui::screens::stage_summary_screen::StageSummaryScreen;
use std::sync::Arc;

// Helper function to create StageSummaryScreen with all required dependencies
fn create_stage_summary_screen(event_bus: Arc<dyn EventBusInterface>) -> StageSummaryScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>;
    let challenge_store = Arc::new(ChallengeStore::new_for_test()) as Arc<dyn ChallengeStoreInterface>;
    let repository_store = Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;
    let session_store = Arc::new(SessionStore::new_for_test()) as Arc<dyn SessionStoreInterface>;
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store,
        repository_store,
        session_store,
    )) as Arc<dyn StageRepositoryInterface>;
    let session_manager = Arc::new(SessionManager::new_with_dependencies(
        event_bus.clone(),
        stage_repository,
    )) as Arc<dyn SessionManagerInterface>;

    StageSummaryScreen::new(event_bus, theme_service, session_manager)
}

screen_snapshot_test!(
    test_stage_summary_screen_snapshot,
    StageSummaryScreen,
    create_stage_summary_screen(Arc::new(EventBus::new())),
    provider = MockStageSummaryDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_stage_summary_screen_esc_navigates_to_session_failure,
    StageSummaryScreen,
    create_stage_summary_screen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_ctrl_c_navigates_to_session_failure,
    StageSummaryScreen,
    create_stage_summary_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_space_continues,
    StageSummaryScreen,
    create_stage_summary_screen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_stage_summary_screen_basic_methods,
    StageSummaryScreen,
    create_stage_summary_screen(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::StageSummary,
    false,
    MockStageSummaryDataProvider
);
