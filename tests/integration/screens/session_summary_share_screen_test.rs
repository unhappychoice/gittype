use crate::integration::screens::mocks::session_summary_share_screen_mock::MockSessionSummaryShareDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::session_summary_share_screen::SessionSummaryShareScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_session_summary_share_screen_snapshot,
    SessionSummaryShareScreen,
    SessionSummaryShareScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockSessionSummaryShareDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_summary_share_screen_1_shares_to_x,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('1'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_2_shares_to_reddit,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('2'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_3_shares_to_linkedin,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('3'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_4_shares_to_facebook,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('4'),
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_esc_goes_back,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionSummaryShareDataProvider
);

screen_key_event_test!(
    test_session_summary_share_screen_ctrl_c_exits,
    SessionSummaryShareScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionSummaryShareDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_summary_share_screen_basic_methods,
    SessionSummaryShareScreen,
    SessionSummaryShareScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    gittype::presentation::tui::ScreenType::SessionSharing,
    false,
    MockSessionSummaryShareDataProvider
);
