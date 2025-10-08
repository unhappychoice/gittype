use crate::integration::screens::mocks::analytics_screen_mock::MockAnalyticsDataProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::analytics_screen::AnalyticsScreen;

screen_snapshot_test!(
    test_analytics_screen_snapshot_overview,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProvider
);

screen_snapshot_test!(
    test_analytics_screen_snapshot_trends,
    AnalyticsScreen,
    AnalyticsScreen::new(EventBus::new()),
    provider = MockAnalyticsDataProvider,
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
