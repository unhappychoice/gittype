use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TypingMetrics {
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
    pub corrections: usize,
    pub consistency_score: f64,
    pub completion_time: Duration,
    pub challenge_score: f64,
}