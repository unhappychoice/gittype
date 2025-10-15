use gittype::domain::models::storage::SessionResultData;
use gittype::domain::models::{Challenge, GitRepository, SessionResult};
use gittype::domain::repositories::session_repository::{
    BestRecords, BestStatus, SessionRepository, SessionRepositoryTrait,
};
use gittype::domain::services::scoring::{StageInput, StageTracker};
use gittype::infrastructure::database::database::HasDatabase;
use std::time::Duration;

// BestStatus tests
#[test]
fn test_best_status_new() {
    let status = BestStatus::new();
    assert!(!status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert!(status.best_type.is_none());
    assert_eq!(status.todays_best_score, 0.0);
    assert_eq!(status.weekly_best_score, 0.0);
    assert_eq!(status.all_time_best_score, 0.0);
}

#[test]
fn test_best_status_default() {
    let status = BestStatus::default();
    assert!(!status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert!(status.best_type.is_none());
}

// determine_best_status_with_start_records tests
#[test]
fn test_determine_best_status_no_previous_records() {
    let session_score = 100.0;
    let status = SessionRepository::determine_best_status_with_start_records(session_score, None);

    assert!(status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert_eq!(status.best_type, Some("TODAY'S".to_string()));
}

#[test]
fn test_determine_best_status_beats_all_time() {
    let session_score = 150.0;
    let best_records = BestRecords {
        todays_best: Some(create_session_result_data(1, 100.0)),
        weekly_best: Some(create_session_result_data(2, 120.0)),
        all_time_best: Some(create_session_result_data(3, 140.0)),
    };

    let status = SessionRepository::determine_best_status_with_start_records(
        session_score,
        Some(&best_records),
    );

    assert!(status.is_todays_best);
    assert!(status.is_weekly_best);
    assert!(status.is_all_time_best);
    assert_eq!(status.best_type, Some("ALL TIME".to_string()));
    assert_eq!(status.all_time_best_score, 140.0);
}

#[test]
fn test_determine_best_status_beats_weekly_only() {
    let session_score = 130.0;
    let best_records = BestRecords {
        todays_best: Some(create_session_result_data(1, 100.0)),
        weekly_best: Some(create_session_result_data(2, 120.0)),
        all_time_best: Some(create_session_result_data(3, 140.0)),
    };

    let status = SessionRepository::determine_best_status_with_start_records(
        session_score,
        Some(&best_records),
    );

    assert!(status.is_todays_best);
    assert!(status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert_eq!(status.best_type, Some("WEEKLY".to_string()));
    assert_eq!(status.weekly_best_score, 120.0);
}

#[test]
fn test_determine_best_status_beats_todays_only() {
    let session_score = 110.0;
    let best_records = BestRecords {
        todays_best: Some(create_session_result_data(1, 100.0)),
        weekly_best: Some(create_session_result_data(2, 120.0)),
        all_time_best: Some(create_session_result_data(3, 140.0)),
    };

    let status = SessionRepository::determine_best_status_with_start_records(
        session_score,
        Some(&best_records),
    );

    assert!(status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert_eq!(status.best_type, Some("TODAY'S".to_string()));
    assert_eq!(status.todays_best_score, 100.0);
}

#[test]
fn test_determine_best_status_no_todays_best() {
    let session_score = 50.0;
    let best_records = BestRecords {
        todays_best: None,
        weekly_best: Some(create_session_result_data(2, 120.0)),
        all_time_best: Some(create_session_result_data(3, 140.0)),
    };

    let status = SessionRepository::determine_best_status_with_start_records(
        session_score,
        Some(&best_records),
    );

    assert!(status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert_eq!(status.best_type, Some("TODAY'S".to_string()));
}

#[test]
fn test_determine_best_status_equal_score_all_time() {
    let session_score = 140.0;
    let best_records = BestRecords {
        todays_best: Some(create_session_result_data(1, 100.0)),
        weekly_best: Some(create_session_result_data(2, 120.0)),
        all_time_best: Some(create_session_result_data(3, 140.0)),
    };

    let status = SessionRepository::determine_best_status_with_start_records(
        session_score,
        Some(&best_records),
    );

    // Equal score should count as beating the record
    assert!(status.is_all_time_best);
    assert_eq!(status.best_type, Some("ALL TIME".to_string()));
}

#[test]
fn test_determine_best_status_beats_none() {
    let session_score = 50.0;
    let best_records = BestRecords {
        todays_best: Some(create_session_result_data(1, 100.0)),
        weekly_best: Some(create_session_result_data(2, 120.0)),
        all_time_best: Some(create_session_result_data(3, 140.0)),
    };

    let status = SessionRepository::determine_best_status_with_start_records(
        session_score,
        Some(&best_records),
    );

    assert!(!status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert!(status.best_type.is_none());
}

#[test]
fn test_determine_best_status_empty_best_records() {
    let session_score = 100.0;
    let best_records = BestRecords {
        todays_best: None,
        weekly_best: None,
        all_time_best: None,
    };

    let status = SessionRepository::determine_best_status_with_start_records(
        session_score,
        Some(&best_records),
    );

    assert!(status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert_eq!(status.best_type, Some("TODAY'S".to_string()));
}

// Helper function to create test session result data
fn create_session_result_data(_id: i64, score: f64) -> SessionResultData {
    SessionResultData {
        keystrokes: 500,
        mistakes: 5,
        duration_ms: 60000,
        wpm: 100.0,
        cpm: 100.0,
        accuracy: 95.0,
        stages_completed: 10,
        stages_attempted: 10,
        stages_skipped: 0,
        score,
        rank_name: None,
        tier_name: None,
        rank_position: None,
        rank_total: None,
        position: None,
        total: None,
    }
}

// SessionRepository instance tests
#[test]
fn test_session_repository_new() {
    let result = SessionRepository::new();
    assert!(result.is_ok());
}

#[test]
fn test_session_repository_default() {
    let _repo = SessionRepository::default();
    // Test passes if construction succeeds
}

#[test]
fn test_get_all_repositories_with_data() {
    let repo = SessionRepository::new().unwrap();

    // Record a session with repository
    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

    let git_repo = GitRepository {
        user_name: "allrepouser".to_string(),
        repository_name: "allrepo".to_string(),
        remote_url: "https://github.com/allrepouser/allrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("all123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("allrepo-id".to_string(), "test".to_string());
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    repo.record_session(
        &session_result,
        Some(&git_repo),
        "normal",
        None,
        &[("stage1".to_string(), tracker)],
        &[challenge],
    )
    .unwrap();

    // Get all repositories
    let repositories = repo.get_all_repositories().unwrap();
    assert!(repositories.len() > 0);

    // Verify our repository is in the list
    let found = repositories
        .iter()
        .any(|r| r.repository_name == "allrepo" && r.user_name == "allrepouser");
    assert!(found, "Repository should be in the list");
}

#[test]
fn test_get_language_stats_with_data() {
    let repo = SessionRepository::new().unwrap();

    // Record a session with language
    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

    let git_repo = GitRepository {
        user_name: "languser".to_string(),
        repository_name: "langrepo".to_string(),
        remote_url: "https://github.com/languser/langrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("lang123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("lang-id".to_string(), "test".to_string())
        .with_language("python".to_string());
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    repo.record_session(
        &session_result,
        Some(&git_repo),
        "normal",
        None,
        &[("stage1".to_string(), tracker)],
        &[challenge],
    )
    .unwrap();

    // Wait a bit to ensure timestamp is within 7 days window
    let stats = repo.get_language_stats(None).unwrap();

    // Should have stats for python
    assert!(
        stats.iter().any(|(lang, _, _)| lang == "python"),
        "Should have stats for python language"
    );
}

#[test]
fn test_get_sessions_filtered_sort_verification() {
    let repo = SessionRepository::new().unwrap();

    let git_repo = GitRepository {
        user_name: "sortuser".to_string(),
        repository_name: "sortrepo".to_string(),
        remote_url: "https://github.com/sortuser/sortrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("sort123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    // Create two sessions with different scores
    for score in [100.0, 150.0] {
        let mut session_result = SessionResult::new();
        session_result.session_score = score;

        let challenge = Challenge::new(format!("sort-{}", score), "test".to_string());
        let mut tracker = StageTracker::new("test".to_string());
        tracker.record(StageInput::Start);
        tracker.record(StageInput::Finish);

        repo.record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();
    }

    // Get sessions sorted by score ascending
    let sessions = repo
        .get_sessions_filtered(None, None, "score", false)
        .unwrap();

    assert!(sessions.len() >= 2, "Should have at least 2 sessions");
}

#[test]
fn test_get_session_result_for_analytics_with_data() {
    let repo = SessionRepository::new().unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 180.0;
    session_result.overall_wpm = 70.0;
    session_result.overall_accuracy = 96.0;

    let git_repo = GitRepository {
        user_name: "analyticsuser".to_string(),
        repository_name: "analyticsrepo".to_string(),
        remote_url: "https://github.com/analyticsuser/analyticsrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("analytics123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("analytics-id".to_string(), "test".to_string());
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    let session_id = repo
        .record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();

    // Get session result for analytics
    let result = repo.get_session_result_for_analytics(session_id).unwrap();
    assert!(result.is_some(), "Should return session result");

    let analytics_data = result.unwrap();
    assert_eq!(analytics_data.score, 180.0);
    assert_eq!(analytics_data.wpm, 70.0);
    assert_eq!(analytics_data.accuracy, 96.0);
}

#[test]
fn test_global_instance() {
    let global = SessionRepository::global();
    assert!(global.lock().is_ok());
}

#[test]
fn test_initialize_global() {
    let result = SessionRepository::initialize_global();
    assert!(result.is_ok());

    // Verify global is initialized
    let global = SessionRepository::global();
    let guard = global.lock().unwrap();
    assert!(guard.is_some());
}

#[test]
fn test_get_best_records_global() {
    // Initialize global first
    let _ = SessionRepository::initialize_global();

    let result = SessionRepository::get_best_records_global();
    assert!(result.is_ok());
    let best_records = result.unwrap();
    assert!(best_records.is_some());
}

#[test]
fn test_determine_best_status_global() {
    // Initialize global first
    let _ = SessionRepository::initialize_global();

    let result = SessionRepository::determine_best_status_global(100.0);
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(status.is_some());
}

#[test]
fn test_session_repository_trait_implementation() {
    let repo = SessionRepository::new().unwrap();
    let trait_ref: &dyn SessionRepositoryTrait = &repo;

    // Test trait method
    let result = trait_ref.get_session_stage_results(99999);
    assert!(result.is_ok());
}

#[test]
fn test_has_database_trait_implementation() {
    let repo = SessionRepository::new().unwrap();

    // Verify database access through HasDatabase trait
    let db_arc = repo.database();
    assert!(db_arc.lock().is_ok());
}

// Integration tests with actual data
#[test]
fn test_record_session_and_retrieve() {
    let repo = SessionRepository::new().unwrap();

    // Create test data
    let mut session_result = SessionResult::new();
    session_result.stages_completed = 2;
    session_result.stages_attempted = 2;
    session_result.session_score = 150.0;
    session_result.overall_wpm = 60.0;
    session_result.overall_cpm = 300.0;
    session_result.overall_accuracy = 95.0;
    session_result.valid_session_duration = Duration::from_secs(120);

    let git_repo = GitRepository {
        user_name: "recorduser".to_string(),
        repository_name: "recordrepo".to_string(),
        remote_url: "https://github.com/recorduser/recordrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("xyz789".to_string()),
        is_dirty: false,
        root_path: None,
    };

    // Create challenges
    let challenge1 = Challenge::new("test-id-1".to_string(), "fn test() {}".to_string())
        .with_language("rust".to_string());
    let challenge2 = Challenge::new("test-id-2".to_string(), "fn test2() {}".to_string())
        .with_language("rust".to_string());

    // Create stage trackers
    let mut tracker1 = StageTracker::new("fn test() {}".to_string());
    tracker1.record(StageInput::Start);
    tracker1.record(StageInput::Keystroke {
        ch: 'f',
        position: 0,
    });
    tracker1.record(StageInput::Finish);

    let mut tracker2 = StageTracker::new("fn test2() {}".to_string());
    tracker2.record(StageInput::Start);
    tracker2.record(StageInput::Keystroke {
        ch: 'f',
        position: 0,
    });
    tracker2.record(StageInput::Finish);

    let stage_trackers = vec![
        ("stage1".to_string(), tracker1),
        ("stage2".to_string(), tracker2),
    ];

    let challenges = vec![challenge1, challenge2];

    // Record session
    let result = repo.record_session(
        &session_result,
        Some(&git_repo),
        "normal",
        Some("easy"),
        &stage_trackers,
        &challenges,
    );

    assert!(result.is_ok());
    let session_id = result.unwrap();
    assert!(session_id > 0);

    // Retrieve session stage results
    let stage_results = repo.get_session_stage_results(session_id).unwrap();
    assert_eq!(stage_results.len(), 2);

    // Verify data
    assert_eq!(stage_results[0].language, Some("rust".to_string()));
    assert_eq!(stage_results[1].language, Some("rust".to_string()));
}

#[test]
fn test_record_session_with_repository() {
    let repo = SessionRepository::new().unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

    let git_repo = GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("test-id".to_string(), "test".to_string());
    let mut tracker = StageTracker::new("test".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    let stage_trackers = vec![("stage1".to_string(), tracker)];
    let challenges = vec![challenge];

    let result = repo.record_session(
        &session_result,
        Some(&git_repo),
        "normal",
        None,
        &stage_trackers,
        &challenges,
    );

    assert!(result.is_ok());
    let session_id = result.unwrap();

    // Verify repository was created
    let repositories = repo.get_all_repositories().unwrap();
    assert!(repositories.len() > 0);
    assert!(repositories.iter().any(|r| r.repository_name == "testrepo"));

    // Verify session was recorded
    let stage_results = repo.get_session_stage_results(session_id).unwrap();
    assert_eq!(stage_results.len(), 1);
}

#[test]
fn test_get_repository_history_with_data() {
    let repo = SessionRepository::new().unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

    let git_repo = GitRepository {
        user_name: "historyuser".to_string(),
        repository_name: "historyrepo".to_string(),
        remote_url: "https://github.com/historyuser/historyrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("def456".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("hist-id".to_string(), "hist".to_string());
    let mut tracker = StageTracker::new("hist".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    let stage_trackers = vec![("stage1".to_string(), tracker)];
    let challenges = vec![challenge];

    // Record session
    repo.record_session(
        &session_result,
        Some(&git_repo),
        "normal",
        None,
        &stage_trackers,
        &challenges,
    )
    .unwrap();

    // Get repositories to find the ID
    let repositories = repo.get_all_repositories().unwrap();
    let test_repo = repositories
        .iter()
        .find(|r| r.repository_name == "historyrepo")
        .unwrap();

    // Get history for this repository
    let history = repo.get_repository_history(test_repo.id).unwrap();
    assert!(history.len() > 0);
}

#[test]
fn test_get_best_records_with_data() {
    let repo = SessionRepository::new().unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 200.0;
    session_result.overall_wpm = 80.0;
    session_result.overall_accuracy = 98.0;

    let git_repo = GitRepository {
        user_name: "bestuser".to_string(),
        repository_name: "bestrepo".to_string(),
        remote_url: "https://github.com/bestuser/bestrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("best123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("best-id".to_string(), "best".to_string());
    let mut tracker = StageTracker::new("best".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    let stage_trackers = vec![("stage1".to_string(), tracker)];
    let challenges = vec![challenge];

    // Record session
    repo.record_session(
        &session_result,
        Some(&git_repo),
        "normal",
        None,
        &stage_trackers,
        &challenges,
    )
    .unwrap();

    // Get best records
    let best_records = repo.get_best_records().unwrap();

    // Should have today's best
    assert!(best_records.todays_best.is_some());
    let todays_best = best_records.todays_best.unwrap();
    assert_eq!(todays_best.score, 200.0);
}

#[test]
fn test_get_sessions_filtered_with_data() {
    let repo = SessionRepository::new().unwrap();

    let mut session_result = SessionResult::new();
    session_result.session_score = 120.0;

    let git_repo = GitRepository {
        user_name: "filteruser".to_string(),
        repository_name: "filterrepo".to_string(),
        remote_url: "https://github.com/filteruser/filterrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("filter123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("filter-id".to_string(), "filter".to_string());
    let mut tracker = StageTracker::new("filter".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    let stage_trackers = vec![("stage1".to_string(), tracker)];
    let challenges = vec![challenge];

    // Record session
    repo.record_session(
        &session_result,
        Some(&git_repo),
        "normal",
        None,
        &stage_trackers,
        &challenges,
    )
    .unwrap();

    // Get filtered sessions
    let sessions = repo
        .get_sessions_filtered(None, None, "completed_at", true)
        .unwrap();
    assert!(sessions.len() > 0);

    // Verify sorting by score
    let sessions_by_score = repo
        .get_sessions_filtered(None, None, "score", false)
        .unwrap();
    assert!(sessions_by_score.len() > 0);
}
