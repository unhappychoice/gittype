use crate::integration::screens::mocks::session_details_dialog_mock::MockSessionDetailsDialogDataProvider;
use crossterm::event::{KeyCode, KeyModifiers};
use gittype::domain::events::EventBus;
use gittype::presentation::game::events::NavigateTo;
use gittype::presentation::tui::screens::session_details_dialog::SessionDetailsDialog;

screen_snapshot_test!(
    test_session_details_dialog_snapshot,
    SessionDetailsDialog,
    SessionDetailsDialog::new(EventBus::new()),
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
