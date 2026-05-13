use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::TrendingLanguageSelectionScreen;
use gittype::presentation::tui::Screen;
use std::sync::Arc;

fn make_screen() -> TrendingLanguageSelectionScreen {
    TrendingLanguageSelectionScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    )
}

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::empty())
}

screen_snapshot_test!(
    test_trending_language_selection_screen_snapshot,
    TrendingLanguageSelectionScreen,
    TrendingLanguageSelectionScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    )
);

// Event-producing key tests
screen_key_event_test!(
    test_trending_language_selection_screen_esc_exits,
    TrendingLanguageSelectionScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_trending_language_selection_screen_ctrl_c_exits,
    TrendingLanguageSelectionScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

screen_key_event_test!(
    test_trending_language_selection_screen_space_selects,
    TrendingLanguageSelectionScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    EmptyMockProvider
);

// Non-event key tests
screen_key_tests!(
    TrendingLanguageSelectionScreen,
    EmptyMockProvider,
    [
        (
            test_trending_language_selection_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_trending_language_selection_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_trending_language_selection_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_trending_language_selection_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_trending_language_selection_screen_basic_methods,
    TrendingLanguageSelectionScreen,
    TrendingLanguageSelectionScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::TrendingLanguageSelection,
    true
);

#[test]
fn get_selected_language_returns_none_before_space() {
    let screen = make_screen();
    assert!(screen.get_selected_language().is_none());
}

#[test]
fn space_press_sets_selected_language_to_first_entry() {
    let screen = make_screen();
    screen.handle_key_event(press(KeyCode::Char(' '))).unwrap();
    assert_eq!(screen.get_selected_language(), Some("C".to_string()));
}

#[test]
fn key_release_event_is_ignored() {
    let screen = make_screen();
    let release_event = KeyEvent {
        code: KeyCode::Char(' '),
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Release,
        state: KeyEventState::empty(),
    };
    screen.handle_key_event(release_event).unwrap();
    assert!(screen.get_selected_language().is_none());
}

#[test]
fn up_at_first_index_does_not_underflow() {
    let screen = make_screen();
    screen.handle_key_event(press(KeyCode::Up)).unwrap();
    screen.handle_key_event(press(KeyCode::Char(' '))).unwrap();
    assert_eq!(screen.get_selected_language(), Some("C".to_string()));
}

#[test]
fn k_after_j_decrements_selection() {
    let screen = make_screen();
    screen.handle_key_event(press(KeyCode::Char('j'))).unwrap();
    screen.handle_key_event(press(KeyCode::Char('j'))).unwrap();
    screen.handle_key_event(press(KeyCode::Char('k'))).unwrap();
    screen.handle_key_event(press(KeyCode::Char(' '))).unwrap();
    assert_eq!(screen.get_selected_language(), Some("C#".to_string()));
}

#[test]
fn down_at_last_index_does_not_overflow() {
    let screen = make_screen();
    for _ in 0..30 {
        screen.handle_key_event(press(KeyCode::Char('j'))).unwrap();
    }
    screen.handle_key_event(press(KeyCode::Char(' '))).unwrap();
    assert_eq!(
        screen.get_selected_language(),
        Some("TypeScript".to_string())
    );
}

#[test]
fn unhandled_key_is_a_noop() {
    let event_bus = Arc::new(EventBus::new());
    let captured: Arc<std::sync::Mutex<Vec<NavigateTo>>> =
        Arc::new(std::sync::Mutex::new(Vec::new()));
    let captured_clone = captured.clone();
    event_bus.subscribe(move |evt: &NavigateTo| {
        captured_clone.lock().unwrap().push(evt.clone());
    });

    let screen = TrendingLanguageSelectionScreen::new(
        event_bus,
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    screen.handle_key_event(press(KeyCode::Tab)).unwrap();
    screen.handle_key_event(press(KeyCode::Char('z'))).unwrap();

    assert!(captured.lock().unwrap().is_empty());
    assert!(screen.get_selected_language().is_none());
}

#[test]
fn cleanup_returns_ok() {
    let screen = make_screen();
    assert!(screen.cleanup().is_ok());
}

#[test]
fn as_any_downcasts_to_concrete_screen() {
    let screen = make_screen();
    let any_ref = screen.as_any();
    assert!(any_ref
        .downcast_ref::<TrendingLanguageSelectionScreen>()
        .is_some());
}

#[test]
fn init_with_data_resets_state() {
    let screen = make_screen();
    screen.handle_key_event(press(KeyCode::Char('j'))).unwrap();
    screen.handle_key_event(press(KeyCode::Char(' '))).unwrap();
    assert!(screen.get_selected_language().is_some());

    screen.init_with_data(Box::new(())).unwrap();
    assert!(screen.get_selected_language().is_none());
}

#[test]
fn default_provider_yields_unit_payload() {
    let payload = TrendingLanguageSelectionScreen::default_provider()
        .provide()
        .unwrap();
    assert!(payload.downcast_ref::<()>().is_some());
}
