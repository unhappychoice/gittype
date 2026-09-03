use gittype::domain::models::color_mode::ColorMode;
use gittype::infrastructure::appearance::for_test::{
    parse_busctl_color_scheme, parse_gsettings_color_scheme, parse_kdeglobals,
    parse_macos_interface_style, parse_windows_apps_use_light_theme,
};

#[test]
fn busctl_parses_prefer_dark() {
    // `busctl --user call ... Read ss ...` prints the (v) reply as `v v u N`.
    assert_eq!(parse_busctl_color_scheme("v v u 1"), Some(ColorMode::Dark));
}

#[test]
fn busctl_parses_prefer_light() {
    assert_eq!(parse_busctl_color_scheme("v v u 2"), Some(ColorMode::Light));
}

#[test]
fn busctl_parses_no_preference_as_none() {
    assert_eq!(parse_busctl_color_scheme("v v u 0"), None);
}

#[test]
fn busctl_rejects_garbage() {
    assert_eq!(parse_busctl_color_scheme(""), None);
    assert_eq!(parse_busctl_color_scheme("variant u"), None);
}

#[test]
fn gsettings_parses_prefer_dark() {
    assert_eq!(
        parse_gsettings_color_scheme("'prefer-dark'"),
        Some(ColorMode::Dark)
    );
}

#[test]
fn gsettings_parses_prefer_light() {
    assert_eq!(
        parse_gsettings_color_scheme("'prefer-light'"),
        Some(ColorMode::Light)
    );
}

#[test]
fn gsettings_parses_default_as_none() {
    assert_eq!(parse_gsettings_color_scheme("'default'"), None);
}

#[test]
fn windows_parses_light_theme() {
    let output = "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize\n    AppsUseLightTheme    REG_DWORD    0x1\n";
    assert_eq!(
        parse_windows_apps_use_light_theme(output),
        Some(ColorMode::Light)
    );
}

#[test]
fn windows_parses_dark_theme() {
    let output = "    AppsUseLightTheme    REG_DWORD    0x0\n";
    assert_eq!(
        parse_windows_apps_use_light_theme(output),
        Some(ColorMode::Dark)
    );
}

#[test]
fn windows_rejects_missing_value() {
    assert_eq!(parse_windows_apps_use_light_theme("no such key"), None);
}

#[test]
fn macos_parses_dark() {
    assert_eq!(
        parse_macos_interface_style("Dark", true),
        Some(ColorMode::Dark)
    );
}

#[test]
fn macos_missing_key_means_light() {
    // `defaults` exits non-zero with empty output when AppleInterfaceStyle is absent.
    assert_eq!(
        parse_macos_interface_style("", false),
        Some(ColorMode::Light)
    );
}

#[test]
fn macos_other_output_means_light() {
    assert_eq!(
        parse_macos_interface_style("something unexpected", true),
        Some(ColorMode::Light)
    );
}

#[test]
fn kdeglobals_parses_dark_scheme() {
    let contents = "[General]\nColorScheme=BreezeDark\n";
    assert_eq!(parse_kdeglobals(contents), Some(ColorMode::Dark));
}

#[test]
fn kdeglobals_parses_light_scheme() {
    let contents = "[General]\nColorScheme=BreezeLight\n";
    assert_eq!(parse_kdeglobals(contents), Some(ColorMode::Light));
}

#[test]
fn kdeglobals_ambiguous_scheme_is_none() {
    let contents = "[General]\nColorScheme=Breeze\n";
    assert_eq!(parse_kdeglobals(contents), None);
}

#[test]
fn kdeglobals_color_scheme_in_other_section_is_none() {
    let contents = "[KDE]\nColorScheme=BreezeDark\n";
    assert_eq!(parse_kdeglobals(contents), None);
}

#[test]
fn kdeglobals_missing_section_is_none() {
    let contents = "[Colors:View]\nColorScheme=BreezeDark\n";
    assert_eq!(parse_kdeglobals(contents), None);
}

#[test]
fn kdeglobals_ignores_other_keys_in_general() {
    let contents = "[General]\nColorScheme=Noctalia\nColorSchemeHash=2c3f\n\n[KDE]\nSingleClick=true\n";
    assert_eq!(parse_kdeglobals(contents), None);
}