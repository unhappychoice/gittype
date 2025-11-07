use crate::integration::screens::mocks::animation_screen_mock::MockAnimationDataProvider;
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
use gittype::presentation::tui::screens::animation_screen::AnimationScreen;
use std::sync::Arc;

// Helper function to create AnimationScreen with all required dependencies
fn create_animation_screen(event_bus: Arc<dyn EventBusInterface>) -> AnimationScreen {
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

    AnimationScreen::new(event_bus, theme_service, session_manager)
}

screen_snapshot_test!(
    test_animation_screen_snapshot_with_session_result,
    AnimationScreen,
    create_animation_screen(Arc::new(EventBus::new())),
    provider = MockAnimationDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_animation_screen_s_skips,
    AnimationScreen,
    create_animation_screen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockAnimationDataProvider
);

screen_key_event_test!(
    test_animation_screen_capital_s_skips,
    AnimationScreen,
    create_animation_screen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockAnimationDataProvider
);

screen_key_event_test!(
    test_animation_screen_ctrl_c_exits,
    AnimationScreen,
    create_animation_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockAnimationDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_animation_screen_basic_methods,
    AnimationScreen,
    create_animation_screen(Arc::new(EventBus::new())),
    gittype::presentation::tui::ScreenType::Animation,
    false,
    MockAnimationDataProvider
);
