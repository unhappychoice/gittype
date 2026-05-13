use crate::integration::screens::mocks::analytics_screen_mock::{
    MockAnalyticsDataProvider, MockAnalyticsDataProviderEmpty,
    MockAnalyticsDataProviderWithActivity,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::analytics_screen::{
    AnalyticsAction, AnalyticsScreen, ViewMode,
};
use gittype::presentation::tui::{Screen, ScreenDataProvider};
use std::sync::{Arc, Mutex};

#[test]
fn test_analytics_view_mode_cycles_forward_and_backward() {
    assert_eq!(ViewMode::Overview.next(), ViewMode::Trends);
    assert_eq!(ViewMode::Trends.next(), ViewMode::Repositories);
    assert_eq!(ViewMode::Repositories.next(), ViewMode::Languages);
    assert_eq!(ViewMode::Languages.next(), ViewMode::Overview);

    assert_eq!(ViewMode::Overview.previous(), ViewMode::Languages);
    assert_eq!(ViewMode::Trends.previous(), ViewMode::Overview);
    assert_eq!(ViewMode::Repositories.previous(), ViewMode::Trends);
    assert_eq!(ViewMode::Languages.previous(), ViewMode::Repositories);
}

screen_snapshot_test!(
    test_analytics_screen_snapshot_overview,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProvider
);

// Test with daily session data to render the chart
screen_snapshot_test!(
    test_analytics_screen_snapshot_overview_with_activity,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProviderWithActivity
);

// Test with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_overview_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProviderEmpty
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_trends,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProvider,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

// Test trends view with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_trends_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProviderEmpty,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_repositories,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProvider,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

// Test repositories view with activity (includes long names)
screen_snapshot_test!(
    test_analytics_screen_snapshot_repositories_with_activity,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProviderWithActivity,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

// Test repositories view with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_repositories_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProviderEmpty,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_languages,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProvider,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

// Test languages view with activity (includes long names)
screen_snapshot_test!(
    test_analytics_screen_snapshot_languages_with_activity,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProviderWithActivity,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

// Test languages view with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_languages_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    provider = MockAnalyticsDataProviderEmpty,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

// Event-producing key tests
screen_key_event_test!(
    test_analytics_screen_esc_navigates_to_title,
    AnalyticsScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockAnalyticsDataProvider
);

screen_key_event_test!(
    test_analytics_screen_ctrl_c_exits,
    AnalyticsScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockAnalyticsDataProvider
);

#[test]
fn test_analytics_screen_esc_sets_return_action_result() {
    let event_bus = Arc::new(EventBus::new());
    let observed_events = Arc::new(Mutex::new(0usize));
    let event_count = Arc::clone(&observed_events);
    let screen = AnalyticsScreen::new(
        event_bus.clone(),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark,
        )) as Arc<dyn ThemeServiceInterface>,
    );

    event_bus.subscribe(move |_: &NavigateTo| {
        *event_count.lock().unwrap() += 1;
    });

    screen
        .init_with_data(MockAnalyticsDataProvider.provide().unwrap())
        .unwrap();
    screen
        .handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
        .unwrap();

    assert!(matches!(
        screen.get_action_result(),
        Some(AnalyticsAction::Return)
    ));
    assert_eq!(*observed_events.lock().unwrap(), 1);
}

// Non-event key tests
screen_key_tests!(
    AnalyticsScreen,
    MockAnalyticsDataProvider,
    [
        (
            test_analytics_screen_left_switches_view,
            KeyCode::Left,
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_h_switches_view,
            KeyCode::Char('h'),
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_right_switches_view,
            KeyCode::Right,
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_l_switches_view,
            KeyCode::Char('l'),
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_up_navigates,
            KeyCode::Up,
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_k_navigates,
            KeyCode::Char('k'),
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_down_navigates,
            KeyCode::Down,
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_j_navigates,
            KeyCode::Char('j'),
            KeyModifiers::empty()
        ),
        (
            test_analytics_screen_r_refreshes,
            KeyCode::Char('r'),
            KeyModifiers::empty()
        ),
    ]
);

// Basic methods test
screen_basic_methods_test!(
    test_analytics_screen_basic_methods,
    AnalyticsScreen,
    AnalyticsScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>
    ),
    gittype::presentation::tui::ScreenType::Analytics,
    false,
    MockAnalyticsDataProvider
);

fn build_analytics_screen() -> AnalyticsScreen {
    let event_bus = Arc::new(EventBus::new());
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    AnalyticsScreen::new(event_bus, theme_service)
}

fn send_keys(screen: &AnalyticsScreen, keys: &[KeyCode]) {
    for code in keys {
        screen
            .handle_key_event(KeyEvent::new(*code, KeyModifiers::empty()))
            .unwrap();
    }
}

#[test]
fn test_analytics_screen_down_navigates_repository_list() {
    let screen = build_analytics_screen();
    screen
        .init_with_data(MockAnalyticsDataProvider.provide().unwrap())
        .unwrap();

    send_keys(
        &screen,
        &[KeyCode::Right, KeyCode::Right, KeyCode::Down, KeyCode::Down],
    );

    // Re-renders without panicking after wrapping
    let backend = ratatui::backend::TestBackend::new(120, 40);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            screen.render_ratatui(f).unwrap();
        })
        .unwrap();
}

#[test]
fn test_analytics_screen_up_navigates_repository_list() {
    let screen = build_analytics_screen();
    screen
        .init_with_data(MockAnalyticsDataProvider.provide().unwrap())
        .unwrap();

    // Right, Right to Repositories, then Up wraps from 0 to last, then Up again to 0.
    send_keys(
        &screen,
        &[KeyCode::Right, KeyCode::Right, KeyCode::Up, KeyCode::Up],
    );
}

#[test]
fn test_analytics_screen_navigates_language_list_with_jk() {
    let screen = build_analytics_screen();
    screen
        .init_with_data(MockAnalyticsDataProvider.provide().unwrap())
        .unwrap();

    // Right Right Right to Languages, then j j k k for next/previous wrapping paths.
    send_keys(
        &screen,
        &[
            KeyCode::Right,
            KeyCode::Right,
            KeyCode::Right,
            KeyCode::Char('j'),
            KeyCode::Char('j'),
            KeyCode::Char('k'),
            KeyCode::Char('k'),
        ],
    );
}

#[test]
fn test_analytics_screen_renders_language_without_detailed_stats() {
    // MockAnalyticsDataProvider's top_languages contains both "Rust" and "Python",
    // but language_stats only has an entry for "Rust" — navigating to Python triggers
    // the `else` branch in LanguagesView::render_language_details where `detailed_stats`
    // resolves to None.
    let screen = build_analytics_screen();
    screen
        .init_with_data(MockAnalyticsDataProvider.provide().unwrap())
        .unwrap();

    send_keys(
        &screen,
        &[
            KeyCode::Right,
            KeyCode::Right,
            KeyCode::Right,
            KeyCode::Char('j'),
        ],
    );

    let backend = ratatui::backend::TestBackend::new(120, 40);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            screen.render_ratatui(f).unwrap();
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut output = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            output.push_str(buffer[(x, y)].symbol());
        }
        output.push('\n');
    }

    assert!(output.contains("Python"));
    assert!(output.contains("Session Count"));
    assert!(output.contains("WPM Equivalent"));
}

#[test]
fn test_analytics_screen_unhandled_key_returns_ok() {
    let screen = build_analytics_screen();
    screen
        .init_with_data(MockAnalyticsDataProvider.provide().unwrap())
        .unwrap();

    let result = screen.handle_key_event(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::empty()));

    assert!(result.is_ok());
}

#[test]
fn test_analytics_view_mode_default_is_overview() {
    assert_eq!(ViewMode::default(), ViewMode::Overview);
}

#[test]
fn test_analytics_view_mode_display_names() {
    assert_eq!(ViewMode::Overview.display_name(), "Overview");
    assert_eq!(ViewMode::Trends.display_name(), "Trends");
    assert_eq!(ViewMode::Repositories.display_name(), "Repositories");
    assert_eq!(ViewMode::Languages.display_name(), "Languages");
}
