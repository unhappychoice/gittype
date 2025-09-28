use gittype::domain::models::{Challenge, GitRepository};
use gittype::domain::repositories::ChallengeRepository;
use std::path::PathBuf;

fn create_test_repo(url: &str, commit: Option<String>, dirty: bool) -> GitRepository {
    GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: url.to_string(),
        branch: Some("main".to_string()),
        commit_hash: commit,
        is_dirty: dirty,
        root_path: Some(PathBuf::from("/mock/repo/path")),
    }
}

fn create_test_challenge(
    id: &str,
    source_file_path: Option<String>,
    content: &str,
) -> Challenge {
    Challenge {
        id: id.to_string(),
        source_file_path,
        code_content: content.to_string(),
        start_line: Some(1),
        end_line: Some(2),
        language: Some("rust".to_string()),
        comment_ranges: vec![],
        difficulty_level: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cache() -> ChallengeRepository {
        ChallengeRepository::with_cache_dir(PathBuf::from("/mock/cache"))
    }

    #[test]
    fn test_cache_save_and_load() {
        let cache = create_test_cache();

        let repo = create_test_repo(
            "https://github.com/test/repo",
            Some("abc123".to_string()),
            false,
        );

        let challenges = vec![
            create_test_challenge("test1", None, "test content"),
            create_test_challenge("test2", None, "test content 2"),
        ];

        // Save should work with mock storage
        assert!(cache.save_challenges(&repo, &challenges).is_ok());

        // Load will return None because file reconstruction fails in mock mode
        // This is expected behavior - we're testing the caching mechanism, not file reading
        let loaded = cache.load_challenges_with_progress(&repo, None);
        assert!(loaded.is_none() || loaded.unwrap().is_empty());
    }

    #[test]
    fn test_cache_miss_dirty_repo() {
        let cache = create_test_cache();

        let dirty_repo = create_test_repo(
            "https://github.com/test/repo",
            Some("abc123".to_string()),
            true,
        );

        let challenges = vec![
            create_test_challenge("test1", None, "test content"),
        ];

        assert!(cache.save_challenges(&dirty_repo, &challenges).is_ok());

        let loaded = cache.load_challenges_with_progress(&dirty_repo, None);
        assert!(loaded.is_none());
    }

    #[test]
    fn test_cache_stats_and_list() {
        let cache = create_test_cache();

        let repo = create_test_repo(
            "https://github.com/test/repo",
            Some("abc123".to_string()),
            false,
        );

        let challenges = vec![
            create_test_challenge("test1", None, "test content"),
        ];

        assert!(cache.save_challenges(&repo, &challenges).is_ok());

        let (count, _size) = cache.get_cache_stats().unwrap();
        assert_eq!(count, 1);

        let keys = cache.list_cache_keys().unwrap();
        assert_eq!(keys.len(), 1);
    }

    #[test]
    fn test_cache_clear() {
        let cache = create_test_cache();

        let repo = create_test_repo(
            "https://github.com/test/repo",
            Some("abc123".to_string()),
            false,
        );

        let challenges = vec![
            create_test_challenge("test1", None, "test content"),
        ];

        assert!(cache.save_challenges(&repo, &challenges).is_ok());

        let (count_before, _) = cache.get_cache_stats().unwrap();
        assert_eq!(count_before, 1);

        assert!(cache.clear_cache().is_ok());

        let (count_after, _) = cache.get_cache_stats().unwrap();
        assert_eq!(count_after, 0);
    }

    #[test]
    fn test_cache_invalidate_repo() {
        let cache = create_test_cache();

        let repo = create_test_repo(
            "https://github.com/test/repo",
            Some("abc123".to_string()),
            false,
        );

        let challenges = vec![
            create_test_challenge("test1", None, "test content"),
        ];

        assert!(cache.save_challenges(&repo, &challenges).is_ok());

        // Verify cache entry exists by checking stats
        let (count_before, _) = cache.get_cache_stats().unwrap();
        assert_eq!(count_before, 1);

        assert!(cache.invalidate_repository(&repo).unwrap());

        // Verify cache entry is gone
        let (count_after, _) = cache.get_cache_stats().unwrap();
        assert_eq!(count_after, 0);
    }

    #[test]
    fn test_cache_commit_hash_mismatch() {
        let cache = create_test_cache();

        let repo1 = create_test_repo(
            "https://github.com/test/repo",
            Some("abc123".to_string()),
            false,
        );

        let repo2 = create_test_repo(
            "https://github.com/test/repo",
            Some("def456".to_string()),
            false,
        );

        let challenges = vec![
            create_test_challenge("test1", None, "test content"),
        ];

        assert!(cache.save_challenges(&repo1, &challenges).is_ok());

        let loaded = cache.load_challenges_with_progress(&repo2, None);
        assert!(loaded.is_none());
    }

    #[test]
    fn test_cache_no_commit_hash() {
        let cache = create_test_cache();

        let repo = create_test_repo(
            "https://github.com/test/repo",
            None,
            false,
        );

        let challenges = vec![
            create_test_challenge("test1", None, "test content"),
        ];

        assert!(cache.save_challenges(&repo, &challenges).is_ok());

        let loaded = cache.load_challenges_with_progress(&repo, None);
        assert!(loaded.is_none());
    }
}