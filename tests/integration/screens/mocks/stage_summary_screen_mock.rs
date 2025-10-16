use gittype::domain::models::StageResult;
use gittype::presentation::tui::screens::stage_summary_screen::StageSummaryData;
use gittype::presentation::tui::ScreenDataProvider;
use gittype::Result;
use std::time::Duration;

pub struct MockStageSummaryDataProvider;

impl ScreenDataProvider for MockStageSummaryDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let stage_result = StageResult {
            challenge_score: 850.0,
            cpm: 280.0,
            wpm: 56.0,
            accuracy: 95.5,
            completion_time: Duration::from_secs_f64(12.5),
            mistakes: 3,
            keystrokes: 58,
            consistency_streaks: vec![5, 3, 4],
            rank_name: "Compiler".to_string(),
            tier_name: "Master".to_string(),
            tier_position: 5,
            tier_total: 10,
            overall_position: 25,
            overall_total: 100,
            was_failed: false,
            was_skipped: false,
            challenge_path: "test/path".to_string(),
        };

        Ok(Box::new(StageSummaryData {
            stage_result,
            current_stage: 2,
            total_stages: 3,
            is_completed: false,
        }))
    }
}
