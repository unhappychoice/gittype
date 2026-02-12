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

#[test]
fn test_init_loads_default_config() {
    let service = ConfigService::new_for_test().unwrap();
    service.init().unwrap();

    let config = service.get_config();
    assert_eq!(config.theme.current_theme_id, "default");
    assert_eq!(config.theme.current_color_mode, ColorMode::Dark);
}

#[test]
fn test_init_overwrites_modified_config() {
    let service = ConfigService::new_for_test().unwrap();
    service
        .update_config(|config| {
            config.theme.current_theme_id = "custom".to_string();
        })
        .unwrap();

    // init should reload from storage (default since no file persisted)
    service.init().unwrap();
    let config = service.get_config();
    assert_eq!(config.theme.current_theme_id, "default");
}

#[test]
fn test_save_succeeds_after_update() {
    let service = ConfigService::new_for_test().unwrap();
    service
        .update_config(|config| {
            config.theme.current_theme_id = "saved_theme".to_string();
            config.theme.current_color_mode = ColorMode::Light;
        })
        .unwrap();

    // save() should succeed even with modified config
    assert!(service.save().is_ok());
}

#[test]
fn test_multiple_updates_accumulate() {
    let service = ConfigService::new_for_test().unwrap();
    service
        .update_config(|config| {
            config.theme.current_theme_id = "first".to_string();
        })
        .unwrap();
    service
        .update_config(|config| {
            config.theme.current_color_mode = ColorMode::Light;
        })
        .unwrap();

    let config = service.get_config();
    assert_eq!(config.theme.current_theme_id, "first");
    assert_eq!(config.theme.current_color_mode, ColorMode::Light);
}

#[test]
fn test_save_without_changes() {
    let service = ConfigService::new_for_test().unwrap();
    // Save default config - should succeed
    assert!(service.save().is_ok());
}
