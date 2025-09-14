use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCacheEntry {
    pub latest_version: String,
    pub current_version: String,
    pub update_available: bool,
    pub last_checked: DateTime<Utc>,
}

#[derive(Debug)]
pub struct VersionCache;

impl VersionCache {
    fn cache_path() -> crate::Result<PathBuf> {
        let cache_dir = if cfg!(debug_assertions) {
            std::env::current_dir().map_err(|e| {
                crate::GitTypeError::ExtractionFailed(format!(
                    "Could not get current directory: {}",
                    e
                ))
            })?
        } else {
            let home_dir = dirs::home_dir().ok_or_else(|| {
                crate::GitTypeError::ExtractionFailed(
                    "Could not determine home directory".to_string(),
                )
            })?;
            home_dir.join(".gittype")
        };

        std::fs::create_dir_all(&cache_dir)?;
        Ok(cache_dir.join("version_cache.json"))
    }

    pub fn get() -> crate::Result<Option<VersionCacheEntry>> {
        if cfg!(debug_assertions) {
            // No cache in development mode
            return Ok(None);
        }

        let cache_path = Self::cache_path()?;
        if cache_path.exists() {
            let contents = std::fs::read_to_string(&cache_path)?;
            let entry: VersionCacheEntry = serde_json::from_str(&contents).map_err(|e| {
                crate::GitTypeError::ExtractionFailed(format!("Failed to parse cache: {}", e))
            })?;
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

    pub fn set(entry: &VersionCacheEntry) -> crate::Result<()> {
        if cfg!(debug_assertions) {
            // No cache in development mode
            return Ok(());
        }

        let cache_path = Self::cache_path()?;
        let contents = serde_json::to_string_pretty(entry).map_err(|e| {
            crate::GitTypeError::ExtractionFailed(format!("Failed to serialize cache: {}", e))
        })?;
        std::fs::write(&cache_path, contents)?;
        Ok(())
    }

    pub fn is_cache_valid(entry: &VersionCacheEntry, frequency_hours: u64) -> bool {
        let now = Utc::now();
        let hours_since_check = (now - entry.last_checked).num_hours();
        let time_valid = hours_since_check < frequency_hours as i64;

        // Also check if the current version matches
        let current_version = env!("CARGO_PKG_VERSION");
        let version_valid = entry.current_version == current_version;

        time_valid && version_valid
    }
}
