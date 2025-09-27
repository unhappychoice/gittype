/// Core scoring calculation logic shared across different levels
pub struct ScoreCalculator;

impl ScoreCalculator {
    /// Calculate score from performance metrics
    pub fn calculate_score_from_metrics(
        cpm: f64,
        accuracy: f64,
        mistakes: usize,
        elapsed_secs: f64,
        total_chars: usize,
    ) -> f64 {
        let base_score = cpm * (accuracy / 100.0) * 10.0;

        let a = accuracy.clamp(0.0, 100.0) / 100.0;
        let consistency_factor = if a <= 0.7 {
            0.0
        } else if a < 0.9 {
            let t = (a - 0.7) / 0.2;
            let s = t * t * (3.0 - 2.0 * t);
            0.5 * s
        } else if a < 0.95 {
            0.5
        } else {
            let t = (a - 0.95) / 0.05;
            let s = t * t * (3.0 - 2.0 * t);
            0.5 + 0.2 * s
        };
        let consistency_bonus = base_score * consistency_factor;

        let time_bonus = if total_chars > 50 {
            let ideal_time = total_chars as f64 / 10.0;
            if elapsed_secs < ideal_time {
                (ideal_time - elapsed_secs) * 20.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        let mistake_penalty = mistakes as f64 * 5.0;
        let raw_score = base_score + consistency_bonus + time_bonus - mistake_penalty;
        (raw_score * 2.0 + 100.0).max(0.0)
    }
}
