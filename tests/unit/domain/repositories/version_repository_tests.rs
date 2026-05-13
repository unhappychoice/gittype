use chrono::{Duration, Utc};
use gittype::domain::models::version::VersionCacheEntry;
use gittype::domain::repositories::version_repository::VersionRepository;
use gittype::infrastructure::http::github_api_client::{GitHubApiClient, GitHubApiClientFactory};
use gittype::{GitTypeError, Result};
use std::sync::Arc;

#[test]
fn new_creates_repository() {
    let result = VersionRepository::new_for_test();
    assert!(result.is_ok());
}

#[tokio::test]
async fn fetch_latest_version_returns_normalized_api_release() {
    let repository = VersionRepository::new_for_test().unwrap();

    let version = repository.fetch_latest_version().await.unwrap();

    assert_eq!(version, "1.0.0");
}

struct FailingGitHubApiClientFactory;

impl GitHubApiClientFactory for FailingGitHubApiClientFactory {
    fn create(&self) -> Result<GitHubApiClient> {
        Err(GitTypeError::ExtractionFailed(
            "GitHub client unavailable".to_string(),
        ))
    }
}

fn cache_entry(current_version: &str, hours_ago: i64) -> VersionCacheEntry {
    VersionCacheEntry {
        latest_version: "1.2.3".to_string(),
        current_version: current_version.to_string(),
        update_available: false,
        last_checked: Utc::now() - Duration::hours(hours_ago),
    }
}

#[tokio::test]
async fn fetch_latest_version_returns_api_error_without_cache_fallback() {
    let repository =
        VersionRepository::new_for_test_with_factory(Arc::new(FailingGitHubApiClientFactory));

    let result = repository.fetch_latest_version().await;

    assert!(matches!(
        result,
        Err(GitTypeError::ExtractionFailed(message)) if message == "GitHub client unavailable"
    ));
}

#[test]
fn is_cache_valid_accepts_fresh_current_version_entry() {
    let entry = cache_entry(env!("CARGO_PKG_VERSION"), 1);
    let repository = VersionRepository::new_for_test().unwrap();

    assert!(repository.is_cache_valid_for_test(&entry, 24));
}

#[test]
fn is_cache_valid_rejects_stale_entry() {
    let entry = cache_entry(env!("CARGO_PKG_VERSION"), 25);
    let repository = VersionRepository::new_for_test().unwrap();

    assert!(!repository.is_cache_valid_for_test(&entry, 24));
}

#[test]
fn is_cache_valid_rejects_different_current_version() {
    let entry = cache_entry("0.0.0", 1);
    let repository = VersionRepository::new_for_test().unwrap();

    assert!(!repository.is_cache_valid_for_test(&entry, 24));
}

#[test]
fn normalize_version_tag_strips_lowercase_v_prefix() {
    assert_eq!(
        VersionRepository::normalize_version_tag_for_test("v1.2.3"),
        "1.2.3"
    );
}

#[test]
fn normalize_version_tag_preserves_unprefixed_tag() {
    assert_eq!(
        VersionRepository::normalize_version_tag_for_test("1.2.3"),
        "1.2.3"
    );
}
