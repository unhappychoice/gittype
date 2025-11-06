use crate::integration::screens::mocks::session_summary_screen_mock::{
    MockCompilerDataProvider, MockLoadBalancerPrimarchDataProvider, MockSessionSummaryDataProvider,
};
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::session_summary_screen::SessionSummaryScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_session_summary_screen_snapshot,
    SessionSummaryScreen,
    SessionSummaryScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockSessionSummaryDataProvider
);

screen_snapshot_test!(
    test_session_summary_screen_load_balancer_primarch_snapshot,
    SessionSummaryScreen,
    SessionSummaryScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockLoadBalancerPrimarchDataProvider
);

screen_snapshot_test!(
    test_session_summary_screen_compiler_snapshot,
    SessionSummaryScreen,
    SessionSummaryScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockCompilerDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_summary_screen_d_opens_details,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('d'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_d_opens_details,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('D'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_r_retries,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('r'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_r_retries,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('R'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_s_shares,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('s'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_s_shares,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('S'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_t_goes_to_title,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('t'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_capital_t_goes_to_title,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('T'),
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_esc_exits,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionSummaryDataProvider
);

screen_key_event_test!(
    test_session_summary_screen_ctrl_c_exits,
    SessionSummaryScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionSummaryDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_summary_screen_basic_methods,
    SessionSummaryScreen,
    SessionSummaryScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    gittype::presentation::tui::ScreenType::SessionSummary,
    false,
    MockSessionSummaryDataProvider
);
