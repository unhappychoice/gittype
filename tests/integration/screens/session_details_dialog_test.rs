use crate::integration::screens::mocks::session_details_dialog_mock::MockSessionDetailsDialogDataProvider;
use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::session_details_dialog::SessionDetailsDialog;

screen_snapshot_test!(
    test_session_details_dialog_snapshot,
    SessionDetailsDialog,
    SessionDetailsDialog::new(EventBus::new()),
    provider = MockSessionDetailsDialogDataProvider
);
