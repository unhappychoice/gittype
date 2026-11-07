use crate::domain::models::color_mode::ColorMode;
use crate::domain::services::appearance_detector::AppearanceDetectorInterface;
use std::process::Command;

/// Detects the OS light/dark appearance using only native platform tools.
/// Detection is best-effort on every platform; any failure degrades to `None`.
#[derive(shaku::Component)]
#[shaku(interface = AppearanceDetectorInterface)]
pub struct OsAppearanceDetector;

impl AppearanceDetectorInterface for OsAppearanceDetector {
    fn detect_color_mode(&self) -> Option<ColorMode> {
        detect_system_color_mode()
    }
}

/// Detect the system appearance by platform. Never panics.
pub(crate) fn detect_system_color_mode() -> Option<ColorMode> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output();
        match output {
            Ok(output) => parse_macos_interface_style(
                &String::from_utf8_lossy(&output.stdout),
                output.status.success(),
            ),
            Err(_) => None,
        }
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("reg")
            .args([
                "query",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output()
            .ok()?;
        parse_windows_apps_use_light_theme(&String::from_utf8_lossy(&output.stdout))
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        detect_linux_color_mode()
    }

    #[cfg(not(any(unix, target_os = "windows")))]
    {
        let _: Option<ColorMode> = None;
        None
    }
}

/// Linux/BSD detection chain: XDG Desktop Portal (via busctl), then gsettings,
/// then KDE's kdeglobals. First successful result wins.
#[cfg(all(unix, not(target_os = "macos")))]
fn detect_linux_color_mode() -> Option<ColorMode> {
    if let Some(mode) = detect_busctl_color_scheme() {
        return Some(mode);
    }
    if let Some(mode) = detect_gsettings_color_scheme() {
        return Some(mode);
    }
    detect_kdeglobals_color_scheme()
}

#[cfg(all(unix, not(target_os = "macos")))]
fn detect_busctl_color_scheme() -> Option<ColorMode> {
    let output = Command::new("busctl")
        .args([
            "--user",
            "call",
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings",
            "Read",
            "ss",
            "org.freedesktop.appearance",
            "color-scheme",
        ])
        .output()
        .ok()?;
    parse_busctl_color_scheme(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn detect_gsettings_color_scheme() -> Option<ColorMode> {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
        .ok()?;
    parse_gsettings_color_scheme(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn detect_kdeglobals_color_scheme() -> Option<ColorMode> {
    let path = dirs::config_dir()?.join("kdeglobals");
    let contents = std::fs::read_to_string(path).ok()?;
    parse_kdeglobals(&contents)
}

/// Portal `color-scheme` output is `u 0|1|2` (0 = no preference, 1 = prefer dark, 2 = prefer light).
pub(crate) fn parse_busctl_color_scheme(output: &str) -> Option<ColorMode> {
    let value = output.split_whitespace().last()?.parse::<u32>().ok()?;
    match value {
        1 => Some(ColorMode::Dark),
        2 => Some(ColorMode::Light),
        _ => None,
    }
}

/// gsettings output is `'default'`, `'prefer-dark'`, or `'prefer-light'`.
pub(crate) fn parse_gsettings_color_scheme(output: &str) -> Option<ColorMode> {
    if output.contains("prefer-dark") {
        Some(ColorMode::Dark)
    } else if output.contains("prefer-light") {
        Some(ColorMode::Light)
    } else {
        None
    }
}

/// Only called on Windows; kept compiled everywhere so unit tests can cover its parsing.
#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
pub(crate) fn parse_windows_apps_use_light_theme(output: &str) -> Option<ColorMode> {
    let line = output.lines().find(|l| l.contains("AppsUseLightTheme"))?;
    if line.contains("0x1") {
        Some(ColorMode::Light)
    } else if line.contains("0x0") {
        Some(ColorMode::Dark)
    } else {
        None
    }
}

/// Only called on macOS; kept compiled everywhere so unit tests can cover its parsing.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn parse_macos_interface_style(output: &str, ok: bool) -> Option<ColorMode> {
    if !ok && output.is_empty() {
        // An absent AppleInterfaceStyle key makes `defaults` exit non-zero; macOS then defaults to light.
        return Some(ColorMode::Light);
    }
    if !ok {
        return None;
    }
    if output.contains("Dark") {
        Some(ColorMode::Dark)
    } else {
        Some(ColorMode::Light)
    }
}
/// KDE stores `ColorScheme=...` under `[General]` in `~/.config/kdeglobals`.
pub(crate) fn parse_kdeglobals(contents: &str) -> Option<ColorMode> {
    let mut in_general_section = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_general_section = line == "[General]";
            continue;
        }
        if in_general_section {
            if let Some(value) = line.strip_prefix("ColorScheme=") {
                if value.contains("Dark") {
                    return Some(ColorMode::Dark);
                }
                if value.contains("Light") {
                    return Some(ColorMode::Light);
                }
                return None;
            }
        }
    }
    None
}

/// Test-only wrappers over the platform parsers. Integration tests are a
/// separate crate and cannot see `pub(crate)` items directly.
#[cfg(feature = "test-mocks")]
pub mod for_test {
    use super::*;

    pub fn parse_busctl_color_scheme(output: &str) -> Option<ColorMode> {
        super::parse_busctl_color_scheme(output)
    }
    pub fn parse_gsettings_color_scheme(output: &str) -> Option<ColorMode> {
        super::parse_gsettings_color_scheme(output)
    }
    pub fn parse_windows_apps_use_light_theme(output: &str) -> Option<ColorMode> {
        super::parse_windows_apps_use_light_theme(output)
    }
    pub fn parse_macos_interface_style(output: &str, ok: bool) -> Option<ColorMode> {
        super::parse_macos_interface_style(output, ok)
    }
    pub fn parse_kdeglobals(contents: &str) -> Option<ColorMode> {
        super::parse_kdeglobals(contents)
    }
}
