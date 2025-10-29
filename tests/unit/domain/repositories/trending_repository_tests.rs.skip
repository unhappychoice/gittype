use gittype::domain::repositories::trending_repository::{
    TrendingRepository, TrendingRepositoryInfo,
};
use std::path::PathBuf;

fn create_test_trending_repository() -> TrendingRepository {
    TrendingRepository::with_cache_dir(PathBuf::from("/mock/trending_cache"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_trending_repositories_with_cache_miss() {
        let repo = create_test_trending_repository();

        // With mocked client, cache miss returns empty vec
        let result = repo
            .get_trending_repositories("test_key", Some("rust"), "daily")
            .await;
        assert!(result.is_ok());
        let repos = result.unwrap();
        assert!(repos.is_empty());
    }

    #[tokio::test]
    async fn test_get_trending_repositories_with_different_keys() {
        let repo = create_test_trending_repository();

        // With mocked client, both return empty vec
        let result1 = repo
            .get_trending_repositories("key1", Some("rust"), "daily")
            .await;
        let result2 = repo
            .get_trending_repositories("key2", Some("python"), "weekly")
            .await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result1.unwrap().is_empty());
        assert!(result2.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_trending_repositories_with_no_language() {
        let repo = create_test_trending_repository();

        // With mocked client, returns empty vec
        let result = repo
            .get_trending_repositories("no_lang_key", None, "monthly")
            .await;
        assert!(result.is_ok());
        let repos = result.unwrap();
        assert!(repos.is_empty());
    }

    #[tokio::test]
    async fn test_get_trending_repositories_graceful_degradation() {
        let repo = create_test_trending_repository();

        // With mocked client, always returns empty vec (graceful degradation)
        let result = repo
            .get_trending_repositories("fail_key", Some("invalid"), "invalid")
            .await;
        assert!(result.is_ok());
        let repos = result.unwrap();
        assert!(repos.is_empty());
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
    fn test_trending_repository_default() {
        let _repo = create_test_trending_repository();
        // Should not panic and create valid instance
    }

    #[test]
    fn test_trending_repository_new() {
        let _repo = create_test_trending_repository();
        // Should not panic and create valid instance
    }
}
