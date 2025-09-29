use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::services::config_manager::ConfigService;

#[test]
fn test_new_config_manager() {
    let config_manager = ConfigService::new().unwrap();
    let config = config_manager.get_config();
    assert_eq!(config.theme.current_theme_id, "default");
    assert_eq!(config.theme.current_color_mode, ColorMode::Dark);
}

#[test]
fn test_get_config() {
    let config_manager = ConfigService::new().unwrap();
    let config = config_manager.get_config();
    assert_eq!(config.theme.current_theme_id, "default");
    assert_eq!(config.theme.current_color_mode, ColorMode::Dark);
}

#[test]
fn test_get_config_mut() {
    let mut config_manager = ConfigService::new().unwrap();
    let config = config_manager.get_config_mut();
    config.theme.current_theme_id = "test_theme".to_string();
    assert_eq!(
        config_manager.get_config().theme.current_theme_id,
        "test_theme"
    );
}

#[test]
fn test_save() {
    let config_manager = ConfigService::new().unwrap();
    // This should not panic with mock implementation
    config_manager.save().unwrap();
}

#[test]
fn test_modify_color_mode() {
    let mut config_manager = ConfigService::new().unwrap();
    let config = config_manager.get_config_mut();
    config.theme.current_color_mode = ColorMode::Light;
    assert_eq!(
        config_manager.get_config().theme.current_color_mode,
        ColorMode::Light
    );
}
