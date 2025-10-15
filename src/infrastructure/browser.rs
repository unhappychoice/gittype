use std::sync::atomic::{AtomicBool, Ordering};

static TEST_MODE: AtomicBool = AtomicBool::new(false);

/// Enable test mode to prevent actual browser opening during tests
pub fn enable_test_mode() {
    TEST_MODE.store(true, Ordering::SeqCst);
}

/// Disable test mode
pub fn disable_test_mode() {
    TEST_MODE.store(false, Ordering::SeqCst);
}

/// Opens a URL in the default browser.
///
/// During tests (when test mode is enabled), this function does nothing.
/// In production, this uses the `open` crate to open the URL.
pub fn open_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if TEST_MODE.load(Ordering::SeqCst) {
        // In test mode, don't actually open browsers
        Ok(())
    } else {
        open::that(url)?;
        Ok(())
    }
}
