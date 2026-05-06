use gittype::domain::repositories::version_repository::VersionRepository;

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
