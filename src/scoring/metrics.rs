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
}