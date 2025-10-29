use crate::infrastructure::http::oss_insight_client::OssInsightClientInterface;
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::Result;
use serde::{Deserialize, Serialize};
use shaku::Interface;
use std::path::PathBuf;
use std::sync::Arc;
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

pub trait TrendingRepositoryInterface: Interface {
    fn get_trending_repositories_sync(
        &self,
        key: &str,
        language: Option<&str>,
        period: &str,
    ) -> Result<Vec<TrendingRepositoryInfo>>;
}

#[derive(Debug, Clone, shaku::Component)]
#[shaku(interface = TrendingRepositoryInterface)]
pub struct TrendingRepository {
    #[shaku(default)]
    cache_dir: PathBuf,
    #[shaku(default)]
    ttl_seconds: u64,
    #[shaku(inject)]
    oss_insight_client: Arc<dyn OssInsightClientInterface>,
    #[shaku(inject)]
    file_storage: Arc<dyn crate::infrastructure::storage::file_storage::FileStorageInterface>,
}

impl TrendingRepository {
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

    /// Synchronous version of get_trending_repositories for use in non-async contexts
    pub fn get_trending_repositories_sync(
        &self,
        key: &str,
        language: Option<&str>,
        period: &str,
    ) -> Result<Vec<TrendingRepositoryInfo>> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.get_trending_repositories(key, language, period))
    }

    /// Get data from cache if valid
    fn get_from_cache(&self, key: &str) -> Option<Vec<TrendingRepositoryInfo>> {
        let cache_file = self.get_cache_file(key);
        if !self.file_storage.file_exists(&cache_file) {
            return None;
        }

        let file_storage =
            (self.file_storage.as_ref() as &dyn std::any::Any).downcast_ref::<FileStorage>()?;

        let cache_data: TrendingCacheData = file_storage.read_json(&cache_file).ok()??;

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

        if let Some(file_storage) =
            (self.file_storage.as_ref() as &dyn std::any::Any).downcast_ref::<FileStorage>()
        {
            let _ = file_storage.write_json(&cache_file, &cache_data);
        }
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

impl TrendingRepositoryInterface for TrendingRepository {
    fn get_trending_repositories_sync(
        &self,
        key: &str,
        language: Option<&str>,
        period: &str,
    ) -> Result<Vec<TrendingRepositoryInfo>> {
        TrendingRepository::get_trending_repositories_sync(self, key, language, period)
    }
}
