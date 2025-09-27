use crate::domain::models::{Rank, RankTier};

/// Calculator for rank and tier position information
pub struct RankCalculator;

impl RankCalculator {
    /// Calculate tier and rank position information for a given score
    /// Returns: (tier_name, tier_position, tier_total, overall_position, overall_total)
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

        let tier_position = same_tier_ranks.len()
            - same_tier_ranks
                .iter()
                .position(|rank| rank.name() == current_rank.name())
                .unwrap_or(0);

        let tier_total = same_tier_ranks.len();

        let overall_position = all_ranks.len()
            - all_ranks
                .iter()
                .position(|rank| rank.name() == current_rank.name())
                .unwrap_or(0);

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
