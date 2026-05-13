use gittype::domain::repositories::trending_repository::{
    TrendingRepository, TrendingRepositoryInfo, TrendingRepositoryInterface,
};
use gittype::infrastructure::http::oss_insight_client::OssInsightClientInterface;
use gittype::presentation::di::AppModule;
use gittype::{GitTypeError, Result};
use shaku::HasComponent;
use std::path::PathBuf;
use std::sync::Arc;

fn create_repository() -> Arc<dyn TrendingRepositoryInterface> {
    let module = AppModule::builder().build();
    module.resolve()
}

fn create_test_trending_info(repo_name: &str, language: Option<&str>) -> TrendingRepositoryInfo {
    TrendingRepositoryInfo {
        repo_name: repo_name.to_string(),
        primary_language: language.map(|s| s.to_string()),
        description: Some("Test repository description".to_string()),
        stars: "100".to_string(),
        forks: "50".to_string(),
        total_score: "150".to_string(),
    }
}

#[test]
fn test_creates_repository_via_di() {
    let _repo = create_repository();
}

#[test]
fn test_get_trending_repositories_sync_returns_ok() {
    let repo = create_repository();
    let result = repo.get_trending_repositories_sync("test_key", Some("rust"), "daily");
    assert!(result.is_ok());
}

#[test]
fn test_get_trending_repositories_sync_with_no_language() {
    let repo = create_repository();
    let result = repo.get_trending_repositories_sync("no_lang", None, "monthly");
    assert!(result.is_ok());
}

#[test]
fn test_get_trending_repositories_sync_different_periods() {
    let repo = create_repository();

    let daily = repo.get_trending_repositories_sync("d", Some("rust"), "daily");
    let weekly = repo.get_trending_repositories_sync("w", Some("rust"), "weekly");
    let monthly = repo.get_trending_repositories_sync("m", Some("rust"), "monthly");

    assert!(daily.is_ok());
    assert!(weekly.is_ok());
    assert!(monthly.is_ok());
}

#[test]
fn test_trending_repository_info_creation() {
    let info = create_test_trending_info("test/repo", Some("rust"));

    assert_eq!(info.repo_name, "test/repo");
    assert_eq!(info.primary_language, Some("rust".to_string()));
    assert_eq!(
        info.description,
        Some("Test repository description".to_string())
    );
    assert_eq!(info.stars, "100");
    assert_eq!(info.forks, "50");
    assert_eq!(info.total_score, "150");
}

#[test]
fn test_trending_repository_info_no_language() {
    let info = create_test_trending_info("test/repo", None);

    assert_eq!(info.repo_name, "test/repo");
    assert_eq!(info.primary_language, None);
}

#[test]
fn test_trending_repository_info_clone() {
    let info = create_test_trending_info("test/repo", Some("python"));
    let cloned = info.clone();

    assert_eq!(info.repo_name, cloned.repo_name);
    assert_eq!(info.primary_language, cloned.primary_language);
    assert_eq!(info.stars, cloned.stars);
}

#[derive(Debug)]
struct FakeOssInsightClient {
    result: std::result::Result<Vec<TrendingRepositoryInfo>, String>,
}

#[async_trait::async_trait]
impl OssInsightClientInterface for FakeOssInsightClient {
    async fn fetch_trending_repositories(
        &self,
        _language: Option<&str>,
        _period: &str,
    ) -> Result<Vec<TrendingRepositoryInfo>> {
        self.result.clone().map_err(GitTypeError::ApiError)
    }
}

fn repository_with_client(client: FakeOssInsightClient) -> TrendingRepository {
    TrendingRepository::new_for_test(PathBuf::from("unused-cache-dir"), 60, Arc::new(client))
}

fn trending_info_with_rust(repo_name: &str) -> TrendingRepositoryInfo {
    TrendingRepositoryInfo {
        repo_name: repo_name.to_string(),
        primary_language: Some("Rust".to_string()),
        description: Some("A test repository".to_string()),
        stars: "42".to_string(),
        forks: "7".to_string(),
        total_score: "49".to_string(),
    }
}

#[tokio::test]
async fn get_trending_repositories_returns_fresh_api_data() {
    let repository = repository_with_client(FakeOssInsightClient {
        result: Ok(vec![trending_info_with_rust("owner/repo")]),
    });

    let repositories = repository
        .get_trending_repositories("fresh-key", Some("rust"), "daily")
        .await
        .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].repo_name, "owner/repo");
    assert_eq!(repositories[0].primary_language, Some("Rust".to_string()));
}

#[tokio::test]
async fn get_trending_repositories_returns_empty_vec_when_api_fails() {
    let repository = repository_with_client(FakeOssInsightClient {
        result: Err("service unavailable".to_string()),
    });

    let repositories = repository
        .get_trending_repositories("error-key", Some("rust"), "weekly")
        .await
        .unwrap();

    assert!(repositories.is_empty());
}
