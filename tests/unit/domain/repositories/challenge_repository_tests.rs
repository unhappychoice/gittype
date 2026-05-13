use gittype::domain::models::loading::StepType;
use gittype::domain::models::{Challenge, DifficultyLevel, GitRepository};
use gittype::domain::repositories::challenge_repository::{
    ChallengeRepository, ChallengeRepositoryInterface,
};
use gittype::infrastructure::storage::file_storage::FileStorage;
use gittype::infrastructure::storage::file_storage::FileStorageInterface;
use gittype::presentation::di::AppModule;
use gittype::presentation::tui::screens::loading_screen::ProgressReporter;
use shaku::HasComponent;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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

fn file_storage_with_source(source_path: PathBuf, content: &str) -> Arc<dyn FileStorageInterface> {
    let mut file_storage = FileStorage::new();
    file_storage.set_file_content(source_path, content.to_string());
    Arc::new(file_storage)
}

#[test]
fn load_challenges_reconstructs_saved_source_slice() {
    let temp_dir = tempfile::tempdir().unwrap();
    let source_path = temp_dir.path().join("repo/src/lib.rs");
    let source = "fn alpha() {}\nfn beta() {}\nfn gamma() {}\n";
    std::fs::create_dir_all(source_path.parent().unwrap()).unwrap();
    std::fs::write(&source_path, source).unwrap();

    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        file_storage_with_source(source_path.canonicalize().unwrap(), source),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-success-{}", std::process::id())),
        is_dirty: false,
        root_path: Some(temp_dir.path().join("repo")),
    };
    let challenge = Challenge::new("t1".to_string(), "fn beta() {}".to_string())
        .with_source_info("src/lib.rs".to_string(), 2, 2)
        .with_language("rust".to_string())
        .with_comment_ranges(vec![(0, 2)])
        .with_difficulty_level(DifficultyLevel::Normal);

    repository
        .save_challenges(&git_repository, &[challenge])
        .unwrap();

    let loaded = repository
        .load_challenges_with_progress(&git_repository, None)
        .expect("saved challenge should be reconstructed");

    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].id, "t1");
    assert_eq!(loaded[0].code_content, "fn beta() {}");
    assert_eq!(loaded[0].source_file_path.as_deref(), Some("src/lib.rs"));
    assert_eq!(loaded[0].start_line, Some(2));
    assert_eq!(loaded[0].end_line, Some(2));
    assert_eq!(loaded[0].language.as_deref(), Some("rust"));
    assert_eq!(loaded[0].comment_ranges, vec![(0, 2)]);
    assert_eq!(loaded[0].difficulty_level, Some(DifficultyLevel::Normal));
}

#[test]
fn load_challenges_uses_whole_file_when_lines_are_absent() {
    let temp_dir = tempfile::tempdir().unwrap();
    let source_path = temp_dir.path().join("repo/src/lib.rs");
    let source = "fn whole() {}\nfn file() {}\n";
    std::fs::create_dir_all(source_path.parent().unwrap()).unwrap();
    std::fs::write(&source_path, source).unwrap();

    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        file_storage_with_source(source_path.canonicalize().unwrap(), source),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-whole-file-{}", std::process::id())),
        is_dirty: false,
        root_path: Some(temp_dir.path().join("repo")),
    };
    let challenge = Challenge {
        id: "no-lines".to_string(),
        source_file_path: Some("src/lib.rs".to_string()),
        code_content: "placeholder".to_string(),
        start_line: None,
        end_line: None,
        language: Some("rust".to_string()),
        comment_ranges: Vec::new(),
        difficulty_level: Some(DifficultyLevel::Easy),
    };

    repository
        .save_challenges(&git_repository, &[challenge])
        .unwrap();

    let loaded = repository
        .load_challenges_with_progress(&git_repository, None)
        .expect("challenge without line info should reconstruct full file");

    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].code_content, source);
    assert_eq!(loaded[0].start_line, None);
    assert_eq!(loaded[0].end_line, None);
}

#[test]
fn load_challenges_returns_none_when_pointer_has_no_source_path() {
    let temp_dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp_dir.path().join("repo")).unwrap();

    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        Arc::new(FileStorage::new()),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-no-path-{}", std::process::id())),
        is_dirty: false,
        root_path: Some(temp_dir.path().join("repo")),
    };
    let challenge = Challenge {
        id: "no-source-path".to_string(),
        source_file_path: None,
        code_content: "fn ghost() {}".to_string(),
        start_line: None,
        end_line: None,
        language: None,
        comment_ranges: Vec::new(),
        difficulty_level: None,
    };

    repository
        .save_challenges(&git_repository, &[challenge])
        .unwrap();

    let loaded = repository.load_challenges_with_progress(&git_repository, None);
    assert!(loaded.is_none());
}

#[test]
fn load_challenges_returns_none_when_start_line_exceeds_file_length() {
    let temp_dir = tempfile::tempdir().unwrap();
    let source_path = temp_dir.path().join("repo/src/lib.rs");
    let source = "fn only() {}\n";
    std::fs::create_dir_all(source_path.parent().unwrap()).unwrap();
    std::fs::write(&source_path, source).unwrap();

    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        file_storage_with_source(source_path.canonicalize().unwrap(), source),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-bad-range-{}", std::process::id())),
        is_dirty: false,
        root_path: Some(temp_dir.path().join("repo")),
    };
    let challenge = Challenge {
        id: "out-of-range".to_string(),
        source_file_path: Some("src/lib.rs".to_string()),
        code_content: "placeholder".to_string(),
        start_line: Some(100),
        end_line: Some(200),
        language: Some("rust".to_string()),
        comment_ranges: Vec::new(),
        difficulty_level: None,
    };

    repository
        .save_challenges(&git_repository, &[challenge])
        .unwrap();

    let loaded = repository.load_challenges_with_progress(&git_repository, None);
    assert!(loaded.is_none());
}

#[test]
fn load_challenges_returns_none_when_start_line_exceeds_end_line() {
    let temp_dir = tempfile::tempdir().unwrap();
    let source_path = temp_dir.path().join("repo/src/lib.rs");
    let source = "line1\nline2\nline3\nline4\nline5\n";
    std::fs::create_dir_all(source_path.parent().unwrap()).unwrap();
    std::fs::write(&source_path, source).unwrap();

    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        file_storage_with_source(source_path.canonicalize().unwrap(), source),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-inverted-{}", std::process::id())),
        is_dirty: false,
        root_path: Some(temp_dir.path().join("repo")),
    };
    let challenge = Challenge {
        id: "inverted-range".to_string(),
        source_file_path: Some("src/lib.rs".to_string()),
        code_content: "placeholder".to_string(),
        start_line: Some(4),
        end_line: Some(2),
        language: Some("rust".to_string()),
        comment_ranges: Vec::new(),
        difficulty_level: None,
    };

    repository
        .save_challenges(&git_repository, &[challenge])
        .unwrap();

    let loaded = repository.load_challenges_with_progress(&git_repository, None);
    assert!(loaded.is_none());
}

#[test]
fn load_challenges_returns_none_when_source_path_escapes_repo_root() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path().join("repo");
    let outside_path = temp_dir.path().join("outside/escape.rs");
    std::fs::create_dir_all(&repo_path).unwrap();
    std::fs::create_dir_all(outside_path.parent().unwrap()).unwrap();
    std::fs::write(&outside_path, "fn outside() {}\n").unwrap();

    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        Arc::new(FileStorage::new()),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-escape-{}", std::process::id())),
        is_dirty: false,
        root_path: Some(repo_path),
    };
    let challenge = Challenge {
        id: "escape-attempt".to_string(),
        source_file_path: Some("../outside/escape.rs".to_string()),
        code_content: "placeholder".to_string(),
        start_line: None,
        end_line: None,
        language: Some("rust".to_string()),
        comment_ranges: Vec::new(),
        difficulty_level: None,
    };

    repository
        .save_challenges(&git_repository, &[challenge])
        .unwrap();

    let loaded = repository.load_challenges_with_progress(&git_repository, None);
    assert!(loaded.is_none());
}

type ProgressCall = (StepType, usize, usize, Option<String>);

#[derive(Default)]
struct RecordingProgressReporter {
    file_counts_calls: Mutex<Vec<ProgressCall>>,
}

impl ProgressReporter for RecordingProgressReporter {
    fn set_step(&self, _step_type: StepType) {}

    fn set_current_file(&self, _file: Option<String>) {}

    fn set_file_counts(
        &self,
        step_type: StepType,
        processed: usize,
        total: usize,
        current_file: Option<String>,
    ) {
        self.file_counts_calls
            .lock()
            .unwrap()
            .push((step_type, processed, total, current_file));
    }
}

#[test]
fn load_challenges_with_progress_reports_progress_to_reporter() {
    let temp_dir = tempfile::tempdir().unwrap();
    let source_path = temp_dir.path().join("repo/src/lib.rs");
    let source = "fn one() {}\nfn two() {}\nfn three() {}\n";
    std::fs::create_dir_all(source_path.parent().unwrap()).unwrap();
    std::fs::write(&source_path, source).unwrap();

    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        file_storage_with_source(source_path.canonicalize().unwrap(), source),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-progress-{}", std::process::id())),
        is_dirty: false,
        root_path: Some(temp_dir.path().join("repo")),
    };
    let challenges = vec![
        Challenge::new("c1".to_string(), "fn one() {}".to_string()).with_source_info(
            "src/lib.rs".to_string(),
            1,
            1,
        ),
        Challenge::new("c2".to_string(), "fn two() {}".to_string()).with_source_info(
            "src/lib.rs".to_string(),
            2,
            2,
        ),
    ];

    repository
        .save_challenges(&git_repository, &challenges)
        .unwrap();

    let reporter = RecordingProgressReporter::default();
    let loaded = repository
        .load_challenges_with_progress(&git_repository, Some(&reporter))
        .expect("saved challenges should reconstruct with progress reporting");

    assert_eq!(loaded.len(), 2);

    let calls = reporter.file_counts_calls.lock().unwrap();
    assert_eq!(calls.len(), 2);
    for (step_type, _processed, total, current_file) in calls.iter() {
        assert_eq!(*step_type, StepType::CacheCheck);
        assert_eq!(*total, 2);
        assert!(current_file
            .as_deref()
            .unwrap_or("")
            .starts_with("Reconstructing challenge"));
    }
    let processed_values: std::collections::HashSet<_> = calls
        .iter()
        .map(|(_, processed, _, _)| *processed)
        .collect();
    assert_eq!(processed_values, std::collections::HashSet::from([1, 2]));
}

#[test]
fn load_challenges_returns_none_when_root_path_is_missing() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repository = ChallengeRepository::new_for_test(
        temp_dir.path().join("cache"),
        Arc::new(FileStorage::new()),
    );
    let git_repository = GitRepository {
        user_name: "test".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/test/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some(format!("load-no-root-{}", std::process::id())),
        is_dirty: false,
        root_path: None,
    };
    let challenge = Challenge::new("c".to_string(), "fn x() {}".to_string()).with_source_info(
        "src/lib.rs".to_string(),
        1,
        1,
    );

    repository
        .save_challenges(&git_repository, &[challenge])
        .unwrap();

    let loaded = repository.load_challenges_with_progress(&git_repository, None);
    assert!(loaded.is_none());
}
