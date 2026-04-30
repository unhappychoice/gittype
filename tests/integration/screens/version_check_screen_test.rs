use crate::integration::screens::helpers::EmptyMockProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::version_check_screen::VersionCheckScreen;
use gittype::presentation::tui::Screen;
use std::sync::{Arc, Mutex};

// Event-producing key tests
screen_key_event_test!(
    test_version_check_screen_esc_exits,
    VersionCheckScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    EmptyMockProvider
);

screen_key_event_test!(
    test_version_check_screen_ctrl_c_exits,
    VersionCheckScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    EmptyMockProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_version_check_screen_basic_methods,
    VersionCheckScreen,
    VersionCheckScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::VersionCheck,
    false
);

// Snapshot test to cover VersionCheckView::draw_ui and centered_rect
screen_snapshot_test!(
    test_version_check_screen_snapshot,
    VersionCheckScreen,
    VersionCheckScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    )
);

#[test]
fn test_version_check_screen_default_provider_returns_unit_data() {
    let data = <VersionCheckScreen as Screen>::default_provider()
        .provide()
        .unwrap();

    assert!(data.downcast::<()>().is_ok());
}

#[test]
fn test_version_check_screen_non_exit_key_does_not_publish_navigation() {
    let event_bus = Arc::new(EventBus::new());
    let published_events = Arc::new(Mutex::new(0usize));
    let observed_events = Arc::clone(&published_events);
    let screen = VersionCheckScreen::new(
        event_bus.clone(),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    event_bus.subscribe(move |_: &NavigateTo| {
        let mut count = observed_events.lock().unwrap();
        *count += 1;
    });

    screen
        .handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()))
        .unwrap();

    assert_eq!(*published_events.lock().unwrap(), 0);
}

#[test]
fn test_version_check_screen_as_any_downcasts_to_concrete_type() {
    let screen = VersionCheckScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    assert!(screen
        .as_any()
        .downcast_ref::<VersionCheckScreen>()
        .is_some());
}
