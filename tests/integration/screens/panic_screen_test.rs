use gittype::domain::events::EventBus;
use gittype::presentation::game::screens::panic_screen::PanicScreen;

screen_snapshot_test!(
    test_panic_screen_snapshot_with_fixed_timestamp,
    PanicScreen,
    PanicScreen::with_error_message(
        "Test panic message".to_string(),
        EventBus::new(),
        Some("SystemTime { tv_sec: 1700000000, tv_nsec: 0 }".to_string())
    )
);
