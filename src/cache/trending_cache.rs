use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingRepository {
    pub repo_name: String,
    pub primary_language: Option<String>,
    pub description: Option<String>,
    pub stars: String,
    pub forks: String,
    pub total_score: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrendingCacheData {
    repositories: Vec<TrendingRepository>,
    timestamp: u64, // Unix timestamp
    cache_key: String,
}

#[derive(Debug)]
pub struct TrendingCache {
    cache_dir: PathBuf,
    ttl_seconds: u64,
}

impl Default for TrendingCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TrendingCache {
    pub fn new() -> Self {
        let mut cache_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        cache_dir.push(".gittype");
        cache_dir.push("trending_cache");

        let ttl_seconds = 300; // 5 minutes

        Self {
            cache_dir,
            ttl_seconds,
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<TrendingRepository>> {
        let cache_file = self.get_cache_file(key);
        if !cache_file.exists() {
            return None;
        }

        let content = fs::read_to_string(&cache_file).ok()?;
        let cache_data: TrendingCacheData = serde_json::from_str(&content).ok()?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        // Check if cache is still valid
        if current_time.saturating_sub(cache_data.timestamp) < self.ttl_seconds {
            Some(cache_data.repositories)
        } else {
            // Remove expired cache file
            let _ = fs::remove_file(&cache_file);
            None
        }
    }

    pub fn set(&self, key: &str, repositories: Vec<TrendingRepository>) {
        let _ = fs::create_dir_all(&self.cache_dir);

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        let cache_data = TrendingCacheData {
            repositories,
            timestamp: current_time,
            cache_key: key.to_string(),
        };

        let cache_file = self.get_cache_file(key);
        if let Ok(content) = serde_json::to_string_pretty(&cache_data) {
            let _ = fs::write(&cache_file, content);
        }
    }

    pub fn cleanup_expired(&self) {
        if !self.cache_dir.exists() {
            return;
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();

        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(cache_data) = serde_json::from_str::<TrendingCacheData>(&content) {
                        if current_time.saturating_sub(cache_data.timestamp) >= self.ttl_seconds {
                            let _ = fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }
    }

    fn get_cache_file(&self, key: &str) -> PathBuf {
        // Create a safe filename from cache key
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

pub static TRENDING_CACHE: once_cell::sync::Lazy<TrendingCache> =
    once_cell::sync::Lazy::new(TrendingCache::new);
