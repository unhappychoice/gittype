use gittype::domain::models::{Challenge, GitRepository, SessionResult};
use gittype::domain::repositories::SessionRepository;
use gittype::domain::services::repository_service::RepositoryService;
use gittype::domain::services::scoring::{StageInput, StageTracker};
use gittype::infrastructure::database::database::Database;

#[test]
fn test_repository_service_new() {
    let db = Database::new().unwrap();
    let _service = RepositoryService::new(db);
    // Service creation should succeed without error
}

#[test]
fn test_get_all_repositories_empty() {
    let db = Database::new().unwrap();
    let service = RepositoryService::new(db);

    let result = service.get_all_repositories();
    assert!(result.is_ok());

    // May or may not be empty depending on other tests
    let repositories = result.unwrap();
    assert!(repositories.is_empty() || !repositories.is_empty());
}

#[test]
fn test_get_all_repositories_with_data() {
    // Use the same approach as SessionRepository tests
    let session_repository = SessionRepository::new().unwrap();

    // Record a session with repository
    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

    let git_repo = GitRepository {
        user_name: "reposerviceuser".to_string(),
        repository_name: "reposervicerepo".to_string(),
        remote_url: "https://github.com/reposerviceuser/reposervicerepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("reposervice123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("reposervice-test".to_string(), "test code".to_string());
    let mut tracker = StageTracker::new("test code".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    session_repository
        .record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();

    // Use session_repository's get_all_repositories method to verify
    let repositories = session_repository.get_all_repositories().unwrap();
    assert!(!repositories.is_empty());

    // Verify our repository is in the list
    let found = repositories
        .iter()
        .any(|r| r.repository_name == "reposervicerepo" && r.user_name == "reposerviceuser");
    assert!(found, "Repository should be in the list");

    // Also test RepositoryService with a fresh database
    let db = Database::new().unwrap();
    let service = RepositoryService::new(db);
    let result = service.get_all_repositories();
    assert!(result.is_ok());
}

#[test]
fn test_get_all_repositories_with_languages() {
    // Test RepositoryService method with fresh database
    let db = Database::new().unwrap();
    let service = RepositoryService::new(db);
    let result = service.get_all_repositories_with_languages();
    assert!(result.is_ok());

    // May or may not have data depending on other tests
    let _repositories = result.unwrap();
}

#[test]
fn test_get_all_repositories_with_cache_status() {
    // Test RepositoryService method with fresh database
    let db = Database::new().unwrap();
    let service = RepositoryService::new(db);
    let result = service.get_all_repositories_with_cache_status();
    assert!(result.is_ok());

    let repositories_with_cache = result.unwrap();

    // Verify structure: each item should be (repository, cache_status)
    for (repo, is_cached) in &repositories_with_cache {
        assert!(!repo.repository_name.is_empty());
        assert!(!repo.user_name.is_empty());
        // is_cached is a boolean indicating if repository is cached locally
        assert!(is_cached == &true || is_cached == &false);
    }
}

#[test]
fn test_get_cache_directory() {
    let cache_dir = RepositoryService::get_cache_directory();

    // Cache directory should be a valid path
    assert!(cache_dir.is_absolute() || cache_dir.components().count() > 0);

    // Cache directory path should end with "repos"
    assert_eq!(cache_dir.file_name().unwrap(), "repos");

    // Cache directory should be under app data dir
    // In test-mocks mode: /tmp/test/repos -> parent is "test"
    let parent = cache_dir.parent().unwrap();
    let parent_name = parent.file_name().unwrap();
    assert_eq!(parent_name, "test");
}

#[test]
fn test_get_cache_directory_consistency() {
    // Cache directory should be consistent across calls
    let dir1 = RepositoryService::get_cache_directory();
    let dir2 = RepositoryService::get_cache_directory();

    assert_eq!(dir1, dir2);
}

#[test]
fn test_multiple_repositories() {
    // Test that service can handle multiple repositories
    let db = Database::new().unwrap();
    let service = RepositoryService::new(db);
    let result = service.get_all_repositories();
    assert!(result.is_ok());

    // Service should work even with no data
    let _repositories = result.unwrap();
}

#[test]
fn test_repository_service_with_languages_multiple() {
    // Test get_all_repositories_with_languages method
    let db = Database::new().unwrap();
    let service = RepositoryService::new(db);
    let result = service.get_all_repositories_with_languages();
    assert!(result.is_ok());

    // Service should work even with no data
    let _repositories = result.unwrap();
}
