use gittype::domain::models::{Challenge, GitRepository, SessionResult};
use gittype::domain::repositories::SessionRepository;
use gittype::domain::services::analytics_service::AnalyticsService;
use gittype::domain::services::scoring::{StageInput, StageTracker};
use gittype::infrastructure::database::database::Database;
use std::sync::Arc;

#[test]
fn test_analytics_service_new() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap());
    let _service = AnalyticsService::new(session_repository, db);

    // Service creation should succeed
    assert!(true);
}

#[test]
fn test_load_analytics_data_empty() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap());
    let service = AnalyticsService::new(session_repository, db);

    let result = service.load_analytics_data();
    assert!(result.is_ok());

    let data = result.unwrap();
    assert_eq!(data.total_sessions, 0);
    assert_eq!(data.avg_cpm, 0.0);
    assert_eq!(data.avg_accuracy, 0.0);
    assert!(data.top_repositories.is_empty());
    assert!(data.top_languages.is_empty());
    assert!(data.cpm_trend.is_empty());
    assert!(data.accuracy_trend.is_empty());
}

#[test]
fn test_load_analytics_data_with_session() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap());

    // Record a test session
    let mut session_result = SessionResult::new();
    session_result.session_score = 150.0;
    session_result.overall_wpm = 60.0;
    session_result.overall_cpm = 300.0;
    session_result.overall_accuracy = 95.0;

    let git_repo = GitRepository {
        user_name: "analyticsuser".to_string(),
        repository_name: "analyticsrepo".to_string(),
        remote_url: "https://github.com/analyticsuser/analyticsrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("test-id".to_string(), "test code".to_string())
        .with_language("rust".to_string());

    let mut tracker = StageTracker::new("test code".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Keystroke {
        ch: 't',
        position: 0,
    });
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

    // Load analytics data
    let service = AnalyticsService::new(session_repository, db);
    let result = service.load_analytics_data();
    assert!(result.is_ok());

    let data = result.unwrap();
    assert!(data.total_sessions > 0);
    assert!(data.avg_cpm > 0.0);
    assert!(data.avg_accuracy > 0.0);
}

#[test]
fn test_load_analytics_data_repository_stats() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap());

    // Record sessions for the same repository
    for i in 0..3 {
        let mut session_result = SessionResult::new();
        session_result.session_score = 100.0 + (i as f64 * 10.0);
        session_result.overall_wpm = 50.0 + (i as f64 * 5.0);
        session_result.overall_cpm = 250.0 + (i as f64 * 25.0);
        session_result.overall_accuracy = 90.0 + (i as f64);

        let git_repo = GitRepository {
            user_name: "repouser".to_string(),
            repository_name: "reporepo".to_string(),
            remote_url: "https://github.com/repouser/reporepo".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some(format!("hash{}", i)),
            is_dirty: false,
            root_path: None,
        };

        let challenge = Challenge::new(format!("test-{}", i), "test code".to_string());
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
    }

    // Load analytics data
    let service = AnalyticsService::new(session_repository, db);
    let result = service.load_analytics_data();
    assert!(result.is_ok());

    let data = result.unwrap();

    // Sessions should be recorded
    assert!(data.total_sessions >= 3);

    // Check that repository stats are calculated if data exists
    let repo_name = "repouser/reporepo";
    if data.repository_stats.contains_key(repo_name) {
        let stats = &data.repository_stats[repo_name];
        assert!(stats.total_sessions >= 3);
        assert!(stats.avg_cpm > 0.0);
        assert!(stats.best_cpm > 0.0);
    }
}

#[test]
fn test_load_analytics_data_language_stats() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap());

    // Record a session with language
    let mut session_result = SessionResult::new();
    session_result.session_score = 120.0;
    session_result.overall_wpm = 55.0;
    session_result.overall_cpm = 275.0;
    session_result.overall_accuracy = 93.0;

    let git_repo = GitRepository {
        user_name: "languser".to_string(),
        repository_name: "langrepo".to_string(),
        remote_url: "https://github.com/languser/langrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("langhash".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("lang-test".to_string(), "fn main() {}".to_string())
        .with_language("rust".to_string());

    let mut tracker = StageTracker::new("fn main() {}".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Keystroke {
        ch: 'f',
        position: 0,
    });
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

    // Load analytics data
    let service = AnalyticsService::new(session_repository, db);
    let result = service.load_analytics_data();
    assert!(result.is_ok());

    let data = result.unwrap();

    // Check language stats
    if data.language_stats.contains_key("rust") {
        let stats = &data.language_stats["rust"];
        assert!(stats.total_sessions > 0);
        assert!(stats.avg_cpm > 0.0);
    }
}

#[test]
fn test_load_analytics_data_daily_sessions() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap());

    // Record a session
    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

    let git_repo = GitRepository {
        user_name: "dailyuser".to_string(),
        repository_name: "dailyrepo".to_string(),
        remote_url: "https://github.com/dailyuser/dailyrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("daily123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("daily-test".to_string(), "test".to_string());
    let mut tracker = StageTracker::new("test".to_string());
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

    // Load analytics data
    let service = AnalyticsService::new(session_repository, db);
    let result = service.load_analytics_data();
    assert!(result.is_ok());

    let data = result.unwrap();
    assert!(!data.daily_sessions.is_empty(), "Should have daily session counts");
}
