use gittype::domain::models::SessionResult;
use gittype::presentation::tui::ScreenDataProvider;
use gittype::presentation::tui::screens::session_failure_screen::SessionFailureScreenData;
use gittype::Result;
use std::time::{Duration, Instant};

pub struct MockSessionFailureDataProvider;

impl ScreenDataProvider for MockSessionFailureDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        // Create mock session result with fixed values for consistent snapshots
        let session_result = SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_secs(60),
            valid_session_duration: Duration::from_secs(40),
            invalid_session_duration: Duration::from_secs(20),
            stages_completed: 1,
            stages_attempted: 3,
            stages_skipped: 0,
            stage_results: vec![],
            overall_accuracy: 85.7,
            overall_wpm: 6.0,
            overall_cpm: 30.0,
            valid_keystrokes: 30,
            valid_mistakes: 0,
            invalid_keystrokes: 5,
            invalid_mistakes: 5,
            best_stage_wpm: 6.0,
            worst_stage_wpm: 6.0,
            best_stage_accuracy: 85.7,
            worst_stage_accuracy: 85.7,
            session_score: 150.0,
            session_successful: false,
        };

        let data = SessionFailureScreenData {
            session_result,
            total_stages: 3,
            repo_info: None,
        };

        Ok(Box::new(data))
    }
}
