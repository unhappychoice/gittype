use gittype::domain::models::SessionResult;
use gittype::presentation::tui::screens::animation_screen::AnimationData;
use gittype::presentation::tui::ScreenDataProvider;
use gittype::Result;
use std::time::{Duration, Instant};

pub struct MockAnimationDataProvider;

impl ScreenDataProvider for MockAnimationDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = SessionResult {
            session_start_time: Instant::now(),
            session_duration: Duration::from_secs(60),
            valid_session_duration: Duration::from_secs(55),
            invalid_session_duration: Duration::from_secs(5),
            stages_completed: 3,
            stages_attempted: 3,
            stages_skipped: 0,
            stage_results: vec![],
            overall_accuracy: 96.0,
            overall_wpm: 75.0,
            overall_cpm: 375.0,
            valid_keystrokes: 500,
            valid_mistakes: 20,
            invalid_keystrokes: 0,
            invalid_mistakes: 0,
            best_stage_wpm: 80.0,
            worst_stage_wpm: 70.0,
            best_stage_accuracy: 98.0,
            worst_stage_accuracy: 94.0,
            session_score: 1200.0,
            session_successful: true,
        };

        Ok(Box::new(AnimationData { session_result }))
    }
}
