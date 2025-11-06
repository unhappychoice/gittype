use crate::integration::screens::mocks::stage_summary_screen_mock::MockStageSummaryDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::stage_summary_screen::StageSummaryScreen;
use std::sync::Arc;

screen_snapshot_test!(
    test_stage_summary_screen_snapshot,
    StageSummaryScreen,
    StageSummaryScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockStageSummaryDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_stage_summary_screen_esc_navigates_to_session_failure,
    StageSummaryScreen,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_ctrl_c_navigates_to_session_failure,
    StageSummaryScreen,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockStageSummaryDataProvider
);

screen_key_event_test!(
    test_stage_summary_screen_space_continues,
    StageSummaryScreen,
    NavigateTo,
    KeyCode::Char(' '),
    KeyModifiers::empty(),
    MockStageSummaryDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_stage_summary_screen_basic_methods,
    StageSummaryScreen,
    StageSummaryScreen::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    gittype::presentation::tui::ScreenType::StageSummary,
    false,
    MockStageSummaryDataProvider
);
