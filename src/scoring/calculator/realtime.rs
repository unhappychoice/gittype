use std::time::Duration;

/// Real-time metric calculation
pub struct RealTimeCalculator;

impl RealTimeCalculator {
    pub fn calculate(
        current_position: usize,
        mistakes: usize,
        elapsed_time: Duration,
    ) -> RealTimeResult {
        let elapsed_secs = elapsed_time.as_secs_f64().max(0.1);
        let cpm = (current_position as f64 / elapsed_secs) * 60.0;
        let wpm = cpm / 5.0;
        let accuracy = if current_position > 0 {
            ((current_position.saturating_sub(mistakes)) as f64 / current_position as f64) * 100.0
        } else {
            0.0
        };

        RealTimeResult {
            wpm,
            cpm,
            accuracy,
            mistakes,
        }
    }
}

pub struct RealTimeResult {
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
}
