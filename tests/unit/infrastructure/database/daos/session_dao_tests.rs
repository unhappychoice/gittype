use gittype::domain::models::{Challenge, DifficultyLevel, GitRepository, SessionResult};
use gittype::infrastructure::database::daos::{ChallengeDao, RepositoryDao, SessionDao};
use gittype::infrastructure::database::database::Database;
use std::time::Duration;

#[test]
fn test_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    let _dao = SessionDao::new(&db);
}

#[test]
fn test_create_session_in_transaction() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;
    session_result.overall_wpm = 50.0;

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
    tx.commit().unwrap();

    assert!(session_id > 0, "Should return positive session ID");
}

#[test]
fn test_save_session_result_in_transaction() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "resultuser".to_string(),
        repository_name: "resultrepo".to_string(),
        remote_url: "https://github.com/resultuser/resultrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("xyz789".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 150.0;
    session_result.overall_wpm = 60.0;
    session_result.overall_cpm = 300.0;
    session_result.overall_accuracy = 95.5;
    session_result.valid_keystrokes = 200;
    session_result.valid_mistakes = 10;
    session_result.invalid_keystrokes = 5;
    session_result.invalid_mistakes = 2;
    session_result.stages_completed = 3;
    session_result.stages_attempted = 4;
    session_result.stages_skipped = 1;
    session_result.session_duration = Duration::from_secs(120);
    session_result.best_stage_wpm = 70.0;
    session_result.worst_stage_wpm = 50.0;
    session_result.best_stage_accuracy = 98.0;
    session_result.worst_stage_accuracy = 92.0;

    let conn = db.get_connection();
    let tx = conn.unchecked_transaction().unwrap();

    // Create session first
    let session_id = session_dao
        .create_session_in_transaction(
            &tx,
            Some(repository_id),
            &session_result,
            Some(&git_repo),
            "normal",
            Some("medium"),
        )
        .unwrap();

    // Save session result
    session_dao
        .save_session_result_in_transaction(
            &tx,
            session_id,
            Some(repository_id),
            &session_result,
            &[],
            "normal",
            Some("medium"),
        )
        .unwrap();

    tx.commit().unwrap();

    // Verify the session result was saved
    let result = session_dao.get_session_result(session_id).unwrap();
    assert!(result.is_some(), "Session result should exist");

    let result_data = result.unwrap();
    assert_eq!(result_data.keystrokes, 200);
    assert_eq!(result_data.mistakes, 10);
    assert_eq!(result_data.wpm, 60.0);
    assert_eq!(result_data.accuracy, 95.5);
    assert_eq!(result_data.score, 150.0);
}

#[test]
fn test_save_stage_result_in_transaction() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);
    let challenge_dao = ChallengeDao::new(&db);

    let git_repo = GitRepository {
        user_name: "stageuser".to_string(),
        repository_name: "stagerepo".to_string(),
        remote_url: "https://github.com/stageuser/stagerepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("stage123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    // Create a challenge
    let challenge = Challenge::new("stage-test-1".to_string(), "fn test() {}".to_string())
        .with_language("rust".to_string())
        .with_difficulty_level(DifficultyLevel::Easy);

    let conn = db.get_connection();
    let tx = conn.unchecked_transaction().unwrap();
    challenge_dao
        .ensure_challenge_in_transaction(&tx, &challenge)
        .unwrap();
    tx.commit().unwrap();

    // Create session
    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

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
    tx.commit().unwrap();

    // Insert stage_results directly with RFC3339 timestamp
    let conn = db.get_connection();
    let tx = conn.unchecked_transaction().unwrap();

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
            50i64,
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

    // Verify stage result was saved
    let stage_results = session_dao.get_session_stage_results(session_id).unwrap();
    assert_eq!(stage_results.len(), 1, "Should have 1 stage result");
    assert_eq!(stage_results[0].stage_number, 1); // 1-based index
}

#[test]
fn test_get_repository_sessions() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "historyuser".to_string(),
        repository_name: "historyrepo".to_string(),
        remote_url: "https://github.com/historyuser/historyrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("hist123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    // Create multiple sessions
    for i in 0..3 {
        let mut session_result = SessionResult::new();
        session_result.session_score = 100.0 + (i as f64) * 10.0;

        let conn = db.get_connection();
        let tx = conn.unchecked_transaction().unwrap();
        session_dao
            .create_session_in_transaction(
                &tx,
                Some(repository_id),
                &session_result,
                Some(&git_repo),
                "normal",
                Some("easy"),
            )
            .unwrap();
        tx.commit().unwrap();
    }

    let sessions = session_dao.get_repository_sessions(repository_id).unwrap();
    assert_eq!(sessions.len(), 3, "Should return 3 sessions");

    // Sessions should be ordered by started_at DESC (most recent first)
    for session in &sessions {
        assert_eq!(session.repository_id, Some(repository_id));
    }
}

#[test]
fn test_get_todays_best_session() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "todayuser".to_string(),
        repository_name: "todayrepo".to_string(),
        remote_url: "https://github.com/todayuser/todayrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("today123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    // Create sessions with different scores
    let scores = vec![100.0, 200.0, 150.0];
    for score in scores {
        let mut session_result = SessionResult::new();
        session_result.session_score = score;

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

        tx.commit().unwrap();
    }

    let best = session_dao.get_todays_best_session().unwrap();
    assert!(best.is_some(), "Should find today's best session");

    let best_session = best.unwrap();
    let best_result = session_dao
        .get_session_result(best_session.id)
        .unwrap()
        .unwrap();
    assert_eq!(
        best_result.score, 200.0,
        "Should return session with highest score"
    );
}

#[test]
fn test_get_weekly_best_session() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "weekuser".to_string(),
        repository_name: "weekrepo".to_string(),
        remote_url: "https://github.com/weekuser/weekrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("week123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 180.0;

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

    tx.commit().unwrap();

    let weekly_best = session_dao.get_weekly_best_session().unwrap();
    assert!(weekly_best.is_some(), "Should find weekly best session");
}

#[test]
fn test_get_all_time_best_session() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "alltimeuser".to_string(),
        repository_name: "alltimerepo".to_string(),
        remote_url: "https://github.com/alltimeuser/alltimerepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("alltime123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    // Create sessions with different scores
    let scores = vec![120.0, 250.0, 180.0];
    for score in scores {
        let mut session_result = SessionResult::new();
        session_result.session_score = score;

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

        tx.commit().unwrap();
    }

    let all_time_best = session_dao.get_all_time_best_session().unwrap();
    assert!(all_time_best.is_some(), "Should find all-time best session");

    let best_session = all_time_best.unwrap();
    let best_result = session_dao
        .get_session_result(best_session.id)
        .unwrap()
        .unwrap();
    assert_eq!(
        best_result.score, 250.0,
        "Should return session with highest score"
    );
}

#[test]
fn test_get_session_result() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "getuser".to_string(),
        repository_name: "getrepo".to_string(),
        remote_url: "https://github.com/getuser/getrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("get123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 135.0;
    session_result.overall_wpm = 55.5;
    session_result.overall_accuracy = 94.2;

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

    tx.commit().unwrap();

    let result = session_dao.get_session_result(session_id).unwrap();
    assert!(result.is_some(), "Should find session result");

    let result_data = result.unwrap();
    assert_eq!(result_data.score, 135.0);
    assert_eq!(result_data.wpm, 55.5);
    assert_eq!(result_data.accuracy, 94.2);
}

#[test]
fn test_get_session_result_not_found() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);

    let result = session_dao.get_session_result(99999).unwrap();
    assert!(
        result.is_none(),
        "Should return None for non-existent session"
    );
}

#[test]
fn test_get_sessions_filtered_by_repository() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    // Create two repositories
    let git_repo1 = GitRepository {
        user_name: "filteruser1".to_string(),
        repository_name: "filterrepo1".to_string(),
        remote_url: "https://github.com/filteruser1/filterrepo1".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("filter1".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let git_repo2 = GitRepository {
        user_name: "filteruser2".to_string(),
        repository_name: "filterrepo2".to_string(),
        remote_url: "https://github.com/filteruser2/filterrepo2".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("filter2".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repo_id1 = repo_dao.ensure_repository(&git_repo1).unwrap();
    let repo_id2 = repo_dao.ensure_repository(&git_repo2).unwrap();

    // Create sessions for both repositories
    for repo_id in &[repo_id1, repo_id2] {
        let git_repo = if *repo_id == repo_id1 {
            &git_repo1
        } else {
            &git_repo2
        };

        let mut session_result = SessionResult::new();
        session_result.session_score = 100.0;

        let conn = db.get_connection();
        let tx = conn.unchecked_transaction().unwrap();
        let session_id = session_dao
            .create_session_in_transaction(
                &tx,
                Some(*repo_id),
                &session_result,
                Some(git_repo),
                "normal",
                Some("easy"),
            )
            .unwrap();

        session_dao
            .save_session_result_in_transaction(
                &tx,
                session_id,
                Some(*repo_id),
                &session_result,
                &[],
                "normal",
                Some("easy"),
            )
            .unwrap();

        tx.commit().unwrap();
    }

    // Filter by repository
    let sessions = session_dao
        .get_sessions_filtered(Some(repo_id1), None, "date", true)
        .unwrap();

    assert!(
        !sessions.is_empty(),
        "Should return sessions for the filtered repository"
    );
    for session in &sessions {
        assert_eq!(session.repository_id, Some(repo_id1));
    }
}

#[test]
fn test_get_sessions_filtered_by_date() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "dateuser".to_string(),
        repository_name: "daterepo".to_string(),
        remote_url: "https://github.com/dateuser/daterepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("date123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

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

    tx.commit().unwrap();

    // Filter by last 7 days
    let sessions = session_dao
        .get_sessions_filtered(None, Some(7), "date", true)
        .unwrap();

    assert!(
        !sessions.is_empty(),
        "Should return sessions from the last 7 days"
    );
}

#[test]
fn test_get_sessions_filtered_sorted_by_score() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);

    let git_repo = GitRepository {
        user_name: "sortuser".to_string(),
        repository_name: "sortrepo".to_string(),
        remote_url: "https://github.com/sortuser/sortrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("sort123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    // Create sessions with different scores
    let scores = vec![150.0, 100.0, 200.0];
    for score in scores {
        let mut session_result = SessionResult::new();
        session_result.session_score = score;

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

        tx.commit().unwrap();
    }

    // Sort by score descending
    let sessions = session_dao
        .get_sessions_filtered(None, None, "score", true)
        .unwrap();

    assert!(sessions.len() >= 3, "Should return at least 3 sessions");

    // Verify scores are sorted in descending order
    let mut prev_score = f64::INFINITY;
    for session in sessions {
        let result = session_dao.get_session_result(session.id).unwrap().unwrap();
        assert!(
            result.score <= prev_score,
            "Scores should be in descending order"
        );
        prev_score = result.score;
    }
}

#[test]
fn test_get_session_stage_results() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let session_dao = SessionDao::new(&db);
    let repo_dao = RepositoryDao::new(&db);
    let challenge_dao = ChallengeDao::new(&db);

    let git_repo = GitRepository {
        user_name: "stageresultuser".to_string(),
        repository_name: "stageresultrepo".to_string(),
        remote_url: "https://github.com/stageresultuser/stageresultrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("stageresult123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let repository_id = repo_dao.ensure_repository(&git_repo).unwrap();

    // Create challenges
    let challenges = vec![
        Challenge::new("stage-result-1".to_string(), "fn test1() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(DifficultyLevel::Easy),
        Challenge::new("stage-result-2".to_string(), "fn test2() {}".to_string())
            .with_language("rust".to_string())
            .with_difficulty_level(DifficultyLevel::Normal),
    ];

    let conn = db.get_connection();
    let tx = conn.unchecked_transaction().unwrap();
    for challenge in &challenges {
        challenge_dao
            .ensure_challenge_in_transaction(&tx, challenge)
            .unwrap();
    }
    tx.commit().unwrap();

    // Create session
    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

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
    tx.commit().unwrap();

    // Create stage results with RFC3339 timestamps
    for (i, challenge) in challenges.iter().enumerate() {
        let conn = db.get_connection();
        let tx = conn.unchecked_transaction().unwrap();

        let completed_at = chrono::Utc::now().to_rfc3339();

        tx.execute(
            "INSERT INTO stages (session_id, challenge_id, stage_number, started_at, completed_at)
             VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![
                session_id,
                challenge.id.as_str(),
                (i + 1) as i64,
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
                (50 + (i as i64 * 10)),
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
    }

    // Get stage results
    let stage_results = session_dao.get_session_stage_results(session_id).unwrap();
    assert_eq!(stage_results.len(), 2, "Should have 2 stage results");
    assert_eq!(stage_results[0].stage_number, 1);
    assert_eq!(stage_results[1].stage_number, 2);
    assert_eq!(stage_results[0].language, Some("rust".to_string()));
    assert_eq!(stage_results[1].language, Some("rust".to_string()));
}
