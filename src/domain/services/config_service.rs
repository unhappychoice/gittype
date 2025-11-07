use crate::domain::models::config::Config;
use crate::infrastructure::storage::file_storage::{FileStorage, FileStorageInterface};
use crate::infrastructure::storage::AppDataProvider;
use crate::Result;
use shaku::Interface;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub trait ConfigServiceInterface: Interface {
    fn init(&self) -> Result<()>;
    fn get_config(&self) -> Config;
    fn save(&self) -> Result<()>;
}

#[derive(shaku::Component)]
#[shaku(interface = ConfigServiceInterface)]
pub struct ConfigService {
    #[shaku(default)]
    config: RwLock<Config>,
    #[shaku(inject)]
    file_storage: Arc<dyn FileStorageInterface>,
}

impl ConfigService {
    pub fn new(file_storage: Arc<dyn FileStorageInterface>) -> Result<Self> {
        let service = ConfigService {
            config: RwLock::new(Config::default()),
            file_storage,
        };

        let config_path = service.get_config_path()?;
        let storage = (service.file_storage.as_ref() as &dyn std::any::Any)
            .downcast_ref::<FileStorage>()
            .ok_or_else(|| {
                crate::GitTypeError::ExtractionFailed("Failed to downcast storage".to_string())
            })?;

        let config = storage
            .read_json::<Config>(&config_path)?
            .unwrap_or_default();

        *service.config.write().unwrap() = config;
        Ok(service)
    }

    #[cfg(feature = "test-mocks")]
    pub fn new_for_test() -> Result<Self> {
        Self::new(Arc::new(FileStorage::new()))
    }

    pub fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Config),
    {
        let mut config = self.config.write().unwrap();
        updater(&mut *config);
        Ok(())
    }

    fn get_config_path(&self) -> Result<PathBuf> {
        Ok(<FileStorage as AppDataProvider>::get_app_data_dir()?.join("config.json"))
    }
}

impl ConfigServiceInterface for ConfigService {
    fn init(&self) -> Result<()> {
        let config_path = self.get_config_path()?;
        let storage = (self.file_storage.as_ref() as &dyn std::any::Any)
            .downcast_ref::<FileStorage>()
            .ok_or_else(|| {
                crate::GitTypeError::ExtractionFailed("Failed to downcast storage".to_string())
            })?;

        let config = storage
            .read_json::<Config>(&config_path)?
            .unwrap_or_default();

        *self.config.write().unwrap() = config;
        Ok(())
    }

    fn get_config(&self) -> Config {
        self.config.read().unwrap().clone()
    }

    fn save(&self) -> Result<()> {
        let config_path = self.get_config_path()?;

        let storage = (self.file_storage.as_ref() as &dyn std::any::Any)
            .downcast_ref::<FileStorage>()
            .ok_or_else(|| {
                crate::GitTypeError::ExtractionFailed("Failed to downcast storage".to_string())
            })?;

        let config = self.config.read().unwrap();
        storage.write_json(&config_path, &*config)
    }
}
