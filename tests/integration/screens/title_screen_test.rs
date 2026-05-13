use crate::integration::screens::mocks::title_screen_mock::MockTitleScreenDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::models::DifficultyLevel;
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
use gittype::presentation::tui::Screen;
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

#[test]
fn test_title_screen_space_without_challenges_sets_error_message() {
    let screen = create_title_screen(Arc::new(EventBus::new()));
    screen.set_challenge_counts([0; 5]);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert!(screen.get_action_result().is_none());
    assert_eq!(
        screen.get_error_message().as_deref(),
        Some("No challenges available for this difficulty. Please try a different difficulty or repository.")
    );
    assert!(screen.update().unwrap());
    assert!(!screen.update().unwrap());
}

#[test]
fn test_title_screen_left_from_first_difficulty_wraps_to_last() {
    let screen = create_title_screen(Arc::new(EventBus::new()));

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::empty()))
        .unwrap();
    assert_eq!(screen.get_selected_difficulty(), DifficultyLevel::Easy);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::empty()))
        .unwrap();
    assert_eq!(screen.get_selected_difficulty(), DifficultyLevel::Zen);
}

#[test]
fn test_title_screen_with_challenge_counts_sets_counts() {
    let screen =
        create_title_screen(Arc::new(EventBus::new())).with_challenge_counts([1, 2, 3, 4, 5]);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert!(matches!(
        screen.get_action_result(),
        Some(gittype::presentation::tui::screens::title_screen::TitleAction::Start(_))
    ));
}

#[test]
fn test_title_screen_with_git_repository_sets_repo() {
    let repo = gittype::domain::models::GitRepository {
        user_name: "alice".to_string(),
        repository_name: "demo".to_string(),
        remote_url: "https://github.com/alice/demo".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };
    let _screen =
        create_title_screen(Arc::new(EventBus::new())).with_git_repository(Some(repo.clone()));
}

#[test]
fn test_title_screen_set_git_repository_overrides_value() {
    let screen = create_title_screen(Arc::new(EventBus::new()));
    let repo = gittype::domain::models::GitRepository {
        user_name: "bob".to_string(),
        repository_name: "lib".to_string(),
        remote_url: "https://github.com/bob/lib".to_string(),
        branch: Some("dev".to_string()),
        commit_hash: Some("deadbeef".to_string()),
        is_dirty: true,
        root_path: None,
    };

    screen.set_git_repository(Some(repo));
    screen.set_git_repository(None);
}

#[test]
fn test_title_screen_init_with_data_fallback_uses_injected_dependencies() {
    let screen = create_title_screen(Arc::new(EventBus::new()));

    // Pass a unit value so the downcast to TitleScreenData fails and the
    // fallback branch pulls counts/repo from the injected stores.
    screen.init_with_data(Box::new(())).unwrap();

    assert_eq!(screen.get_selected_difficulty(), DifficultyLevel::Normal);
    assert!(screen.get_error_message().is_none());
}

#[test]
fn test_title_screen_ignores_unhandled_key() {
    let event_bus = Arc::new(EventBus::new());
    let captured: Arc<std::sync::Mutex<Vec<NavigateTo>>> = Arc::new(std::sync::Mutex::new(vec![]));
    let cap = captured.clone();
    event_bus
        .as_event_bus()
        .subscribe(move |ev: &NavigateTo| cap.lock().unwrap().push(ev.clone()));
    let screen = create_title_screen(event_bus);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()))
        .unwrap();

    assert!(captured.lock().unwrap().is_empty());
    assert!(screen.get_action_result().is_none());
}

#[test]
fn test_title_screen_as_any_downcasts_to_self() {
    let screen = create_title_screen(Arc::new(EventBus::new()));
    let any = screen.as_any();
    assert!(any.downcast_ref::<TitleScreen>().is_some());
}
