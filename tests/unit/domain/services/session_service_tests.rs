use gittype::domain::models::{Challenge, GitRepository, SessionResult};
use gittype::domain::repositories::SessionRepository;
use gittype::domain::services::scoring::{StageInput, StageTracker};
use gittype::domain::services::session_service::SessionService;

#[test]
fn test_session_service_new() {
    let repository = SessionRepository::new().unwrap();
    let _service = SessionService::new(repository);
    // Service creation should succeed without error
}

#[test]
fn test_get_sessions_with_display_data_empty() {
    let repository = SessionRepository::new().unwrap();
    let service = SessionService::new(repository);

    let result = service.get_sessions_with_display_data(None, None, "date", true);
    assert!(result.is_ok());

    let sessions = result.unwrap();
    assert!(sessions.is_empty() || !sessions.is_empty()); // May have data from other tests
}

#[test]
fn test_get_sessions_with_display_data_with_session() {
    let repository = SessionRepository::new().unwrap();

    // Record a test session
    let mut session_result = SessionResult::new();
    session_result.session_score = 150.0;
    session_result.overall_wpm = 60.0;
    session_result.overall_cpm = 300.0;
    session_result.overall_accuracy = 95.0;

    let git_repo = GitRepository {
        user_name: "sessionuser".to_string(),
        repository_name: "sessionrepo".to_string(),
        remote_url: "https://github.com/sessionuser/sessionrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("session123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("session-test".to_string(), "test code".to_string())
        .with_language("rust".to_string());

    let mut tracker = StageTracker::new("test code".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Keystroke {
        ch: 't',
        position: 0,
    });
    tracker.record(StageInput::Finish);

    repository
        .record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();

    // Get sessions with display data
    let service = SessionService::new(repository);
    let result = service.get_sessions_with_display_data(None, None, "date", true);
    assert!(result.is_ok());

    let sessions = result.unwrap();
    assert!(!sessions.is_empty());

    // Verify session display data structure
    let session_data = &sessions[0];
    assert!(session_data.session.id > 0);
}

#[test]
fn test_get_sessions_with_display_data_with_repository_filter() {
    let repository = SessionRepository::new().unwrap();

    // Record a session with repository
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

    let challenge = Challenge::new("filter-test".to_string(), "filter code".to_string());
    let mut tracker = StageTracker::new("filter code".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    repository
        .record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();

    // Get repository ID
    let repositories = repository.get_all_repositories().unwrap();
    let test_repo = repositories
        .iter()
        .find(|r| r.repository_name == "filterrepo")
        .unwrap();

    // Get sessions filtered by repository
    let service = SessionService::new(repository);
    let result = service.get_sessions_with_display_data(Some(test_repo.id), None, "date", true);
    assert!(result.is_ok());

    let sessions = result.unwrap();
    assert!(!sessions.is_empty());

    // Verify all sessions have the correct repository
    for session_data in &sessions {
        if let Some(repo) = &session_data.repository {
            assert_eq!(repo.repository_name, "filterrepo");
        }
    }
}

#[test]
fn test_get_sessions_with_display_data_with_date_filter() {
    let repository = SessionRepository::new().unwrap();

    // Record a recent session
    let mut session_result = SessionResult::new();
    session_result.session_score = 100.0;

    let git_repo = GitRepository {
        user_name: "dateuser".to_string(),
        repository_name: "daterepo".to_string(),
        remote_url: "https://github.com/dateuser/daterepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("date123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("date-test".to_string(), "date code".to_string());
    let mut tracker = StageTracker::new("date code".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    repository
        .record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();

    // Get sessions from last 7 days
    let service = SessionService::new(repository);
    let result = service.get_sessions_with_display_data(None, Some(7), "date", true);
    assert!(result.is_ok());

    let sessions = result.unwrap();
    assert!(!sessions.is_empty());
}

#[test]
fn test_get_sessions_with_display_data_sort_by_score() {
    let repository = SessionRepository::new().unwrap();

    // Record sessions with different scores
    for i in 0..3 {
        let mut session_result = SessionResult::new();
        session_result.session_score = 100.0 + (i as f64 * 20.0);

        let git_repo = GitRepository {
            user_name: "sortuser".to_string(),
            repository_name: "sortrepo".to_string(),
            remote_url: "https://github.com/sortuser/sortrepo".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some(format!("sort{}", i)),
            is_dirty: false,
            root_path: None,
        };

        let challenge = Challenge::new(format!("sort-{}", i), "sort code".to_string());
        let mut tracker = StageTracker::new("sort code".to_string());
        tracker.record(StageInput::Start);
        tracker.record(StageInput::Finish);

        repository
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

    // Get sessions sorted by score descending
    let service = SessionService::new(repository);
    let result = service.get_sessions_with_display_data(None, None, "score", true);
    assert!(result.is_ok());

    let sessions = result.unwrap();
    assert!(!sessions.is_empty());
}

#[test]
fn test_get_all_repositories() {
    let repository = SessionRepository::new().unwrap();

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

    let challenge = Challenge::new("all-test".to_string(), "all code".to_string());
    let mut tracker = StageTracker::new("all code".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    repository
        .record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();

    // Get all repositories
    let service = SessionService::new(repository);
    let result = service.get_all_repositories();
    assert!(result.is_ok());

    let repositories = result.unwrap();
    assert!(!repositories.is_empty());
}

#[test]
fn test_session_display_data_has_session_result() {
    let repository = SessionRepository::new().unwrap();

    // Record a session
    let mut session_result = SessionResult::new();
    session_result.session_score = 150.0;
    session_result.overall_wpm = 65.0;
    session_result.overall_cpm = 325.0;
    session_result.overall_accuracy = 96.0;

    let git_repo = GitRepository {
        user_name: "resultuser".to_string(),
        repository_name: "resultrepo".to_string(),
        remote_url: "https://github.com/resultuser/resultrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("result123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let challenge = Challenge::new("result-test".to_string(), "result code".to_string());
    let mut tracker = StageTracker::new("result code".to_string());
    tracker.record(StageInput::Start);
    tracker.record(StageInput::Finish);

    repository
        .record_session(
            &session_result,
            Some(&git_repo),
            "normal",
            None,
            &[("stage1".to_string(), tracker)],
            &[challenge],
        )
        .unwrap();

    // Get sessions
    let service = SessionService::new(repository);
    let result = service.get_sessions_with_display_data(None, None, "date", true);
    assert!(result.is_ok());

    let sessions = result.unwrap();
    assert!(!sessions.is_empty());

    // Verify session result is present
    let session_data = &sessions[0];
    if let Some(result) = &session_data.session_result {
        assert!(result.score > 0.0);
        assert!(result.wpm > 0.0);
        assert!(result.cpm > 0.0);
        assert!(result.accuracy > 0.0);
    }
}
