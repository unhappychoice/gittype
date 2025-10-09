use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::info_dialog::InfoDialogScreen;

screen_snapshot_test!(
    test_info_dialog_snapshot_default,
    InfoDialogScreen,
    InfoDialogScreen::new(EventBus::new())
);
