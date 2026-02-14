use gittype::domain::models::{Challenge, DifficultyLevel, GitRepository};
use gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use gittype::presentation::di::AppModule;
use shaku::HasComponent;
use std::path::PathBuf;
use std::sync::Arc;

fn create_repository() -> Arc<dyn ChallengeRepositoryInterface> {
    let module = AppModule::builder().build();
    module.resolve()
}

fn create_test_repo(commit: Option<String>, dirty: bool) -> GitRepository {
    GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: commit,
        is_dirty: dirty,
        root_path: Some(PathBuf::from("/tmp/mock-repo")),
    }
}

fn create_test_challenge(id: &str, content: &str) -> Challenge {
    Challenge::new(id.to_string(), content.to_string())
        .with_language("rust".to_string())
        .with_difficulty_level(DifficultyLevel::Normal)
}

#[test]
fn test_creates_repository_via_di() {
    let _repo = create_repository();
}

#[test]
fn test_get_cache_stats_empty() {
    let repo = create_repository();
    let result = repo.get_cache_stats();
    assert!(result.is_ok());
}

#[test]
fn test_clear_cache_succeeds() {
    let repo = create_repository();
    let result = repo.clear_cache();
    assert!(result.is_ok());
}

#[test]
fn test_list_cache_keys_succeeds() {
    let repo = create_repository();
    let result = repo.list_cache_keys();
    assert!(result.is_ok());
}

#[test]
fn test_save_challenges_dirty_repo_is_noop() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("abc123".to_string()), true);
    let challenges = vec![create_test_challenge("t1", "fn main() {}")];

    let result = repo.save_challenges(&git_repo, &challenges, None);
    assert!(result.is_ok());
}

#[test]
fn test_save_challenges_no_commit_hash_is_noop() {
    let repo = create_repository();
    let git_repo = create_test_repo(None, false);
    let challenges = vec![create_test_challenge("t1", "fn main() {}")];

    let result = repo.save_challenges(&git_repo, &challenges, None);
    assert!(result.is_ok());
}

#[test]
fn test_save_challenges_empty_commit_hash_is_noop() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("".to_string()), false);
    let challenges = vec![create_test_challenge("t1", "fn main() {}")];

    let result = repo.save_challenges(&git_repo, &challenges, None);
    assert!(result.is_ok());
}

#[test]
fn test_save_challenges_valid_repo() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("abc123".to_string()), false);
    let challenges = vec![
        create_test_challenge("t1", "fn main() {}"),
        create_test_challenge("t2", "fn test() {}"),
    ];

    let result = repo.save_challenges(&git_repo, &challenges, None);
    assert!(result.is_ok());
}

#[test]
fn test_save_challenges_empty_list() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("abc123".to_string()), false);

    let result = repo.save_challenges(&git_repo, &[], None);
    assert!(result.is_ok());
}

#[test]
fn test_load_challenges_dirty_repo_returns_none() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("abc123".to_string()), true);

    let result = repo.load_challenges_with_progress(&git_repo, None);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_load_challenges_cache_miss_returns_none() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("nonexistent".to_string()), false);

    let result = repo.load_challenges_with_progress(&git_repo, None);
    assert!(result.is_ok());
    let loaded = result.unwrap();
    assert!(loaded.is_none() || loaded.unwrap().is_empty());
}

#[test]
fn test_invalidate_repository_nonexistent() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("xxx".to_string()), false);

    let result = repo.invalidate_repository(&git_repo);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_save_then_invalidate() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("save-then-invalidate".to_string()), false);
    let challenges = vec![create_test_challenge("t1", "fn main() {}")];

    repo.save_challenges(&git_repo, &challenges, None).unwrap();

    let result = repo.invalidate_repository(&git_repo);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_save_then_get_cache_stats() {
    let repo = create_repository();
    let git_repo = create_test_repo(Some("stats-test".to_string()), false);
    let challenges = vec![create_test_challenge("t1", "fn main() {}")];

    repo.save_challenges(&git_repo, &challenges, None).unwrap();

    let (count, size) = repo.get_cache_stats().unwrap();
    assert!(count >= 1);
    assert!(size > 0);
}

#[test]
fn test_save_then_clear_cache() {
    // Each test uses its own repository instance to avoid shared state
    let repo = create_repository();
    let git_repo = create_test_repo(Some("clear-test".to_string()), false);
    let challenges = vec![create_test_challenge("t1", "fn main() {}")];

    repo.save_challenges(&git_repo, &challenges, None).unwrap();
    repo.clear_cache().unwrap();

    let (count, _) = repo.get_cache_stats().unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_save_then_list_cache_keys() {
    // Each test uses its own repository instance to avoid shared state
    let repo = create_repository();
    repo.clear_cache().unwrap();

    let git_repo = create_test_repo(Some("list-keys".to_string()), false);
    let challenges = vec![create_test_challenge("t1", "fn main() {}")];

    repo.save_challenges(&git_repo, &challenges, None).unwrap();

    let keys = repo.list_cache_keys().unwrap();
    assert!(!keys.is_empty());
}

#[test]
fn test_commit_hash_mismatch_returns_none() {
    let repo = create_repository();
    let git_repo1 = create_test_repo(Some("commit-a".to_string()), false);
    let git_repo2 = create_test_repo(Some("commit-b".to_string()), false);

    let challenges = vec![create_test_challenge("t1", "fn main() {}")];
    repo.save_challenges(&git_repo1, &challenges, None).unwrap();

    let result = repo.load_challenges_with_progress(&git_repo2, None);
    assert!(result.is_ok());
    let loaded = result.unwrap();
    assert!(loaded.is_none() || loaded.unwrap().is_empty());
}
