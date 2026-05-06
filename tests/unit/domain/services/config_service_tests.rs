use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::services::config_service::{ConfigService, ConfigServiceInterface};
use gittype::infrastructure::storage::file_storage::{FileEntry, FileStorageInterface};
use gittype::GitTypeError;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug)]
struct NonFileStorage;

impl FileStorageInterface for NonFileStorage {
    fn delete_file(&self, _file_path: &Path) -> gittype::Result<()> {
        Ok(())
    }

    fn file_exists(&self, _file_path: &Path) -> bool {
        false
    }

    fn walk_directory(&self, _path: &Path) -> gittype::Result<Vec<FileEntry>> {
        Ok(Vec::new())
    }

    fn read_to_string(&self, _file_path: &Path) -> gittype::Result<String> {
        Ok(String::new())
    }

    fn create_dir_all(&self, _path: &Path) -> gittype::Result<()> {
        Ok(())
    }

    fn write(&self, _file_path: &Path, _contents: &[u8]) -> gittype::Result<()> {
        Ok(())
    }

    fn metadata(&self, file_path: &Path) -> gittype::Result<std::fs::Metadata> {
        std::fs::metadata(file_path).map_err(Into::into)
    }

    fn read_dir(&self, path: &Path) -> gittype::Result<std::fs::ReadDir> {
        std::fs::read_dir(path).map_err(Into::into)
    }

    fn remove_dir_all(&self, _path: &Path) -> gittype::Result<()> {
        Ok(())
    }

    fn get_app_data_dir(&self) -> gittype::Result<PathBuf> {
        Ok(PathBuf::from("/tmp/test"))
    }
}

fn non_file_storage() -> Arc<dyn FileStorageInterface> {
    Arc::new(NonFileStorage)
}

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

#[test]
fn test_new_rejects_non_file_storage() {
    let result = ConfigService::new(non_file_storage());

    assert!(matches!(
        result,
        Err(GitTypeError::ExtractionFailed(message))
        if message == "Failed to downcast storage"
    ));
}
