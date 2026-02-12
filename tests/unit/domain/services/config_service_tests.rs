use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::services::config_service::{ConfigService, ConfigServiceInterface};

#[test]
fn test_new_config_manager() {
    let config_manager = ConfigService::new_for_test().unwrap();
    let config = config_manager.get_config();
    assert_eq!(config.theme.current_theme_id, "default");
    assert_eq!(config.theme.current_color_mode, ColorMode::Dark);
}

#[test]
fn test_get_config() {
    let config_manager = ConfigService::new_for_test().unwrap();
    let config = config_manager.get_config();
    assert_eq!(config.theme.current_theme_id, "default");
    assert_eq!(config.theme.current_color_mode, ColorMode::Dark);
}

#[test]
fn test_update_config() {
    let config_manager = ConfigService::new_for_test().unwrap();
    config_manager
        .update_config(|config| {
            config.theme.current_theme_id = "test_theme".to_string();
        })
        .unwrap();
    assert_eq!(
        config_manager.get_config().theme.current_theme_id,
        "test_theme"
    );
}

#[test]
fn test_save() {
    let config_manager = ConfigService::new_for_test().unwrap();
    // This should not panic with mock implementation
    config_manager.save().unwrap();
}

#[test]
fn test_modify_color_mode() {
    let config_manager = ConfigService::new_for_test().unwrap();
    config_manager
        .update_config(|config| {
            config.theme.current_color_mode = ColorMode::Light;
        })
        .unwrap();
    assert_eq!(
        config_manager.get_config().theme.current_color_mode,
        ColorMode::Light
    );
}
