use crate::domain::models::version::VersionCacheEntry;
use crate::infrastructure::version::cache::VersionCache;
use crate::game::screens::{VersionCheckResult, VersionCheckScreen};
use crate::{GitTypeError, Result};
use chrono::Utc;
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

pub struct VersionChecker;

impl VersionChecker {
    const GITHUB_API_URL: &'static str =
        "https://api.github.com/repos/unhappychoice/gittype/releases/latest";
    pub const CURRENT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub async fn check_for_updates() -> Result<Option<VersionCacheEntry>> {
        const CHECK_FREQUENCY_HOURS: u64 = 24;

        if let Some(cached_entry) = VersionCache::get()? {
            if VersionCache::is_cache_valid(&cached_entry, CHECK_FREQUENCY_HOURS) {
                return Ok(Some(cached_entry));
            }
        }

        match Self::fetch_latest_version().await {
            Ok(latest_version) => {
                let current_version = Self::CURRENT_VERSION.to_string();
                let update_available = Self::is_version_newer(&latest_version, &current_version);

                let entry = VersionCacheEntry {
                    latest_version,
                    current_version,
                    update_available,
                    last_checked: Utc::now(),
                };

                VersionCache::set(&entry)?;
                Ok(Some(entry))
            }
            Err(e) => {
                log::warn!("Failed to check for updates: {}", e);
                if let Some(cached_entry) = VersionCache::get()? {
                    Ok(Some(cached_entry))
                } else {
                    Ok(None)
                }
            }
        }
    }

    async fn fetch_latest_version() -> Result<String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent(format!("gittype/{}", Self::CURRENT_VERSION))
            .build()
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to create HTTP client: {}", e))
            })?;

        let response = client.get(Self::GITHUB_API_URL).send().await.map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Failed to fetch latest version: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(GitTypeError::ExtractionFailed(format!(
                "GitHub API returned status: {}",
                response.status()
            )));
        }

        let release: GitHubRelease = response.json().await.map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Failed to parse GitHub API response: {}", e))
        })?;

        let version = release
            .tag_name
            .strip_prefix('v')
            .unwrap_or(&release.tag_name);
        Ok(version.to_string())
    }

    pub fn is_version_newer(latest: &str, current: &str) -> bool {
        let latest_parts = Self::parse_version(latest);
        let current_parts = Self::parse_version(current);

        match (latest_parts, current_parts) {
            (Ok(latest), Ok(current)) => {
                for (l, c) in latest.iter().zip(current.iter()) {
                    if l > c {
                        return true;
                    } else if l < c {
                        return false;
                    }
                }
                latest.len() > current.len()
            }
            _ => false,
        }
    }

    pub fn parse_version(version: &str) -> Result<Vec<u32>> {
        version
            .split('.')
            .map(|part| {
                part.parse::<u32>().map_err(|e| {
                    GitTypeError::ExtractionFailed(format!("Invalid version format: {}", e))
                })
            })
            .collect()
    }

    pub fn display_update_notification(entry: &VersionCacheEntry) -> Result<bool> {
        if entry.update_available {
            match VersionCheckScreen::show_legacy(&entry.current_version, &entry.latest_version)? {
                VersionCheckResult::Continue => Ok(true),
                VersionCheckResult::Exit => Ok(false),
            }
        } else {
            Ok(true)
        }
    }
}