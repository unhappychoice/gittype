use super::stage::{Stage, StageResult};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Session {
    pub stages: Vec<Stage>,
    pub session_start_time: Instant,
}

#[derive(Debug, Clone)]
pub struct SessionResult {
    pub session_start_time: Instant,
    pub session_duration: Duration, // Total duration (valid + invalid) for backward compatibility
    pub valid_session_duration: Duration, // Duration of completed stages only
    pub invalid_session_duration: Duration, // Duration of skipped/failed stages
    pub stages_completed: usize,
    pub stages_attempted: usize,
    pub stages_skipped: usize,
    pub stage_results: Vec<StageResult>,
    pub overall_accuracy: f64,
    pub overall_wpm: f64,
    pub overall_cpm: f64,
    pub valid_keystrokes: usize,
    pub valid_mistakes: usize,
    pub invalid_keystrokes: usize,
    pub invalid_mistakes: usize,
    pub best_stage_wpm: f64,
    pub worst_stage_wpm: f64,
    pub best_stage_accuracy: f64,
    pub worst_stage_accuracy: f64,
    pub session_score: f64,
    pub session_successful: bool, // True if session was completed successfully
}

impl Session {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self {
            stages,
            session_start_time: Instant::now(),
        }
    }
}

impl SessionResult {
    pub fn new() -> Self {
        Self {
            session_start_time: Instant::now(),
            session_duration: Duration::default(),
            valid_session_duration: Duration::default(),
            invalid_session_duration: Duration::default(),
            stages_completed: 0,
            stages_attempted: 0,
            stages_skipped: 0,
            stage_results: Vec::new(),
            overall_accuracy: 0.0,
            overall_wpm: 0.0,
            overall_cpm: 0.0,
            valid_keystrokes: 0,
            valid_mistakes: 0,
            invalid_keystrokes: 0,
            invalid_mistakes: 0,
            best_stage_wpm: 0.0,
            worst_stage_wpm: f64::MAX,
            best_stage_accuracy: 0.0,
            worst_stage_accuracy: f64::MAX,
            session_score: 0.0,
            session_successful: false,
        }
    }


    pub fn get_session_completion_status(&self) -> String {
        match (self.stages_completed, self.stages_skipped) {
            (0, 0) => "No challenges attempted".to_string(),
            (completed, 0) if completed > 0 => {
                format!("Perfect session! {} challenges completed", completed)
            }
            (completed, skips) => format!("{} completed, {} skipped", completed, skips),
        }
    }
}

impl Default for SessionResult {
    fn default() -> Self {
        Self::new()
    }
}
