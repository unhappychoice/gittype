use crate::integration::screens::mocks::analytics_screen_mock::{
    MockAnalyticsDataProvider, MockAnalyticsDataProviderEmpty,
    MockAnalyticsDataProviderWithActivity,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::analytics_screen::AnalyticsScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_analytics_screen_snapshot_overview,
    AnalyticsScreen,
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockAnalyticsDataProvider
);

// Test with daily session data to render the chart
screen_snapshot_test!(
    test_analytics_screen_snapshot_overview_with_activity,
    AnalyticsScreen,
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockAnalyticsDataProviderWithActivity
);

// Test with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_overview_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockAnalyticsDataProviderEmpty
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_trends,
    AnalyticsScreen,
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockAnalyticsDataProvider,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

// Test trends view with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_trends_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockAnalyticsDataProviderEmpty,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_repositories,
    AnalyticsScreen,
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
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
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
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
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockAnalyticsDataProviderEmpty,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_languages,
    AnalyticsScreen,
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
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
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
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
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
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
    AnalyticsScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    gittype::presentation::tui::ScreenType::Analytics,
    false,
    MockAnalyticsDataProvider
);
