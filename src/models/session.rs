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
    pub session_duration: Duration,
    pub stages_completed: usize,
    pub stages_attempted: usize,
    pub stages_skipped: usize,
    pub stage_results: Vec<StageResult>,
    pub overall_accuracy: f64,
    pub overall_wpm: f64,
    pub overall_cpm: f64,
    pub total_keystrokes: usize,
    pub total_mistakes: usize,
    pub session_score: f64,
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
            stages_completed: 0,
            stages_attempted: 0,
            stages_skipped: 0,
            stage_results: Vec::new(),
            overall_accuracy: 0.0,
            overall_wpm: 0.0,
            overall_cpm: 0.0,
            total_keystrokes: 0,
            total_mistakes: 0,
            session_score: 0.0,
        }
    }
}

impl Default for SessionResult {
    fn default() -> Self {
        Self::new()
    }
}