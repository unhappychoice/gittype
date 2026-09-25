use crate::domain::models::color_mode::ColorMode;
use shaku::Interface;

/// Detects the OS light/dark appearance preference.
pub trait AppearanceDetectorInterface: Interface {
    /// Returns `Some(Dark|Light)` when the OS appearance can be determined,
    /// `None` when it cannot (unsupported platform, no desktop session, etc).
    fn detect_color_mode(&self) -> Option<ColorMode>;
}

/// Always reports undetectable; used as the default injection in test helpers.
#[cfg(feature = "test-mocks")]
pub struct NoopAppearanceDetector;

#[cfg(feature = "test-mocks")]
impl AppearanceDetectorInterface for NoopAppearanceDetector {
    fn detect_color_mode(&self) -> Option<ColorMode> {
        None
    }
}

/// Returns a fixed mode; gives deterministic resolution in tests.
#[cfg(feature = "test-mocks")]
pub struct FixedAppearanceDetector(pub Option<ColorMode>);

#[cfg(feature = "test-mocks")]
impl AppearanceDetectorInterface for FixedAppearanceDetector {
    fn detect_color_mode(&self) -> Option<ColorMode> {
        self.0.clone()
    }
}
