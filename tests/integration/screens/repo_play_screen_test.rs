use crate::integration::screens::mocks::repo_play_screen_mock::MockRepoPlayDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::RepoPlayScreen;
use gittype::presentation::tui::{Screen, ScreenDataProvider};
use std::sync::{Arc, Mutex};

screen_snapshot_test!(
    test_repo_play_screen_snapshot,
    RepoPlayScreen,
    RepoPlayScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockRepoPlayDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_repo_play_screen_esc_exits,
    RepoPlayScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockRepoPlayDataProvider
);

screen_key_event_test!(
    test_repo_play_screen_ctrl_c_exits,
    RepoPlayScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockRepoPlayDataProvider
);

screen_key_event_test!(
    test_repo_play_screen_space_selects,
    RepoPlayScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockRepoPlayDataProvider
);

// Non-event key tests
screen_key_tests!(
    RepoPlayScreen,
    MockRepoPlayDataProvider,
    [
        (
            test_repo_play_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_repo_play_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_repo_play_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_repo_play_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_repo_play_screen_basic_methods,
    RepoPlayScreen,
    RepoPlayScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::RepoPlay,
    true,
    MockRepoPlayDataProvider
);

fn make_screen() -> RepoPlayScreen {
    RepoPlayScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    )
}

#[test]
fn test_repo_play_screen_selected_accessors_default_to_none() {
    let screen = make_screen();
    screen
        .init_with_data(MockRepoPlayDataProvider.provide().unwrap())
        .unwrap();

    assert!(screen.get_selected_index().is_none());
    assert!(screen.get_selected_repository().is_none());
}

#[test]
fn test_repo_play_screen_space_sets_selected_repository_for_caller() {
    let screen = make_screen();
    screen
        .init_with_data(MockRepoPlayDataProvider.provide().unwrap())
        .unwrap();

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(0));
    let (repo, cached) = screen
        .get_selected_repository()
        .expect("space should record a selection");
    assert_eq!(repo.user_name, "unhappychoice");
    assert!(!cached);
}

#[test]
fn test_repo_play_screen_down_then_space_selects_second_repository() {
    let screen = make_screen();
    screen
        .init_with_data(MockRepoPlayDataProvider.provide().unwrap())
        .unwrap();

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    let (repo, cached) = screen
        .get_selected_repository()
        .expect("down + space should record a selection");
    assert_eq!(repo.user_name, "rails");
    assert!(cached);
}

#[test]
fn test_repo_play_screen_up_at_first_index_stays_at_zero() {
    let screen = make_screen();
    screen
        .init_with_data(MockRepoPlayDataProvider.provide().unwrap())
        .unwrap();

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(0));
}

#[test]
fn test_repo_play_screen_down_past_end_clamps_to_last_index() {
    let screen = make_screen();
    screen
        .init_with_data(MockRepoPlayDataProvider.provide().unwrap())
        .unwrap();

    for _ in 0..10 {
        screen
            .handle_key_event(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty()))
            .unwrap();
    }
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(2));
}

#[test]
fn test_repo_play_screen_key_release_event_is_ignored() {
    let screen = make_screen();
    screen
        .init_with_data(MockRepoPlayDataProvider.provide().unwrap())
        .unwrap();

    let release_event = KeyEvent {
        code: KeyCode::Char(' '),
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Release,
        state: KeyEventState::empty(),
    };

    screen.handle_key_event(release_event).unwrap();

    assert!(screen.get_selected_index().is_none());
}

#[test]
fn test_repo_play_screen_unhandled_key_does_not_publish_or_select() {
    let event_bus = Arc::new(EventBus::new());
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let captured = Arc::new(Mutex::new(Vec::<NavigateTo>::new()));
    let captured_clone = Arc::clone(&captured);
    event_bus.subscribe(move |event: &NavigateTo| {
        captured_clone.lock().unwrap().push(event.clone());
    });

    let screen = RepoPlayScreen::new(event_bus, theme_service);
    screen
        .init_with_data(MockRepoPlayDataProvider.provide().unwrap())
        .unwrap();

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()))
        .unwrap();

    assert!(captured.lock().unwrap().is_empty());
    assert!(screen.get_selected_index().is_none());
}

#[test]
fn test_repo_play_screen_cleanup_returns_ok() {
    let screen = make_screen();

    assert!(screen.cleanup().is_ok());
}

#[test]
fn test_repo_play_screen_as_any_downcasts_to_concrete_type() {
    let screen = make_screen();

    assert!(screen.as_any().downcast_ref::<RepoPlayScreen>().is_some());
}
