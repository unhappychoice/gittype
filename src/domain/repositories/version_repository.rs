use crate::domain::models::version::VersionCacheEntry;
use crate::infrastructure::http::GitHubApiClient;
use crate::infrastructure::storage::{file_storage::FileStorage, AppDataProvider};
use crate::Result;
use chrono::Utc;
use std::path::PathBuf;

pub struct VersionRepository {
    github_client: GitHubApiClient,
    file_storage: FileStorage,
}

impl VersionRepository {
    const VERSION_CACHE_FILENAME: &'static str = "version_cache.json";

    pub fn new() -> Result<Self> {
        Ok(Self {
            github_client: GitHubApiClient::new()?,
            file_storage: FileStorage::new(),
        })
    }

    /// Create a new repository for testing that doesn't make HTTP requests
    #[doc(hidden)]
    pub fn new_test() -> Result<Self> {
        Ok(Self {
            github_client: GitHubApiClient::new()?,
            file_storage: FileStorage::new(),
        })
    }

    /// Fetch the latest version from cache or API
    pub async fn fetch_latest_version(&self) -> Result<String> {
        const CHECK_FREQUENCY_HOURS: u64 = 24;

        // Check cache first
        if let Some(cached_entry) = self.get_cached_version()? {
            if self.is_cache_valid(&cached_entry, CHECK_FREQUENCY_HOURS) {
                return Ok(cached_entry.latest_version);
            }
        }

        // Fetch from API
        match self.fetch_from_api().await {
            Ok(latest_version) => {
                self.save_to_cache(&latest_version)?;
                Ok(latest_version)
            }
            Err(e) => {
                log::warn!("Failed to fetch latest version from API: {}", e);
                // Fall back to cached version if available
                if let Some(cached_entry) = self.get_cached_version()? {
                    Ok(cached_entry.latest_version)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Get cached version information
    fn get_cached_version(&self) -> Result<Option<VersionCacheEntry>> {
        if cfg!(debug_assertions) {
            return Ok(None);
        }

        let cache_path = self.get_version_cache_path()?;
        self.file_storage.read_json(&cache_path)
    }

    /// Save version information to cache
    fn save_to_cache(&self, latest_version: &str) -> Result<()> {
        if cfg!(debug_assertions) {
            return Ok(());
        }

        let entry = VersionCacheEntry {
            latest_version: latest_version.to_string(),
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            update_available: false, // not used
            last_checked: Utc::now(),
        };

        let cache_path = self.get_version_cache_path()?;
        self.file_storage.write_json(&cache_path, &entry)
    }

    /// Fetch the latest version from GitHub API
    async fn fetch_from_api(&self) -> Result<String> {
        let release = self.github_client.fetch_latest_release().await?;
        let version = Self::normalize_version_tag(&release.tag_name);
        Ok(version)
    }

    fn get_version_cache_path(&self) -> Result<PathBuf> {
        let app_dir = <FileStorage as AppDataProvider>::get_app_data_dir()?;
        Ok(app_dir.join(Self::VERSION_CACHE_FILENAME))
    }

    /// Check if a cache entry is still valid
    fn is_cache_valid(&self, entry: &VersionCacheEntry, frequency_hours: u64) -> bool {
        let now = chrono::Utc::now();
        let hours_since_check = (now - entry.last_checked).num_hours();
        let time_valid = hours_since_check < frequency_hours as i64;

        // Also check if the current version matches
        let current_version = env!("CARGO_PKG_VERSION");
        let version_valid = entry.current_version == current_version;

        time_valid && version_valid
    }

    /// Strip 'v' prefix from version tag if present
    fn normalize_version_tag(tag: &str) -> String {
        tag.strip_prefix('v').unwrap_or(tag).to_string()
    }
}
