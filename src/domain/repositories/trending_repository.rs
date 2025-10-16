use crate::infrastructure::http::OssInsightClient;
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingRepositoryInfo {
    pub repo_name: String,
    pub primary_language: Option<String>,
    pub description: Option<String>,
    pub stars: String,
    pub forks: String,
    pub total_score: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrendingCacheData {
    repositories: Vec<TrendingRepositoryInfo>,
    timestamp: u64,
    cache_key: String,
}

#[derive(Debug)]
pub struct TrendingRepository {
    cache_dir: PathBuf,
    ttl_seconds: u64,
    oss_insight_client: OssInsightClient,
    file_storage: FileStorage,
}

impl Default for TrendingRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TrendingRepository {
    pub fn new() -> Self {
        let mut cache_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        cache_dir.push(".gittype");
        cache_dir.push("trending_cache");

        let ttl_seconds = 300; // 5 minutes

        Self {
            cache_dir,
            ttl_seconds,
            oss_insight_client: OssInsightClient::new(),
            file_storage: FileStorage::new(),
        }
    }

    pub fn with_cache_dir(cache_dir: PathBuf) -> Self {
        let ttl_seconds = 300; // 5 minutes

        Self {
            cache_dir,
            ttl_seconds,
            oss_insight_client: OssInsightClient::new(),
            file_storage: FileStorage::new(),
        }
    }

    /// Get trending repositories with caching and fallback to fresh data
    pub async fn get_trending_repositories(
        &self,
        key: &str,
        language: Option<&str>,
        period: &str,
    ) -> Result<Vec<TrendingRepositoryInfo>> {
        // Try cache first
        if let Some(cached_repos) = self.get_from_cache(key) {
            return Ok(cached_repos);
        }

        // Fetch fresh data from API
        match self.fetch_from_api(language, period).await {
            Ok(repos) => {
                // Cache the fresh data
                self.save_to_cache(key, &repos);
                Ok(repos)
            }
            Err(e) => {
                log::warn!("Failed to fetch trending repositories from API: {}", e);
                // Return empty vec instead of error for graceful degradation
                Ok(Vec::new())
            }
        }
    }

    /// Get data from cache if valid
    fn get_from_cache(&self, key: &str) -> Option<Vec<TrendingRepositoryInfo>> {
        let cache_file = self.get_cache_file(key);
        if !self.file_storage.file_exists(&cache_file) {
            return None;
        }

        let cache_data: TrendingCacheData = self.file_storage.read_json(&cache_file).ok()??;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        // Check if cache is still valid
        if current_time.saturating_sub(cache_data.timestamp) < self.ttl_seconds {
            Some(cache_data.repositories)
        } else {
            // Remove expired cache file
            let _ = self.file_storage.delete_file(&cache_file);
            None
        }
    }

    /// Save data to cache
    fn save_to_cache(&self, key: &str, repositories: &[TrendingRepositoryInfo]) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        let cache_data = TrendingCacheData {
            repositories: repositories.to_vec(),
            timestamp: current_time,
            cache_key: key.to_string(),
        };

        let cache_file = self.get_cache_file(key);
        let _ = self.file_storage.write_json(&cache_file, &cache_data);
    }

    /// Fetch fresh data from API
    async fn fetch_from_api(
        &self,
        language: Option<&str>,
        period: &str,
    ) -> Result<Vec<TrendingRepositoryInfo>> {
        let trending_repos = self
            .oss_insight_client
            .fetch_trending_repositories(language, period)
            .await?;

        let repositories = trending_repos
            .into_iter()
            .map(|repo| TrendingRepositoryInfo {
                repo_name: repo.repo_name,
                primary_language: repo.primary_language,
                description: repo.description,
                stars: repo.stars,
                forks: repo.forks,
                total_score: repo.total_score,
            })
            .collect();

        Ok(repositories)
    }

    fn get_cache_file(&self, key: &str) -> PathBuf {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let digest = hasher.finalize();
        let hex = digest
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        self.cache_dir.join(format!("{}.json", hex))
    }
}

pub static TRENDING_REPOSITORY: once_cell::sync::Lazy<TrendingRepository> =
    once_cell::sync::Lazy::new(TrendingRepository::new);
