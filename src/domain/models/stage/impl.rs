use std::time::Duration;

use crate::domain::models::Challenge;

#[derive(Debug, Clone)]
pub struct Stage {
    pub challenge: Challenge,
    pub stage_number: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StageResult {
    pub cpm: f64,
    pub wpm: f64,
    pub accuracy: f64,
    pub keystrokes: usize,
    pub mistakes: usize,
    pub consistency_streaks: Vec<usize>,
    pub completion_time: Duration,
    pub challenge_score: f64,
    pub rank_name: String,
    pub tier_name: String,
    pub tier_position: usize,
    pub tier_total: usize,
    pub overall_position: usize,
    pub overall_total: usize,
    pub was_skipped: bool,
    pub was_failed: bool,
    pub challenge_path: String,
}

impl Default for StageResult {
    fn default() -> Self {
        Self {
            cpm: 0.0,
            wpm: 0.0,
            accuracy: 0.0,
            keystrokes: 0,
            mistakes: 0,
            consistency_streaks: vec![],
            completion_time: Duration::new(0, 0),
            challenge_score: 0.0,
            rank_name: "Unranked".to_string(),
            tier_name: "Beginner".to_string(),
            tier_position: 0,
            tier_total: 0,
            overall_position: 0,
            overall_total: 0,
            was_skipped: false,
            was_failed: false,
            challenge_path: String::new(),
        }
    }
}

impl Stage {
    pub fn new(challenge: Challenge, stage_number: usize) -> Self {
        Self {
            challenge,
            stage_number,
        }
    }
}
