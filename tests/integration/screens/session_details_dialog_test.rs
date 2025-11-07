use crate::integration::screens::mocks::session_details_dialog_mock::MockSessionDetailsDialogDataProvider;
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
use gittype::presentation::tui::screens::session_details_dialog::SessionDetailsDialog;
use std::sync::Arc;

// Helper function to create SessionDetailsDialog with all required dependencies
fn create_session_details_dialog(event_bus: Arc<dyn EventBusInterface>) -> SessionDetailsDialog {
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

    SessionDetailsDialog::new(event_bus, theme_service, session_manager, repository_store)
}

screen_snapshot_test!(
    test_session_details_dialog_snapshot,
    SessionDetailsDialog,
    create_session_details_dialog(Arc::new(EventBus::new())),
    provider = MockSessionDetailsDialogDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_details_dialog_esc_closes,
    SessionDetailsDialog,
    create_session_details_dialog,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionDetailsDialogDataProvider
);

screen_key_event_test!(
    test_session_details_dialog_ctrl_c_exits,
    SessionDetailsDialog,
    create_session_details_dialog,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionDetailsDialogDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_details_dialog_basic_methods,
    SessionDetailsDialog,
    create_session_details_dialog(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::DetailsDialog,
    false,
    MockSessionDetailsDialogDataProvider
);
