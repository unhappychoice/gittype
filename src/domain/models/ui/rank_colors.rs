use crate::domain::models::RankTier;

/// Get the color palette for a given rank tier
pub fn get_tier_colors(tier: &RankTier) -> &'static [u8] {
    match tier {
        RankTier::Beginner => &BEGINNER_COLORS,
        RankTier::Intermediate => &INTERMEDIATE_COLORS,
        RankTier::Advanced => &ADVANCED_COLORS,
        RankTier::Expert => &EXPERT_COLORS,
        RankTier::Legendary => &LEGENDARY_COLORS,
    }
}

/// Beginner tier colors (grad-blue palette)
const BEGINNER_COLORS: [u8; 1] = [24];

/// Intermediate tier colors (dawn palette)
const INTERMEDIATE_COLORS: [u8; 3] = [24, 30, 36];

/// Advanced tier colors (forest palette)
const ADVANCED_COLORS: [u8; 5] = [28, 34, 40, 46, 82];

/// Expert tier colors (gold palette)
const EXPERT_COLORS: [u8; 3] = [202, 208, 214];

/// Legendary tier colors (fire palette)
const LEGENDARY_COLORS: [u8; 4] = [196, 197, 203, 209];
