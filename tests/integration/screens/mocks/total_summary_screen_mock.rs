use gittype::domain::models::TotalResult;
use gittype::presentation::tui::screens::total_summary_screen::TotalSummaryScreenData;
use gittype::presentation::game::ScreenDataProvider;
use gittype::Result;
use std::time::{Duration, Instant};

pub struct MockTotalSummaryDataProvider;

impl ScreenDataProvider for MockTotalSummaryDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let total_result = TotalResult {
            start_time: Instant::now(),
            total_duration: Duration::from_secs(900), // 15 minutes
            total_sessions_completed: 3,
            total_sessions_attempted: 3,
            session_results: vec![],
            total_stages_completed: 15,
            total_stages_attempted: 15,
            total_stages_skipped: 0,
            overall_accuracy: 95.5,
            overall_wpm: 55.0,
            overall_cpm: 275.0,
            total_keystrokes: 4125,
            total_mistakes: 185,
            best_session_wpm: 60.0,
            worst_session_wpm: 50.0,
            best_session_accuracy: 98.0,
            worst_session_accuracy: 93.0,
            total_score: 9800.0,
        };

        let data = TotalSummaryScreenData { total_result };
        Ok(Box::new(data))
    }
}
