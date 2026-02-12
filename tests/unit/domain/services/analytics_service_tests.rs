use chrono::{DateTime, Utc};
use gittype::domain::models::storage::{
    SessionResultData, SessionStageResult, StoredRepository, StoredSession,
};
use gittype::domain::models::{Challenge, GitRepository, SessionResult};
use gittype::domain::repositories::session_repository::{
    SessionRepository, SessionRepositoryTrait,
};
use gittype::domain::services::analytics_service::{AnalyticsService, AnalyticsServiceInterface};
use gittype::domain::services::scoring::StageTracker;
use gittype::infrastructure::database::daos::{RepositoryDao, RepositoryDaoInterface};
use gittype::infrastructure::database::database::{Database, DatabaseInterface};
use gittype::Result;
use rusqlite::Transaction;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Mock SessionRepositoryTrait returning controlled data
// ---------------------------------------------------------------------------
struct MockSessionRepo {
    sessions: Vec<StoredSession>,
    results: Vec<(i64, SessionResultData)>,
    stage_results: Vec<(i64, Vec<SessionStageResult>)>,
    language_stats: Vec<(String, f64, usize)>,
    repositories: Vec<StoredRepository>,
}

impl MockSessionRepo {
    fn new() -> Self {
        Self {
            sessions: Vec::new(),
            results: Vec::new(),
            stage_results: Vec::new(),
            language_stats: Vec::new(),
            repositories: Vec::new(),
        }
    }
}

impl SessionRepositoryTrait for MockSessionRepo {
    fn record_session(
        &self,
        _session_result: &SessionResult,
        _git_repository: Option<&GitRepository>,
        _game_mode: &str,
        _difficulty_level: Option<&str>,
        _stage_trackers: &[(String, StageTracker)],
        _challenges: &[Challenge],
    ) -> Result<i64> {
        Ok(1)
    }
    fn get_session_stage_results(&self, session_id: i64) -> Result<Vec<SessionStageResult>> {
        Ok(self
            .stage_results
            .iter()
            .find(|(id, _)| *id == session_id)
            .map(|(_, r)| r.clone())
            .unwrap_or_default())
    }
    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        Ok(self.repositories.clone())
    }
    fn get_sessions_filtered(
        &self,
        _repository_filter: Option<i64>,
        _date_filter_days: Option<i64>,
        _sort_by: &str,
        _sort_descending: bool,
    ) -> Result<Vec<StoredSession>> {
        Ok(self.sessions.clone())
    }
    fn get_session_result(&self, session_id: i64) -> Result<Option<SessionResultData>> {
        Ok(self
            .results
            .iter()
            .find(|(id, _)| *id == session_id)
            .map(|(_, r)| r.clone()))
    }
    fn get_language_stats(&self, _days: Option<i64>) -> Result<Vec<(String, f64, usize)>> {
        Ok(self.language_stats.clone())
    }
    fn get_session_result_for_analytics(
        &self,
        session_id: i64,
    ) -> Result<Option<SessionResultData>> {
        self.get_session_result(session_id)
    }
}

// ---------------------------------------------------------------------------
// Mock RepositoryDaoInterface returning controlled data
// ---------------------------------------------------------------------------
struct MockRepoDao {
    repositories: Vec<StoredRepository>,
}

impl MockRepoDao {
    fn new(repos: Vec<StoredRepository>) -> Self {
        Self {
            repositories: repos,
        }
    }
}

impl RepositoryDaoInterface for MockRepoDao {
    fn ensure_repository(&self, _git_repo: &GitRepository) -> Result<i64> {
        Ok(1)
    }
    fn ensure_repository_in_transaction(
        &self,
        _tx: &Transaction,
        _git_repo: &GitRepository,
    ) -> Result<i64> {
        Ok(1)
    }
    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        Ok(self.repositories.clone())
    }
    fn get_repository_by_id(&self, repository_id: i64) -> Result<Option<StoredRepository>> {
        Ok(self
            .repositories
            .iter()
            .find(|r| r.id == repository_id)
            .cloned())
    }
    fn find_repository(
        &self,
        user_name: &str,
        repository_name: &str,
    ) -> Result<Option<StoredRepository>> {
        Ok(self
            .repositories
            .iter()
            .find(|r| r.user_name == user_name && r.repository_name == repository_name)
            .cloned())
    }
    fn get_all_repositories_with_languages(
        &self,
    ) -> Result<Vec<gittype::domain::models::storage::StoredRepositoryWithLanguages>> {
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
fn make_session(id: i64, repo_id: Option<i64>) -> StoredSession {
    StoredSession {
        id,
        repository_id: repo_id,
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
        branch: Some("main".to_string()),
        commit_hash: Some("abc".to_string()),
        is_dirty: false,
        game_mode: "normal".to_string(),
        difficulty_level: None,
        max_stages: None,
        time_limit_seconds: None,
    }
}

fn make_result(cpm: f64, accuracy: f64, duration_ms: u64) -> SessionResultData {
    SessionResultData {
        keystrokes: 100,
        mistakes: 5,
        duration_ms,
        wpm: cpm / 5.0,
        cpm,
        accuracy,
        stages_completed: 3,
        stages_attempted: 3,
        stages_skipped: 0,
        score: cpm * accuracy / 100.0,
        rank_name: None,
        tier_name: None,
        rank_position: None,
        rank_total: None,
        position: None,
        total: None,
    }
}

fn make_stage_result(language: Option<&str>) -> SessionStageResult {
    SessionStageResult {
        stage_number: 1,
        wpm: 50.0,
        cpm: 250.0,
        accuracy: 95.0,
        keystrokes: 100,
        mistakes: 5,
        duration_ms: 30000,
        score: 237.5,
        language: language.map(|s| s.to_string()),
        difficulty_level: None,
        rank_name: None,
        tier_name: None,
        rank_position: 0,
        rank_total: 0,
        position: 0,
        total: 0,
        was_skipped: false,
        was_failed: false,
        file_path: None,
        start_line: None,
        end_line: None,
        code_content: None,
    }
}

fn make_repo(id: i64, user: &str, name: &str) -> StoredRepository {
    StoredRepository {
        id,
        user_name: user.to_string(),
        repository_name: name.to_string(),
        remote_url: format!("https://github.com/{}/{}", user, name),
    }
}

// ---------------------------------------------------------------------------
// Tests using real in-memory DB (kept from original)
// ---------------------------------------------------------------------------
#[test]
fn test_analytics_service_new() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap()) as Arc<dyn DatabaseInterface>;
    let repository_dao =
        Arc::new(RepositoryDao::new(Arc::clone(&db))) as Arc<dyn RepositoryDaoInterface>;
    let _service = AnalyticsService::new(session_repository, repository_dao);
}

#[test]
fn test_load_analytics_data_empty() {
    let session_repository = Arc::new(SessionRepository::new().unwrap());
    let db = Arc::new(Database::new().unwrap()) as Arc<dyn DatabaseInterface>;
    let repository_dao =
        Arc::new(RepositoryDao::new(Arc::clone(&db))) as Arc<dyn RepositoryDaoInterface>;
    let service = AnalyticsService::new(session_repository, repository_dao);

    let data = service.load_analytics_data().unwrap();
    assert_eq!(data.total_sessions, 0);
    assert_eq!(data.avg_cpm, 0.0);
    assert_eq!(data.avg_accuracy, 0.0);
    assert!(data.top_repositories.is_empty());
    assert!(data.top_languages.is_empty());
    assert!(data.cpm_trend.is_empty());
    assert!(data.accuracy_trend.is_empty());
    assert!(data.repository_stats.is_empty());
    assert!(data.language_stats.is_empty());
    assert!(data.reference_date.is_none());
}

// ---------------------------------------------------------------------------
// Tests using mocks — full coverage of computation paths
// ---------------------------------------------------------------------------
#[test]
fn test_analytics_empty_sessions_returns_zeroed_data() {
    let mock_repo = MockSessionRepo::new();
    let mock_dao = MockRepoDao::new(vec![]);
    let service = AnalyticsService::new(Arc::new(mock_repo), Arc::new(mock_dao));

    let data = service.load_analytics_data().unwrap();
    assert_eq!(data.total_sessions, 0);
    assert_eq!(data.best_cpm, 0.0);
    assert_eq!(data.total_mistakes, 0);
    assert_eq!(data.current_streak, 0);
}

#[test]
fn test_analytics_single_session_basic_aggregation() {
    let repo = make_repo(1, "user", "repo");
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, Some(1))];
    mock.results = vec![(1, make_result(300.0, 95.0, 60000))];
    mock.stage_results = vec![(1, vec![make_stage_result(Some("rust"))])];
    mock.language_stats = vec![("rust".to_string(), 300.0, 1)];

    let mock_dao = MockRepoDao::new(vec![repo]);
    let service = AnalyticsService::new(Arc::new(mock), Arc::new(mock_dao));

    let data = service.load_analytics_data().unwrap();
    assert_eq!(data.total_sessions, 1);
    assert!((data.avg_cpm - 300.0).abs() < 0.01);
    assert!((data.avg_accuracy - 95.0).abs() < 0.01);
    assert!((data.best_cpm - 300.0).abs() < 0.01);
    assert!(data.total_time_hours > 0.0);
    assert_eq!(data.daily_sessions.len(), 1);
    assert_eq!(data.cpm_trend.len(), 1);
    assert_eq!(data.accuracy_trend.len(), 1);
}

#[test]
fn test_analytics_repository_stats_fully_computed() {
    let repo = make_repo(1, "owner", "project");
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, Some(1)), make_session(2, Some(1))];
    mock.results = vec![
        (1, make_result(250.0, 90.0, 30000)),
        (2, make_result(350.0, 98.0, 45000)),
    ];
    mock.stage_results = vec![
        (1, vec![make_stage_result(None)]),
        (2, vec![make_stage_result(None)]),
    ];
    mock.repositories = vec![repo.clone()];

    let mock_dao = MockRepoDao::new(vec![repo]);
    let service = AnalyticsService::new(Arc::new(mock), Arc::new(mock_dao));

    let data = service.load_analytics_data().unwrap();

    // top_repositories path: repo_stats populated via repositories_map
    assert!(
        !data.top_repositories.is_empty(),
        "top_repositories should be populated"
    );
    let (ref name, avg) = data.top_repositories[0];
    assert_eq!(name, "owner/project");
    assert!((avg - 300.0).abs() < 0.01);

    // repository_stats path: computed from get_all_repositories → repo_map
    assert!(
        data.repository_stats.contains_key("owner/project"),
        "repository_stats should contain owner/project"
    );
    let stats = &data.repository_stats["owner/project"];
    assert_eq!(stats.total_sessions, 2);
    assert_eq!(stats.total_keystrokes, 200);
    assert_eq!(stats.total_mistakes, 10);
    assert!(stats.avg_cpm > 0.0, "avg_cpm should be computed");
    assert!(stats.avg_wpm > 0.0, "avg_wpm should be computed");
    assert!(stats.avg_accuracy > 0.0, "avg_accuracy should be computed");
    assert!(stats.avg_score > 0.0, "avg_score should be computed");
    assert!((stats.best_cpm - 350.0).abs() < 0.01);
    assert!((stats.best_accuracy - 98.0).abs() < 0.01);
    assert_eq!(stats.stages_completed, 6);
    assert_eq!(stats.stages_attempted, 6);
    assert_eq!(stats.stages_skipped, 0);
}

#[test]
fn test_analytics_language_stats_fully_computed() {
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, None), make_session(2, None)];
    mock.results = vec![
        (1, make_result(200.0, 85.0, 20000)),
        (2, make_result(400.0, 99.0, 50000)),
    ];
    mock.stage_results = vec![
        (
            1,
            vec![
                make_stage_result(Some("rust")),
                make_stage_result(Some("go")),
            ],
        ),
        (2, vec![make_stage_result(Some("rust"))]),
    ];

    let mock_dao = MockRepoDao::new(vec![]);
    let service = AnalyticsService::new(Arc::new(mock), Arc::new(mock_dao));

    let data = service.load_analytics_data().unwrap();

    assert!(
        data.language_stats.contains_key("rust"),
        "should have rust stats"
    );
    let rust_stats = &data.language_stats["rust"];
    assert_eq!(rust_stats.total_sessions, 2);
    assert!(rust_stats.avg_cpm > 0.0);
    assert!(rust_stats.avg_wpm > 0.0);
    assert!(rust_stats.avg_accuracy > 0.0);
    assert!(rust_stats.avg_score > 0.0);
    assert_eq!(rust_stats.stages_completed, 2);
    assert!((rust_stats.best_cpm - 250.0).abs() < 0.01);
    assert!((rust_stats.best_accuracy - 95.0).abs() < 0.01);

    assert!(data.language_stats.contains_key("go"));
    let go_stats = &data.language_stats["go"];
    assert_eq!(go_stats.total_sessions, 1);
}

#[test]
fn test_analytics_multiple_days_trend_sorted() {
    let mut mock = MockSessionRepo::new();
    // Create sessions with different dates
    let mut s1 = make_session(1, None);
    s1.started_at = DateTime::parse_from_rfc3339("2026-01-15T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let mut s2 = make_session(2, None);
    s2.started_at = DateTime::parse_from_rfc3339("2026-01-10T10:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    mock.sessions = vec![s1, s2];
    mock.results = vec![
        (1, make_result(300.0, 90.0, 30000)),
        (2, make_result(200.0, 80.0, 20000)),
    ];
    mock.stage_results = vec![
        (1, vec![make_stage_result(None)]),
        (2, vec![make_stage_result(None)]),
    ];

    let service = AnalyticsService::new(Arc::new(mock), Arc::new(MockRepoDao::new(vec![])));
    let data = service.load_analytics_data().unwrap();

    // Trends should be sorted by date (ascending)
    assert_eq!(data.cpm_trend.len(), 2);
    assert!(
        data.cpm_trend[0].0 < data.cpm_trend[1].0,
        "cpm_trend should be sorted"
    );
    assert_eq!(data.accuracy_trend.len(), 2);
    assert!(
        data.accuracy_trend[0].0 < data.accuracy_trend[1].0,
        "accuracy_trend should be sorted"
    );
}

#[test]
fn test_analytics_best_cpm_tracked_across_sessions() {
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![
        make_session(1, None),
        make_session(2, None),
        make_session(3, None),
    ];
    mock.results = vec![
        (1, make_result(100.0, 90.0, 10000)),
        (2, make_result(500.0, 95.0, 50000)),
        (3, make_result(200.0, 85.0, 20000)),
    ];
    mock.stage_results = vec![(1, vec![]), (2, vec![]), (3, vec![])];

    let service = AnalyticsService::new(Arc::new(mock), Arc::new(MockRepoDao::new(vec![])));
    let data = service.load_analytics_data().unwrap();

    assert!((data.best_cpm - 500.0).abs() < 0.01);
    assert_eq!(data.total_sessions, 3);
}

#[test]
fn test_analytics_mistakes_estimation() {
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, None)];
    // 90% accuracy with 3 stages attempted → (100-90)/100 * 3 = 0.3 → 0 as usize
    mock.results = vec![(1, make_result(200.0, 90.0, 10000))];
    mock.stage_results = vec![(1, vec![])];

    let service = AnalyticsService::new(Arc::new(mock), Arc::new(MockRepoDao::new(vec![])));
    let data = service.load_analytics_data().unwrap();

    // (100.0 - 90.0) / 100.0 * 3 = 0.3 → 0 as usize
    assert_eq!(data.total_mistakes, 0);
}

#[test]
fn test_analytics_session_duration_calculated() {
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, None)];
    // 120000ms = 2 minutes
    mock.results = vec![(1, make_result(300.0, 95.0, 120000))];
    mock.stage_results = vec![(1, vec![])];

    let service = AnalyticsService::new(Arc::new(mock), Arc::new(MockRepoDao::new(vec![])));
    let data = service.load_analytics_data().unwrap();

    // avg_session_duration = 120000 / 1 / 60000 = 2.0 minutes
    assert!((data.avg_session_duration - 2.0).abs() < 0.01);
    // total_time_hours = 120000 / 3600000 ≈ 0.0333
    assert!(data.total_time_hours > 0.0);
}

#[test]
fn test_analytics_session_without_repo_skips_repo_stats() {
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, None)];
    mock.results = vec![(1, make_result(300.0, 95.0, 30000))];
    mock.stage_results = vec![(1, vec![])];

    let service = AnalyticsService::new(Arc::new(mock), Arc::new(MockRepoDao::new(vec![])));
    let data = service.load_analytics_data().unwrap();

    assert!(data.top_repositories.is_empty());
    assert!(data.repository_stats.is_empty());
}

#[test]
fn test_analytics_top_repositories_sorted_by_avg_cpm_descending() {
    let repo_a = make_repo(1, "a", "low");
    let repo_b = make_repo(2, "b", "high");
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, Some(1)), make_session(2, Some(2))];
    mock.results = vec![
        (1, make_result(100.0, 90.0, 10000)),
        (2, make_result(500.0, 95.0, 50000)),
    ];
    mock.stage_results = vec![(1, vec![]), (2, vec![])];

    let mock_dao = MockRepoDao::new(vec![repo_a, repo_b]);
    let service = AnalyticsService::new(Arc::new(mock), Arc::new(mock_dao));
    let data = service.load_analytics_data().unwrap();

    assert_eq!(data.top_repositories.len(), 2);
    // Highest CPM should come first
    assert_eq!(data.top_repositories[0].0, "b/high");
    assert_eq!(data.top_repositories[1].0, "a/low");
}

#[test]
fn test_analytics_stage_results_without_language_ignored() {
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![make_session(1, None)];
    mock.results = vec![(1, make_result(300.0, 95.0, 30000))];
    mock.stage_results = vec![(1, vec![make_stage_result(None)])];

    let service = AnalyticsService::new(Arc::new(mock), Arc::new(MockRepoDao::new(vec![])));
    let data = service.load_analytics_data().unwrap();

    assert!(
        data.language_stats.is_empty(),
        "stages without language should not create lang_stats entries"
    );
}

#[test]
fn test_analytics_multiple_repos_independent_stats() {
    let repo_a = make_repo(1, "owner", "alpha");
    let repo_b = make_repo(2, "owner", "beta");
    let mut mock = MockSessionRepo::new();
    mock.sessions = vec![
        make_session(1, Some(1)),
        make_session(2, Some(2)),
        make_session(3, Some(1)),
    ];
    mock.results = vec![
        (1, make_result(200.0, 90.0, 20000)),
        (2, make_result(400.0, 99.0, 40000)),
        (3, make_result(300.0, 95.0, 30000)),
    ];
    mock.stage_results = vec![
        (1, vec![make_stage_result(None)]),
        (2, vec![make_stage_result(None)]),
        (3, vec![make_stage_result(None)]),
    ];

    let mock_dao = MockRepoDao::new(vec![repo_a, repo_b]);
    let service = AnalyticsService::new(Arc::new(mock), Arc::new(mock_dao));
    let data = service.load_analytics_data().unwrap();

    assert!(data.repository_stats.contains_key("owner/alpha"));
    assert!(data.repository_stats.contains_key("owner/beta"));
    assert_eq!(data.repository_stats["owner/alpha"].total_sessions, 2);
    assert_eq!(data.repository_stats["owner/beta"].total_sessions, 1);
}
