use crate::models::{Rank, StageResult};
use crate::scoring::tracker::StageTracker;

/// Stage level result calculation
pub struct StageCalculator;

impl StageCalculator {
    pub fn calculate(tracker: &StageTracker) -> StageResult {
        let data = tracker.get_data();

        if data.start_time.is_none() {
            return StageResult::default();
        }

        // Calculate metrics from raw data
        let cpm = if data.keystrokes.is_empty() {
            0.1
        } else {
            let correct_chars = data.keystrokes.iter().filter(|k| k.is_correct).count() as f64;
            let elapsed_secs = data.elapsed_time.as_secs_f64().max(0.1);
            (correct_chars / elapsed_secs) * 60.0
        };

        let wpm = cpm / 5.0;

        let accuracy = if data.keystrokes.is_empty() {
            0.0
        } else {
            let correct_chars = data.keystrokes.iter().filter(|k| k.is_correct).count();
            (correct_chars as f64 / data.keystrokes.len() as f64) * 100.0
        };

        let mistakes = data.keystrokes.iter().filter(|k| !k.is_correct).count();
        let total_chars = data.keystrokes.len();

        let mut all_streaks = data.streaks.clone();
        if data.current_streak > 0 {
            all_streaks.push(data.current_streak);
        }

        let challenge_score = crate::scoring::ScoreCalculator::calculate_score_from_metrics(
            cpm,
            accuracy,
            mistakes,
            data.elapsed_time.as_secs_f64(),
            total_chars,
        );
        let rank_name = Rank::for_score(challenge_score).name().to_string();
        let (tier_name, tier_position, tier_total, overall_position, overall_total) =
            crate::scoring::RankCalculator::calculate_tier_info(challenge_score);

        StageResult {
            cpm,
            wpm,
            accuracy,
            keystrokes: data.keystrokes.len(),
            mistakes,
            consistency_streaks: all_streaks,
            completion_time: data.elapsed_time,
            challenge_score,
            rank_name,
            tier_name,
            tier_position,
            tier_total,
            overall_position,
            overall_total,
            was_skipped: data.was_skipped,
            was_failed: data.was_failed,
            challenge_path: data.challenge_path,
        }
    }
}
