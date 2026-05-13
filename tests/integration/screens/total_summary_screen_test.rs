use crate::integration::screens::mocks::total_summary_screen_mock::MockTotalSummaryDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::domain::models::SessionResult;
use gittype::domain::services::scoring::{TotalTracker, TotalTrackerInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::total_summary_screen::{
    TotalSummaryScreen, TotalSummaryScreenData, TotalSummaryScreenDataProvider,
};
use gittype::presentation::tui::{Screen, ScreenDataProvider};
use std::sync::Arc;

screen_snapshot_test!(
    test_total_summary_screen_snapshot,
    TotalSummaryScreen,
    TotalSummaryScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::default()) as Arc<dyn TotalTrackerInterface>
    ),
    provider = MockTotalSummaryDataProvider
);

// Helper function to create TotalSummaryScreen
fn create_total_summary_screen(
    event_bus: Arc<dyn gittype::domain::events::EventBusInterface>,
) -> TotalSummaryScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;
    let total_tracker: Arc<dyn TotalTrackerInterface> = Arc::new(TotalTracker::default());
    TotalSummaryScreen::new(event_bus, theme_service, total_tracker)
}

// Event-producing key tests
screen_key_event_test!(
    test_total_summary_screen_s_shares,
    TotalSummaryScreen,
    create_total_summary_screen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockTotalSummaryDataProvider
);

screen_key_event_test!(
    test_total_summary_screen_capital_s_shares,
    TotalSummaryScreen,
    create_total_summary_screen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockTotalSummaryDataProvider
);

screen_key_event_test!(
    test_total_summary_screen_esc_exits,
    TotalSummaryScreen,
    create_total_summary_screen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockTotalSummaryDataProvider
);

screen_key_event_test!(
    test_total_summary_screen_ctrl_c_exits,
    TotalSummaryScreen,
    create_total_summary_screen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockTotalSummaryDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_total_summary_screen_basic_methods,
    TotalSummaryScreen,
    TotalSummaryScreen::new(
        Arc::new(EventBus::new()),
        Arc::new(ThemeService::new_for_test(
            Theme::default(),
            ColorMode::Dark
        )) as Arc<dyn ThemeServiceInterface>,
        Arc::new(TotalTracker::default()) as Arc<dyn TotalTrackerInterface>
    ),
    gittype::presentation::tui::ScreenType::TotalSummary,
    false,
    MockTotalSummaryDataProvider
);

#[test]
fn test_total_summary_screen_default_provider_errors_without_tracker() {
    let result = <TotalSummaryScreen as Screen>::default_provider().provide();
    assert!(
        result.is_err(),
        "default_provider should error because tracker mutex holds None"
    );
}

#[test]
fn test_total_summary_screen_data_provider_returns_data_when_tracker_present() {
    let tracker = TotalTracker::default();
    let mut session_result = SessionResult::new();
    session_result.session_score = 1500.0;
    session_result.overall_cpm = 320.0;
    tracker.record(session_result);

    let provider = TotalSummaryScreenDataProvider::new_for_test(Some(tracker));
    let data = provider
        .provide()
        .expect("provider should produce data when tracker has sessions");
    let screen_data = data
        .downcast::<TotalSummaryScreenData>()
        .expect("data should downcast to TotalSummaryScreenData");
    assert_eq!(screen_data.total_result.total_sessions_attempted, 1);
}

#[test]
fn test_total_summary_screen_data_provider_errors_when_tracker_none() {
    let provider = TotalSummaryScreenDataProvider::new_for_test(None);
    assert!(provider.provide().is_err());
}

#[test]
fn test_total_summary_screen_init_with_data_falls_back_to_injected_tracker() {
    let event_bus = Arc::new(EventBus::new());
    let screen = create_total_summary_screen(event_bus);

    let data: Box<dyn std::any::Any> = Box::new(());
    assert!(screen.init_with_data(data).is_ok());
}

#[test]
fn test_total_summary_screen_as_any_downcasts_to_concrete_type() {
    let screen = create_total_summary_screen(Arc::new(EventBus::new()));
    assert!(screen
        .as_any()
        .downcast_ref::<TotalSummaryScreen>()
        .is_some());
}

#[test]
fn test_total_summary_screen_unrelated_keys_are_noop() {
    use crossterm::event::KeyEvent;

    let screen = create_total_summary_screen(Arc::new(EventBus::new()));
    let result = screen.handle_key_event(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty()));
    assert!(result.is_ok());
}
