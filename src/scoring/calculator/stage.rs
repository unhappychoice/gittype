use crate::models::{Rank, RankTier, StageResult};
use crate::scoring::tracker::StageTracker;

/// Stage level result calculation
pub struct StageCalculator;

impl StageCalculator {
    pub fn calculate(tracker: &StageTracker, was_skipped: bool, was_failed: bool) -> StageResult {
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
            Self::calculate_tier_info(challenge_score);

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
            was_skipped,
            was_failed,
            challenge_path: data.challenge_path,
        }
    }

    pub fn calculate_tier_info(score: f64) -> (String, usize, usize, usize, usize) {
        let all_ranks = Rank::all_ranks();
        let current_rank = Rank::for_score(score);

        let same_tier_ranks: Vec<_> = all_ranks
            .iter()
            .filter(|rank| rank.tier() == current_rank.tier())
            .collect();

        let tier_name = match current_rank.tier() {
            RankTier::Beginner => "Beginner",
            RankTier::Intermediate => "Intermediate",
            RankTier::Advanced => "Advanced",
            RankTier::Expert => "Expert",
            RankTier::Legendary => "Legendary",
        }
        .to_string();

        let tier_position = same_tier_ranks
            .iter()
            .rev()
            .position(|rank| rank.name() == current_rank.name())
            .map(|pos| pos + 1)
            .unwrap_or(1);

        let tier_total = same_tier_ranks.len();

        let overall_position = all_ranks
            .iter()
            .rev()
            .position(|rank| rank.name() == current_rank.name())
            .map(|pos| pos + 1)
            .unwrap_or(1);

        let overall_total = all_ranks.len();

        (
            tier_name,
            tier_position,
            tier_total,
            overall_position,
            overall_total,
        )
    }
}
