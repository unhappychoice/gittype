use crate::integration::screens::mocks::analytics_screen_mock::{
    MockAnalyticsDataProvider, MockAnalyticsDataProviderEmpty,
    MockAnalyticsDataProviderWithActivity,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::analytics_screen::AnalyticsScreen;

screen_snapshot_test!(
    test_analytics_screen_snapshot_overview,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProvider
);

// Test with daily session data to render the chart
screen_snapshot_test!(
    test_analytics_screen_snapshot_overview_with_activity,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProviderWithActivity
);

// Test with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_overview_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProviderEmpty
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_trends,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProvider,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

// Test trends view with empty data
screen_snapshot_test!(
    test_analytics_screen_snapshot_trends_empty,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProviderEmpty,
    keys = [KeyEvent::new(KeyCode::Right, KeyModifiers::empty())]
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_repositories,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
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
    AnalyticsScreen::new(EventBus::new()),
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
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProviderEmpty,
    keys = [
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
        KeyEvent::new(KeyCode::Right, KeyModifiers::empty())
    ]
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_languages,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
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
    AnalyticsScreen::new(EventBus::new()),
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
    AnalyticsScreen::new(EventBus::new()),
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
    AnalyticsScreen::new(EventBus::new()),
    gittype::presentation::tui::ScreenType::Analytics,
    false,
    MockAnalyticsDataProvider
);
