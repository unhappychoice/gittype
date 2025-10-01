use crate::domain::models::config::Config;
use crate::infrastructure::storage::{file_storage::FileStorage, AppDataProvider};
use crate::Result;
use std::path::PathBuf;

pub struct ConfigService {
    config: Config,
}

impl ConfigService {
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        let storage = FileStorage::new();
        let config = storage
            .read_json::<Config>(&config_path)?
            .unwrap_or_default();

        Ok(ConfigService { config })
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let storage = FileStorage::new();
        storage.write_json(&config_path, &self.config)
    }

    fn get_config_path() -> Result<PathBuf> {
        Ok(<FileStorage as AppDataProvider>::get_app_data_dir()?.join("config.json"))
    }
}
