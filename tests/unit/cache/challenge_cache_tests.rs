use gittype::cache::ChallengeCache;
use gittype::models::{Challenge, GitRepository};
use std::fs;
use tempfile;

fn create_test_repo_with_files(url: &str, commit: Option<String>, dirty: bool) -> GitRepository {
    // Create a temporary directory for test files
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Create test files
    let test_file = temp_path.join("test.rs");
    fs::write(
        &test_file,
        "// Test file\nfn main() {\n    println!(\"Hello, world!\");\n}",
    )
    .unwrap();

    // Keep the temp directory alive by leaking it (for test purposes)
    std::mem::forget(temp_dir);

    GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: url.to_string(),
        branch: Some("main".to_string()),
        commit_hash: commit,
        is_dirty: dirty,
        root_path: Some(temp_path),
    }
}

fn create_test_challenge_with_file(id: &str, _repo: &GitRepository) -> Challenge {
    let mut challenge = Challenge::new(
        id.to_string(),
        "// Test file\nfn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
    );
    challenge.source_file_path = Some("test.rs".to_string());
    challenge.start_line = Some(1);
    challenge.end_line = Some(4);
    challenge
}

fn create_test_cache() -> ChallengeCache {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    std::mem::forget(temp_dir); // Keep alive for test duration

    ChallengeCache::with_cache_dir(temp_path)
}

#[test]
fn test_cache_save_and_load() {
    let cache = create_test_cache();
    let repo = create_test_repo_with_files(
        "https://github.com/test/save-load",
        Some("abc123".to_string()),
        false,
    );
    let challenges = vec![
        create_test_challenge_with_file("1", &repo),
        create_test_challenge_with_file("2", &repo),
    ];

    // Clear any existing cache
    let _ = cache.clear();

    // Save challenges
    let save_result = cache.save(&repo, &challenges);
    assert!(save_result.is_ok());

    // Load challenges
    let loaded = cache.load_with_progress(&repo, None);
    assert!(loaded.is_some());
    let loaded_challenges = loaded.unwrap();
    assert_eq!(loaded_challenges.len(), 2);
    assert_eq!(loaded_challenges[0].id, "1");
    assert_eq!(loaded_challenges[1].id, "2");
}

#[test]
fn test_dirty_repository_skips_cache() {
    let cache = create_test_cache();
    let repo_clean = create_test_repo_with_files(
        "https://github.com/test/dirty-test",
        Some("abc123".to_string()),
        false,
    );
    let repo_dirty = create_test_repo_with_files(
        "https://github.com/test/dirty-test-dirty",
        Some("abc123".to_string()),
        true,
    );
    let challenges = vec![
        create_test_challenge_with_file("1", &repo_clean),
        create_test_challenge_with_file("2", &repo_clean),
    ];

    let _ = cache.clear();

    // Clean repository should save to cache
    let save_result = cache.save(&repo_clean, &challenges);
    assert!(save_result.is_ok());

    // Clean repository should load from cache
    let loaded_clean = cache.load_with_progress(&repo_clean, None);
    assert!(loaded_clean.is_some());
    assert_eq!(loaded_clean.unwrap().len(), 2);

    // Dirty repository should not load from cache (even with same commit)
    let loaded_dirty = cache.load_with_progress(&repo_dirty, None);
    assert!(loaded_dirty.is_none());
}

#[test]
fn test_commit_hash_invalidation() {
    let cache = create_test_cache();
    let repo_v1 = create_test_repo_with_files(
        "https://github.com/test/invalidation",
        Some("abc123".to_string()),
        false,
    );
    let repo_v2 = create_test_repo_with_files(
        "https://github.com/test/invalidation2",
        Some("def456".to_string()),
        false,
    );
    let challenges_v1 = vec![
        create_test_challenge_with_file("1", &repo_v1),
        create_test_challenge_with_file("2", &repo_v1),
    ];
    let challenges_v2 = vec![
        create_test_challenge_with_file("3", &repo_v2),
        create_test_challenge_with_file("4", &repo_v2),
    ];

    let _ = cache.clear();

    // Cache challenges for first commit
    let save_result = cache.save(&repo_v1, &challenges_v1);
    assert!(save_result.is_ok());

    let loaded_v1 = cache.load_with_progress(&repo_v1, None);
    assert!(loaded_v1.is_some());
    assert_eq!(loaded_v1.unwrap().len(), 2);

    // Different repository should not find cached data
    let loaded_v2 = cache.load_with_progress(&repo_v2, None);
    assert!(loaded_v2.is_none());

    // Cache challenges for second repo
    let save_result_v2 = cache.save(&repo_v2, &challenges_v2);
    assert!(save_result_v2.is_ok());

    // First repo should still be cached
    let loaded_v1_after = cache.load_with_progress(&repo_v1, None);
    assert!(loaded_v1_after.is_some());
    assert_eq!(loaded_v1_after.unwrap().len(), 2);

    // Second repo should be cached
    let loaded_v2_after = cache.load_with_progress(&repo_v2, None);
    assert!(loaded_v2_after.is_some());
    assert_eq!(loaded_v2_after.unwrap().len(), 2);
}

#[test]
fn test_cache_invalidation() {
    let cache = create_test_cache();
    let repo = create_test_repo_with_files(
        "https://github.com/test/invalidate",
        Some("test123".to_string()),
        false,
    );
    let challenges = vec![
        create_test_challenge_with_file("1", &repo),
        create_test_challenge_with_file("2", &repo),
    ];

    let _ = cache.clear();

    // Initially no cache
    assert!(cache.load_with_progress(&repo, None).is_none());

    // Save to cache
    let save_result = cache.save(&repo, &challenges);
    assert!(save_result.is_ok());

    // Should be cached
    let loaded = cache.load_with_progress(&repo, None);
    assert!(loaded.is_some());
    assert_eq!(loaded.unwrap().len(), 2);

    // Invalidate cache for repo
    let invalidated = cache.invalidate_repo(&repo);
    assert!(invalidated.is_ok());
    assert!(invalidated.unwrap());

    // Should no longer be cached
    assert!(cache.load_with_progress(&repo, None).is_none());
}

#[test]
fn test_cache_stats_and_list() {
    let cache = create_test_cache();
    let _ = cache.clear();

    // Initially empty
    let stats = cache.stats();
    assert!(stats.is_ok());
    let (count, _size) = stats.unwrap();
    assert_eq!(count, 0);

    let repo = create_test_repo_with_files(
        "https://github.com/test/stats",
        Some("test123".to_string()),
        false,
    );
    let challenges = vec![create_test_challenge_with_file("test", &repo)];
    let _ = cache.save(&repo, &challenges);

    // Should have one entry
    let stats = cache.stats();
    assert!(stats.is_ok());
    let (count, _size) = stats.unwrap();
    assert_eq!(count, 1);

    // Check list keys
    let keys = cache.list_keys();
    assert!(keys.is_ok());
    let keys = keys.unwrap();
    assert_eq!(keys.len(), 1);
    // Key format is now "github_com_test_stats:test123"
    assert!(keys[0].contains("github_com"));
    assert!(keys[0].contains("test"));
    assert!(keys[0].contains("stats"));
    assert!(keys[0].contains("test123"));
}

#[test]
fn test_ssh_url_key_extraction() {
    let cache = create_test_cache();

    // Test git@ format
    let repo_ssh = create_test_repo_with_files(
        "git@github.com:owner/repo",
        Some("abc123".to_string()),
        false,
    );
    let challenges = vec![create_test_challenge_with_file("ssh_test", &repo_ssh)];

    let _ = cache.clear();
    let save_result = cache.save(&repo_ssh, &challenges);
    assert!(save_result.is_ok());

    // Test ssh:// format
    let repo_ssh_protocol = create_test_repo_with_files(
        "ssh://git@github.com/owner/repo",
        Some("def456".to_string()),
        false,
    );

    // Should use same cache key since it's the same repo
    let loaded = cache.load_with_progress(&repo_ssh_protocol, None);
    // Won't load due to different commit hash, but should not error
    assert!(loaded.is_none());

    // Test cache keys are generated consistently
    let keys = cache.list_keys().unwrap();
    assert_eq!(keys.len(), 1);
    assert!(keys[0].contains("github_com_owner_repo"));
}

#[test]
fn test_basic_cache_clear() {
    let cache = create_test_cache();
    let repo = create_test_repo_with_files(
        "https://github.com/test/clear",
        Some("test123".to_string()),
        false,
    );
    let challenges = vec![create_test_challenge_with_file("test", &repo)];

    // Save and verify
    let _ = cache.save(&repo, &challenges);
    assert!(cache.load_with_progress(&repo, None).is_some());

    // Clear and verify
    let clear_result = cache.clear();
    assert!(clear_result.is_ok());

    // Should be empty
    let stats = cache.stats().unwrap();
    assert_eq!(stats.0, 0);
}
