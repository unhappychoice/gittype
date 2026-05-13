use crate::integration::screens::mocks::trending_repository_mock::MockTrendingRepository;
use crate::integration::screens::mocks::trending_repository_selection_screen_mock::MockTrendingRepositorySelectionDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::TrendingRepositorySelectionScreen;
use gittype::presentation::tui::{Screen, ScreenDataProvider};
use std::sync::{Arc, Mutex};

screen_snapshot_test!(
    test_trending_repository_selection_screen_snapshot,
    TrendingRepositorySelectionScreen,
    TrendingRepositorySelectionScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(MockTrendingRepository::new())
    ),
    provider = MockTrendingRepositorySelectionDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_trending_repository_selection_screen_esc_exits,
    TrendingRepositorySelectionScreen,
    |event_bus| {
        let theme_service = Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>;
        TrendingRepositorySelectionScreen::new(
            event_bus,
            theme_service,
            Arc::new(MockTrendingRepository::new()),
        )
    },
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTrendingRepositorySelectionDataProvider
);

screen_key_event_test!(
    test_trending_repository_selection_screen_ctrl_c_exits,
    TrendingRepositorySelectionScreen,
    |event_bus| {
        let theme_service = Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>;
        TrendingRepositorySelectionScreen::new(
            event_bus,
            theme_service,
            Arc::new(MockTrendingRepository::new()),
        )
    },
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTrendingRepositorySelectionDataProvider
);

screen_key_event_test!(
    test_trending_repository_selection_screen_space_selects,
    TrendingRepositorySelectionScreen,
    |event_bus| {
        let theme_service = Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>;
        TrendingRepositorySelectionScreen::new(
            event_bus,
            theme_service,
            Arc::new(MockTrendingRepository::new()),
        )
    },
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockTrendingRepositorySelectionDataProvider
);

// Non-event key tests
screen_key_tests_custom!(
    TrendingRepositorySelectionScreen,
    |event_bus| {
        let theme_service = Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>;
        TrendingRepositorySelectionScreen::new(
            event_bus,
            theme_service,
            Arc::new(MockTrendingRepository::new()),
        )
    },
    MockTrendingRepositorySelectionDataProvider,
    [
        (
            test_trending_repository_selection_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_trending_repository_selection_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_trending_repository_selection_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_trending_repository_selection_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_trending_repository_selection_screen_basic_methods,
    TrendingRepositorySelectionScreen,
    TrendingRepositorySelectionScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(MockTrendingRepository::new())
    ),
    gittype::presentation::tui::ScreenType::TrendingRepositorySelection,
    true,
    MockTrendingRepositorySelectionDataProvider
);

fn make_screen() -> TrendingRepositorySelectionScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    TrendingRepositorySelectionScreen::new(
        Arc::new(EventBus::new()),
        theme_service,
        Arc::new(MockTrendingRepository::new()),
    )
}

fn init_with_mock_data(screen: &TrendingRepositorySelectionScreen) {
    let data = MockTrendingRepositorySelectionDataProvider
        .provide()
        .unwrap();
    screen.init_with_data(data).unwrap();
}

#[test]
fn test_accessors_start_with_defaults() {
    let screen = make_screen();

    assert_eq!(screen.get_selected_index(), None);
    assert!(screen.get_repositories().is_empty());
}

#[test]
fn test_init_with_data_populates_repositories() {
    let screen = make_screen();

    init_with_mock_data(&screen);

    let repositories = screen.get_repositories();
    assert_eq!(repositories.len(), 2);
    assert_eq!(repositories[0].repo_name, "rust-lang/rust");
    assert_eq!(screen.get_selected_index(), None);
}

#[test]
fn test_init_with_data_ignores_wrong_payload_type() {
    let screen = make_screen();

    screen
        .init_with_data(Box::new(()) as Box<dyn std::any::Any>)
        .unwrap();

    assert!(screen.get_repositories().is_empty());
}

#[test]
fn test_handle_key_event_ignores_release_events() {
    let screen = make_screen();
    init_with_mock_data(&screen);

    let release = KeyEvent {
        code: KeyCode::Down,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Release,
        state: KeyEventState::empty(),
    };
    screen.handle_key_event(release).unwrap();

    assert_eq!(screen.get_selected_index(), None);
}

#[test]
fn test_space_selects_current_index() {
    let event_bus = Arc::new(EventBus::new());
    let observed = Arc::new(Mutex::new(Vec::<NavigateTo>::new()));
    let observed_clone = Arc::clone(&observed);
    event_bus.subscribe(move |event: &NavigateTo| {
        observed_clone.lock().unwrap().push(event.clone());
    });
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let screen = TrendingRepositorySelectionScreen::new(
        event_bus,
        theme_service,
        Arc::new(MockTrendingRepository::new()),
    );
    init_with_mock_data(&screen);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(0));
    assert!(observed
        .lock()
        .unwrap()
        .iter()
        .any(|e| matches!(e, NavigateTo::Exit)));
}

#[test]
fn test_down_navigation_moves_selection() {
    let screen = make_screen();
    init_with_mock_data(&screen);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(1));
}

#[test]
fn test_down_navigation_does_not_pass_last_index() {
    let screen = make_screen();
    init_with_mock_data(&screen);

    for _ in 0..5 {
        screen
            .handle_key_event(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty()))
            .unwrap();
    }
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(1));
}

#[test]
fn test_up_navigation_moves_selection() {
    let screen = make_screen();
    init_with_mock_data(&screen);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()))
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(0));
}

#[test]
fn test_up_navigation_clamps_at_zero() {
    let screen = make_screen();
    init_with_mock_data(&screen);

    for _ in 0..3 {
        screen
            .handle_key_event(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty()))
            .unwrap();
    }
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), Some(0));
}

#[test]
fn test_unhandled_key_does_not_change_state() {
    let event_bus = Arc::new(EventBus::new());
    let observed = Arc::new(Mutex::new(Vec::<NavigateTo>::new()));
    let observed_clone = Arc::clone(&observed);
    event_bus.subscribe(move |event: &NavigateTo| {
        observed_clone.lock().unwrap().push(event.clone());
    });
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let screen = TrendingRepositorySelectionScreen::new(
        event_bus,
        theme_service,
        Arc::new(MockTrendingRepository::new()),
    );
    init_with_mock_data(&screen);

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Tab, KeyModifiers::empty()))
        .unwrap();

    assert_eq!(screen.get_selected_index(), None);
    assert!(observed.lock().unwrap().is_empty());
}

#[test]
fn test_cleanup_returns_ok() {
    let screen = make_screen();
    screen.cleanup().unwrap();
}

#[test]
fn test_as_any_downcasts_to_concrete_type() {
    let screen = make_screen();
    assert!(screen
        .as_any()
        .downcast_ref::<TrendingRepositorySelectionScreen>()
        .is_some());
}

#[test]
fn test_default_provider_returns_unit_payload() {
    let data = <TrendingRepositorySelectionScreen as Screen>::default_provider()
        .provide()
        .unwrap();
    assert!(data.downcast::<()>().is_ok());
}
