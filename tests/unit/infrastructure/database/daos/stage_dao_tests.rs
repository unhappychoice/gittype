use gittype::domain::models::{Challenge, DifficultyLevel, GitRepository, SessionResult};
use gittype::infrastructure::database::daos::{ChallengeDao, RepositoryDao, SessionDao, StageDao};
use gittype::infrastructure::database::database::Database;

#[test]
fn test_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    let _dao = StageDao::new(&db);
}

fn setup_test_data(
    db: &Database,
) -> (
    i64,
    GitRepository,
    Vec<(i64, Challenge)>, // (session_id, challenge)
) {
    let repo_dao = RepositoryDao::new(db);
    let session_dao = SessionDao::new(db);
    let challenge_dao = ChallengeDao::new(db);

    let git_repo = GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("test123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    // Create challenges with different languages and difficulties
    let challenges = vec![
        Challenge::new("stage-1".to_string(), "fn test1() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(DifficultyLevel::Easy),
        Challenge::new("stage-2".to_string(), "function test2() {}".to_string())
            .with_language("javascript".to_string())
            .with_difficulty_level(DifficultyLevel::Normal),
        Challenge::new("stage-3".to_string(), "def test3(): pass".to_string())
            .with_language("python".to_string())
            .with_difficulty_level(DifficultyLevel::Hard),
        Challenge::new("stage-4".to_string(), "fn test4() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(DifficultyLevel::Easy),
    ];

    let conn = db.get_connection();
    let tx = conn.unchecked_transaction().unwrap();
    for challenge in &challenges {
        challenge_dao
            .ensure_challenge_in_transaction(&tx, challenge)
            .unwrap();
    }
    tx.commit().unwrap();

    // Create sessions and stage results
    let mut session_challenges = Vec::new();
    for (i, challenge) in challenges.iter().enumerate() {
        let mut session_result = SessionResult::new();
        session_result.session_score = 100.0 + (i as f64 * 10.0);

        let conn = db.get_connection();
        let tx = conn.unchecked_transaction().unwrap();
        let session_id = session_dao
            .create_session_in_transaction(
                &tx,
                Some(repository_id),
                &session_result,
                Some(&git_repo),
                "normal",
                Some("easy"),
            )
            .unwrap();

        session_dao
            .save_session_result_in_transaction(
                &tx,
                session_id,
                Some(repository_id),
                &session_result,
                &[],
                "normal",
                Some("easy"),
            )
            .unwrap();

        // Insert stage_results directly with RFC3339 timestamp
        let completed_at = chrono::Utc::now().to_rfc3339();

        tx.execute(
            "INSERT INTO stages (session_id, challenge_id, stage_number, started_at, completed_at)
             VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![
                session_id,
                challenge.id.as_str(),
                1i64,
                &completed_at,
                &completed_at
            ],
        )
        .unwrap();
        let stage_id = tx.last_insert_rowid();

        tx.execute(
            "INSERT INTO stage_results (
                stage_id, session_id, repository_id, keystrokes, mistakes, duration_ms,
                wpm, cpm, accuracy, consistency_streaks, score, rank_name, tier_name,
                rank_position, rank_total, position, total,
                was_skipped, was_failed, completed_at, language, difficulty_level
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                stage_id,
                session_id,
                repository_id,
                50 + (i as i64 * 10),
                0i64,
                1000i64,
                50.0,
                250.0,
                100.0,
                "[]",
                50.0,
                "Beginner",
                "Bronze",
                1i64,
                100i64,
                1i64,
                500i64,
                false,
                false,
                &completed_at,
                challenge.language.as_deref(),
                challenge
                    .difficulty_level
                    .as_ref()
                    .map(|d| format!("{:?}", d))
            ],
        )
        .unwrap();

        tx.commit().unwrap();

        session_challenges.push((session_id, challenge.clone()));
    }

    (repository_id, git_repo, session_challenges)
}

#[test]
fn test_get_completed_stages() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    let (repository_id, _, _) = setup_test_data(&db);

    // Get all completed stages
    let stages = stage_dao.get_completed_stages(None).unwrap();
    assert!(
        stages.len() >= 4,
        "Should return at least 4 completed stages"
    );

    // Get stages for specific repository
    let repo_stages = stage_dao.get_completed_stages(Some(repository_id)).unwrap();
    assert_eq!(
        repo_stages.len(),
        4,
        "Should return 4 stages for repository"
    );

    for stage in &repo_stages {
        assert_eq!(stage.repository_id, Some(repository_id));
    }
}

#[test]
fn test_get_completed_stages_by_language() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    let (repository_id, _, _) = setup_test_data(&db);

    // Get rust stages
    let rust_stages = stage_dao
        .get_completed_stages_by_language("rust", Some(repository_id))
        .unwrap();
    assert_eq!(rust_stages.len(), 2, "Should return 2 rust stages");

    for stage in &rust_stages {
        assert_eq!(stage.language, Some("rust".to_string()));
        assert_eq!(stage.repository_id, Some(repository_id));
    }

    // Get javascript stages
    let js_stages = stage_dao
        .get_completed_stages_by_language("javascript", Some(repository_id))
        .unwrap();
    assert_eq!(js_stages.len(), 1, "Should return 1 javascript stage");
    assert_eq!(js_stages[0].language, Some("javascript".to_string()));

    // Get python stages
    let py_stages = stage_dao
        .get_completed_stages_by_language("python", Some(repository_id))
        .unwrap();
    assert_eq!(py_stages.len(), 1, "Should return 1 python stage");
    assert_eq!(py_stages[0].language, Some("python".to_string()));

    // Get stages for non-existent language
    let no_stages = stage_dao
        .get_completed_stages_by_language("nonexistent", Some(repository_id))
        .unwrap();
    assert_eq!(
        no_stages.len(),
        0,
        "Should return 0 stages for non-existent language"
    );
}

#[test]
fn test_get_completed_stages_by_language_without_repository() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    setup_test_data(&db);

    // Get rust stages across all repositories
    let rust_stages = stage_dao
        .get_completed_stages_by_language("rust", None)
        .unwrap();
    assert!(
        rust_stages.len() >= 2,
        "Should return at least 2 rust stages"
    );

    for stage in &rust_stages {
        assert_eq!(stage.language, Some("rust".to_string()));
    }
}

#[test]
fn test_get_completed_stages_by_difficulty() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    let (repository_id, _, _) = setup_test_data(&db);

    // Get easy stages
    let easy_stages = stage_dao
        .get_completed_stages_by_difficulty("Easy", Some(repository_id))
        .unwrap();
    assert_eq!(easy_stages.len(), 2, "Should return 2 easy stages");

    for stage in &easy_stages {
        assert_eq!(stage.difficulty_level, Some("Easy".to_string()));
        assert_eq!(stage.repository_id, Some(repository_id));
    }

    // Get normal stages
    let normal_stages = stage_dao
        .get_completed_stages_by_difficulty("Normal", Some(repository_id))
        .unwrap();
    assert_eq!(normal_stages.len(), 1, "Should return 1 normal stage");
    assert_eq!(
        normal_stages[0].difficulty_level,
        Some("Normal".to_string())
    );

    // Get hard stages
    let hard_stages = stage_dao
        .get_completed_stages_by_difficulty("Hard", Some(repository_id))
        .unwrap();
    assert_eq!(hard_stages.len(), 1, "Should return 1 hard stage");
    assert_eq!(hard_stages[0].difficulty_level, Some("Hard".to_string()));
}

#[test]
fn test_get_completed_stages_by_difficulty_without_repository() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    setup_test_data(&db);

    // Get easy stages across all repositories
    let easy_stages = stage_dao
        .get_completed_stages_by_difficulty("Easy", None)
        .unwrap();
    assert!(
        easy_stages.len() >= 2,
        "Should return at least 2 easy stages"
    );

    for stage in &easy_stages {
        assert_eq!(stage.difficulty_level, Some("Easy".to_string()));
    }
}

#[test]
fn test_get_stage_statistics() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    let (repository_id, _, _) = setup_test_data(&db);

    // Get statistics for specific repository
    let stats = stage_dao.get_stage_statistics(Some(repository_id)).unwrap();

    assert_eq!(stats.total_completed, 4, "Should have 4 completed stages");
    assert!(stats.avg_wpm >= 0.0, "Average WPM should be non-negative");
    assert!(
        stats.avg_accuracy >= 0.0,
        "Average accuracy should be non-negative"
    );
    assert!(
        stats.total_keystrokes >= 0,
        "Total keystrokes should be non-negative"
    );
    assert!(
        stats.total_mistakes >= 0,
        "Total mistakes should be non-negative"
    );

    // Get statistics for all repositories
    let all_stats = stage_dao.get_stage_statistics(None).unwrap();
    assert!(
        all_stats.total_completed >= 4,
        "Should have at least 4 completed stages"
    );
}

#[test]
fn test_get_language_breakdown() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    let (repository_id, _, _) = setup_test_data(&db);

    // Get language breakdown for specific repository
    let breakdown = stage_dao
        .get_language_breakdown(Some(repository_id))
        .unwrap();

    assert_eq!(breakdown.len(), 3, "Should have 3 languages");

    // Find rust stats (should be first due to ORDER BY stage_count DESC)
    let rust_stats = breakdown
        .iter()
        .find(|s| s.language == "rust")
        .expect("Should have rust stats");
    assert_eq!(rust_stats.stage_count, 2, "Should have 2 rust stages");
    assert!(
        rust_stats.avg_wpm >= 0.0,
        "Average WPM should be non-negative"
    );
    assert!(
        rust_stats.avg_accuracy >= 0.0,
        "Average accuracy should be non-negative"
    );

    // Find javascript stats
    let js_stats = breakdown
        .iter()
        .find(|s| s.language == "javascript")
        .expect("Should have javascript stats");
    assert_eq!(js_stats.stage_count, 1, "Should have 1 javascript stage");

    // Find python stats
    let py_stats = breakdown
        .iter()
        .find(|s| s.language == "python")
        .expect("Should have python stats");
    assert_eq!(py_stats.stage_count, 1, "Should have 1 python stage");
}

#[test]
fn test_get_language_breakdown_without_repository() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    setup_test_data(&db);

    // Get language breakdown for all repositories
    let breakdown = stage_dao.get_language_breakdown(None).unwrap();

    assert!(breakdown.len() >= 3, "Should have at least 3 languages");

    // Verify rust is in the breakdown
    let rust_stats = breakdown.iter().find(|s| s.language == "rust");
    assert!(rust_stats.is_some(), "Should have rust stats");
    assert!(
        rust_stats.unwrap().stage_count >= 2,
        "Should have at least 2 rust stages"
    );
}

#[test]
fn test_get_difficulty_breakdown() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    let (repository_id, _, _) = setup_test_data(&db);

    // Get difficulty breakdown for specific repository
    let breakdown = stage_dao
        .get_difficulty_breakdown(Some(repository_id))
        .unwrap();

    assert_eq!(breakdown.len(), 3, "Should have 3 difficulty levels");

    // Verify ordering (Easy, Normal, Hard, Wild, Zen)
    assert_eq!(breakdown[0].difficulty_level, "Easy");
    assert_eq!(breakdown[1].difficulty_level, "Normal");
    assert_eq!(breakdown[2].difficulty_level, "Hard");

    // Verify easy stats
    assert_eq!(breakdown[0].stage_count, 2, "Should have 2 easy stages");
    assert!(
        breakdown[0].avg_wpm >= 0.0,
        "Average WPM should be non-negative"
    );
    assert!(
        breakdown[0].avg_accuracy >= 0.0,
        "Average accuracy should be non-negative"
    );

    // Verify normal stats
    assert_eq!(breakdown[1].stage_count, 1, "Should have 1 normal stage");

    // Verify hard stats
    assert_eq!(breakdown[2].stage_count, 1, "Should have 1 hard stage");
}

#[test]
fn test_get_difficulty_breakdown_without_repository() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    setup_test_data(&db);

    // Get difficulty breakdown for all repositories
    let breakdown = stage_dao.get_difficulty_breakdown(None).unwrap();

    assert!(
        breakdown.len() >= 3,
        "Should have at least 3 difficulty levels"
    );

    // Verify easy difficulty is in the breakdown
    let easy_stats = breakdown.iter().find(|s| s.difficulty_level == "Easy");
    assert!(easy_stats.is_some(), "Should have easy difficulty stats");
    assert!(
        easy_stats.unwrap().stage_count >= 2,
        "Should have at least 2 easy stages"
    );
}

#[test]
fn test_empty_stage_statistics() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    // Create a repository with no stages
    let git_repo = GitRepository {
        user_name: "emptyuser".to_string(),
        repository_name: "emptyrepo".to_string(),
        remote_url: "https://github.com/emptyuser/emptyrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("empty123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let empty_repo_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let stats = stage_dao.get_stage_statistics(Some(empty_repo_id)).unwrap();

    assert_eq!(stats.total_completed, 0, "Should have 0 completed stages");
    assert_eq!(stats.avg_wpm, 0.0, "Average WPM should be 0.0");
    assert_eq!(stats.avg_accuracy, 0.0, "Average accuracy should be 0.0");
    assert_eq!(stats.total_keystrokes, 0, "Total keystrokes should be 0");
    assert_eq!(stats.total_mistakes, 0, "Total mistakes should be 0");
}

#[test]
fn test_empty_language_breakdown() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    // Create a repository with no stages
    let git_repo = GitRepository {
        user_name: "emptylanguser".to_string(),
        repository_name: "emptylangrepo".to_string(),
        remote_url: "https://github.com/emptylanguser/emptylangrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("emptylang123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let empty_repo_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let breakdown = stage_dao
        .get_language_breakdown(Some(empty_repo_id))
        .unwrap();

    assert_eq!(
        breakdown.len(),
        0,
        "Should have 0 languages for empty repository"
    );
}

#[test]
fn test_empty_difficulty_breakdown() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    // Create a repository with no stages
    let git_repo = GitRepository {
        user_name: "emptydiffuser".to_string(),
        repository_name: "emptydiffrepo".to_string(),
        remote_url: "https://github.com/emptydiffuser/emptydiffrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("emptydiff123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let empty_repo_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let breakdown = stage_dao
        .get_difficulty_breakdown(Some(empty_repo_id))
        .unwrap();

    assert_eq!(
        breakdown.len(),
        0,
        "Should have 0 difficulty levels for empty repository"
    );
}

#[test]
fn test_completed_stages_ordered_by_date() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    let (repository_id, _, _) = setup_test_data(&db);

    let stages = stage_dao.get_completed_stages(Some(repository_id)).unwrap();

    // Stages should be ordered by completed_at DESC (most recent first)
    assert!(stages.len() >= 2, "Should have at least 2 stages");

    for i in 0..stages.len() - 1 {
        assert!(
            stages[i].completed_at >= stages[i + 1].completed_at,
            "Stages should be ordered by completed_at DESC"
        );
    }
}

#[test]
fn test_multiple_repositories_isolation() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let stage_dao = StageDao::new(&db);

    // Setup first repository
    let (repo_id1, _, _) = setup_test_data(&db);

    // Setup second repository
    let repo_dao = RepositoryDao::new(&db);
    let git_repo2 = GitRepository {
        user_name: "user2".to_string(),
        repository_name: "repo2".to_string(),
        remote_url: "https://github.com/user2/repo2".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("test456".to_string()),
        is_dirty: false,
        root_path: None,
    };
    let repo_id2 = repo_dao.ensure_repository(&git_repo2).unwrap();

    // Get stages for each repository
    let stages1 = stage_dao.get_completed_stages(Some(repo_id1)).unwrap();
    let stages2 = stage_dao.get_completed_stages(Some(repo_id2)).unwrap();

    assert_eq!(stages1.len(), 4, "Repo 1 should have 4 stages");
    assert_eq!(stages2.len(), 0, "Repo 2 should have 0 stages");

    // Verify all stages belong to correct repository
    for stage in &stages1 {
        assert_eq!(stage.repository_id, Some(repo_id1));
    }
}
