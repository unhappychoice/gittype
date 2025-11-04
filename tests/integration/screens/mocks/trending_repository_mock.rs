use gittype::domain::repositories::trending_repository::{
    TrendingRepositoryInfo, TrendingRepositoryInterface,
};
use gittype::Result;

pub struct MockTrendingRepository;

impl MockTrendingRepository {
    pub fn new() -> Self {
        MockTrendingRepository
    }
}

impl TrendingRepositoryInterface for MockTrendingRepository {
    fn get_trending_repositories_sync(
        &self,
        _key: &str,
        _language: Option<&str>,
        _period: &str,
    ) -> Result<Vec<TrendingRepositoryInfo>> {
        Ok(vec![
            TrendingRepositoryInfo {
                repo_name: "rust-lang/rust".to_string(),
                primary_language: Some("Rust".to_string()),
                description: Some(
                    "Empowering everyone to build reliable and efficient software.".to_string(),
                ),
                stars: "85000".to_string(),
                forks: "11000".to_string(),
                total_score: "100.0".to_string(),
            },
            TrendingRepositoryInfo {
                repo_name: "tokio-rs/tokio".to_string(),
                primary_language: Some("Rust".to_string()),
                description: Some(
                    "A runtime for writing reliable asynchronous applications with Rust."
                        .to_string(),
                ),
                stars: "20000".to_string(),
                forks: "2000".to_string(),
                total_score: "90.0".to_string(),
            },
        ])
    }
}
