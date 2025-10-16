use gittype::domain::models::{GitRepository, SessionResult};
use gittype::presentation::tui::screens::session_summary_share_screen::SessionSummaryShareData;
use gittype::presentation::tui::ScreenDataProvider;
use gittype::Result;
use std::time::{Duration, Instant};

pub struct MockSessionSummaryShareDataProvider;

impl ScreenDataProvider for MockSessionSummaryShareDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_secs(120),
            valid_session_duration: Duration::from_secs(100),
            invalid_session_duration: Duration::from_secs(20),
            stages_completed: 3,
            stages_attempted: 3,
            stages_skipped: 0,
            stage_results: vec![],
            overall_accuracy: 92.5,
            overall_wpm: 45.0,
            overall_cpm: 225.0,
            valid_keystrokes: 450,
            valid_mistakes: 15,
            invalid_keystrokes: 20,
            invalid_mistakes: 20,
            best_stage_wpm: 50.0,
            worst_stage_wpm: 40.0,
            best_stage_accuracy: 95.0,
            worst_stage_accuracy: 90.0,
            session_score: 8500.0,
            session_successful: true,
        };

        let git_repository = Some(GitRepository {
            user_name: "unhappychoice".to_string(),
            repository_name: "gittype".to_string(),
            remote_url: "https://github.com/unhappychoice/gittype.git".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some("abc123".to_string()),
            is_dirty: false,
            root_path: None,
        });

        let data = SessionSummaryShareData {
            session_result,
            git_repository,
        };
        Ok(Box::new(data))
    }
}
