use gittype::presentation::signal_handler::cleanup_panic_terminal_for_test;

#[test]
fn cleanup_panic_terminal_ignores_missing_terminal_state() {
    cleanup_panic_terminal_for_test(false, false);
    cleanup_panic_terminal_for_test(false, true);
}

#[test]
fn cleanup_panic_terminal_handles_initialized_terminal_state() {
    cleanup_panic_terminal_for_test(true, false);
    cleanup_panic_terminal_for_test(true, true);
}
