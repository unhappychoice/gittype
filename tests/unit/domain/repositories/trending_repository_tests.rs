use gittype::domain::repositories::trending_repository::{
    TrendingRepositoryInfo, TrendingRepositoryInterface,
};
use gittype::presentation::di::AppModule;
use shaku::HasComponent;
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
