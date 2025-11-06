use crate::integration::screens::mocks::session_details_dialog_mock::MockSessionDetailsDialogDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::events::presentation_events::NavigateTo;
use gittype::presentation::tui::screens::session_details_dialog::SessionDetailsDialog;
use std::sync::Arc;

screen_snapshot_test!(
    test_session_details_dialog_snapshot,
    SessionDetailsDialog,
    SessionDetailsDialog::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    provider = MockSessionDetailsDialogDataProvider
);

// Event-producing key tests
screen_key_event_test!(
    test_session_details_dialog_esc_closes,
    SessionDetailsDialog,
    NavigateTo,
    KeyCode::Esc,
    KeyModifiers::empty(),
    MockSessionDetailsDialogDataProvider
);

screen_key_event_test!(
    test_session_details_dialog_ctrl_c_exits,
    SessionDetailsDialog,
    NavigateTo,
    KeyCode::Char('c'),
    KeyModifiers::CONTROL,
    MockSessionDetailsDialogDataProvider
);

// Basic methods test
screen_basic_methods_test!(
    test_session_details_dialog_basic_methods,
    SessionDetailsDialog,
    SessionDetailsDialog::new(Arc::new(EventBus::new()), Arc::new(ThemeService::new_for_test(Theme::default(), ColorMode::Dark)) as Arc<dyn ThemeServiceInterface>),
    gittype::presentation::tui::ScreenType::DetailsDialog,
    false,
    MockSessionDetailsDialogDataProvider
);
