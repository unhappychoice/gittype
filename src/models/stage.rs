use super::challenge::Challenge;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Stage {
    pub challenge: Challenge,
    pub stage_number: usize,
}

#[derive(Debug, Clone)]
pub struct StageResult {
    pub cpm: f64,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
    pub consistency_streaks: Vec<usize>,
    pub completion_time: Duration,
    pub challenge_score: f64,
    pub ranking_title: String,
    pub ranking_tier: String,
    pub tier_position: usize,
    pub tier_total: usize,
    pub overall_position: usize,
    pub overall_total: usize,
    pub was_skipped: bool,
    pub was_failed: bool,
}

impl Stage {
    pub fn new(challenge: Challenge, stage_number: usize) -> Self {
        Self {
            challenge,
            stage_number,
        }
    }
}