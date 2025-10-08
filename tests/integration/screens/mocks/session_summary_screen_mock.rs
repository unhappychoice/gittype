use gittype::domain::models::{GitRepository, SessionResult};
use gittype::presentation::game::screens::session_summary_screen::SessionSummaryScreenData;
use gittype::presentation::game::{SessionManager, ScreenDataProvider};
use gittype::Result;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct MockSessionSummaryDataProvider;

impl ScreenDataProvider for MockSessionSummaryDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = Some(SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_secs(180),
            valid_session_duration: Duration::from_secs(150),
            invalid_session_duration: Duration::from_secs(30),
            stages_completed: 5,
            stages_attempted: 5,
            stages_skipped: 0,
            stage_results: vec![],
            overall_accuracy: 94.5,
            overall_wpm: 52.0,
            overall_cpm: 260.0,
            valid_keystrokes: 780,
            valid_mistakes: 30,
            invalid_keystrokes: 15,
            invalid_mistakes: 15,
            best_stage_wpm: 60.0,
            worst_stage_wpm: 45.0,
            best_stage_accuracy: 98.0,
            worst_stage_accuracy: 90.0,
            session_score: 9500.0,
            session_successful: true,
        });

        let git_repository = Some(GitRepository {
            user_name: "unhappychoice".to_string(),
            repository_name: "gittype".to_string(),
            remote_url: "https://github.com/unhappychoice/gittype.git".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some("def456".to_string()),
            is_dirty: false,
            root_path: None,
        });

        let data = SessionSummaryScreenData {
            session_result,
            git_repository,
            session_manager: Arc::new(Mutex::new(SessionManager::new())),
        };
        Ok(Box::new(data))
    }
}
