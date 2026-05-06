use gittype::domain::models::ui::rank_colors::get_tier_colors;
use gittype::domain::models::RankTier;

#[test]
fn intermediate_tier_colors_use_dawn_palette() {
    assert_eq!(get_tier_colors(&RankTier::Intermediate), &[24, 30, 36]);
}
