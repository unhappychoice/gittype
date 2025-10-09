use gittype::domain::models::TotalResult;
use gittype::presentation::game::screens::total_summary_share_screen::TotalSummaryShareData;
use gittype::presentation::game::ScreenDataProvider;
use gittype::Result;
use std::time::Duration;

pub struct MockTotalSummaryShareDataProvider;

impl ScreenDataProvider for MockTotalSummaryShareDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let mut total_result = TotalResult::new();
        total_result.total_score = 5000.0;
        total_result.overall_cpm = 320.0;
        total_result.total_keystrokes = 15000;
        total_result.total_sessions_completed = 25;
        total_result.total_sessions_attempted = 30;
        total_result.total_duration = Duration::from_secs(3600);

        Ok(Box::new(TotalSummaryShareData { total_result }))
    }
}
