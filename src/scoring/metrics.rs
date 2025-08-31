use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TypingMetrics {
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
