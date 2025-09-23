use crate::ui::color_mode::ColorMode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: ThemeConfig,
    pub trending: TrendingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_id")]
    pub current_theme_id: String,
    pub current_color_mode: ColorMode,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            current_theme_id: "default".to_string(),
            current_color_mode: ColorMode::default(),
        }
    }
}

fn default_theme_id() -> String {
    "default".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingConfig {
    #[serde(default = "default_cache_ttl_minutes")]
    pub cache_ttl_minutes: u64,
    #[serde(default = "default_rate_limit_ms")]
    pub rate_limit_ms: u64,
    #[serde(default = "default_period")]
    pub default_period: String,
    #[serde(default)]
    pub default_language: Option<String>,
}

impl Default for TrendingConfig {
    fn default() -> Self {
        Self {
            cache_ttl_minutes: default_cache_ttl_minutes(),
            rate_limit_ms: default_rate_limit_ms(),
            default_period: default_period(),
            default_language: None,
        }
    }
}

fn default_cache_ttl_minutes() -> u64 {
    5 // 5 minutes cache
}

fn default_rate_limit_ms() -> u64 {
    100 // 100ms rate limit
}

fn default_period() -> String {
    "daily".to_string()
}

pub struct ConfigManager {
    config: Config,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> anyhow::Result<Self> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join(".gittype");

        fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.json");
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        Ok(ConfigManager {
            config,
            config_path,
        })
    }

    pub fn with_config_path(config_path: PathBuf) -> anyhow::Result<Self> {
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        Ok(ConfigManager {
            config,
            config_path,
        })
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }
}
