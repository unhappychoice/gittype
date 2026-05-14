use crate::integration::screens::mocks::animation_screen_mock::MockAnimationDataProvider;
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
use gittype::presentation::tui::screens::animation_screen::{
    AnimationDataProvider, AnimationScreen,
};
use gittype::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::sync::{Arc, Mutex};

// Helper function to create AnimationScreen with all required dependencies
fn create_animation_screen(event_bus: Arc<dyn EventBusInterface>) -> AnimationScreen {
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
        repository_store,
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

// A SessionManagerInterface impl that is not SessionManager, so the screen's
// downcast_ref::<SessionManager>() in init_with_data() fails.
struct FakeSessionManager;

impl SessionManagerInterface for FakeSessionManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn create_animation_screen_with_fake_session_manager(
    event_bus: Arc<dyn EventBusInterface>,
) -> AnimationScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let session_manager = Arc::new(FakeSessionManager) as Arc<dyn SessionManagerInterface>;
    AnimationScreen::new(event_bus, theme_service, session_manager)
}

#[test]
fn animation_data_provider_provides_unit_value() {
    let provider = AnimationDataProvider;
    let any_data = provider.provide().unwrap();
    assert!(any_data.downcast::<()>().is_ok());
}

#[test]
fn init_with_data_falls_back_to_session_manager_when_data_is_unit() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen(event_bus);

    let result = screen.init_with_data(Box::new(()));

    assert!(result.is_ok());
    // After fallback through SessionManager, animation should be initialized.
    assert!(!screen.is_animation_complete());
}

#[test]
fn init_with_data_returns_err_when_session_manager_is_not_real_session_manager() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen_with_fake_session_manager(event_bus);

    let result = screen.init_with_data(Box::new(()));

    assert!(result.is_err());
}

#[test]
fn is_animation_complete_returns_false_when_animation_not_set() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen(event_bus);

    assert!(!screen.is_animation_complete());
}

#[test]
fn update_returns_false_when_no_animation_set() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen(event_bus);

    let updated = screen.update().unwrap();
    assert!(!updated);
}

#[test]
fn handle_key_event_unrelated_key_publishes_no_event() {
    let event_bus = Arc::new(EventBus::new());
    let events: Arc<Mutex<Vec<NavigateTo>>> = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);
    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = create_animation_screen(event_bus);
    let data = MockAnimationDataProvider.provide().unwrap();
    screen.init_with_data(data).unwrap();

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty()))
        .unwrap();

    assert!(events.lock().unwrap().is_empty());
}

#[test]
fn handle_key_event_plain_c_without_ctrl_publishes_no_event() {
    let event_bus = Arc::new(EventBus::new());
    let events: Arc<Mutex<Vec<NavigateTo>>> = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);
    event_bus.subscribe(move |event: &NavigateTo| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let screen = create_animation_screen(event_bus);
    let data = MockAnimationDataProvider.provide().unwrap();
    screen.init_with_data(data).unwrap();

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::empty()))
        .unwrap();

    assert!(events.lock().unwrap().is_empty());
}

#[test]
fn render_ratatui_succeeds_without_initialization() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen(event_bus);

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            screen.render_ratatui(frame).unwrap();
        })
        .unwrap();
}

#[test]
fn get_type_returns_animation_screen_type() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen(event_bus);
    assert_eq!(screen.get_type(), ScreenType::Animation);
}

#[test]
fn as_any_downcasts_to_animation_screen() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen(event_bus);

    assert!(screen.as_any().downcast_ref::<AnimationScreen>().is_some());
}

#[test]
fn get_update_strategy_is_time_based() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_animation_screen(event_bus);
    assert!(matches!(
        screen.get_update_strategy(),
        UpdateStrategy::TimeBased(_)
    ));
}

#[test]
fn default_provider_returns_animation_data_provider() {
    let provider = <AnimationScreen as Screen>::default_provider();
    let any_data = provider.provide().unwrap();
    assert!(any_data.downcast::<()>().is_ok());
}
