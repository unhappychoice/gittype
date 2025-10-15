use gittype::domain::services::scoring::RankCalculator;

#[test]
fn test_beginner_tier_low_score() {
    // Very low score should be in Beginner tier
    let (tier_name, tier_pos, tier_total, overall_pos, overall_total) =
        RankCalculator::calculate_tier_info(100.0);

    assert_eq!(tier_name, "Beginner");
    assert!(tier_pos > 0);
    assert!(tier_total > 0);
    assert!(overall_pos > 0);
    assert!(overall_total > 0);
}

#[test]
fn test_tier_progression() {
    // Higher scores should progress through tiers
    let (tier_beginner, _, _, _, _) = RankCalculator::calculate_tier_info(500.0);
    let (tier_intermediate, _, _, _, _) = RankCalculator::calculate_tier_info(6000.0);
    let (tier_advanced, _, _, _, _) = RankCalculator::calculate_tier_info(8000.0);
    let (tier_expert, _, _, _, _) = RankCalculator::calculate_tier_info(10000.0);

    // Verify we're getting different tiers as score increases
    assert_eq!(tier_beginner, "Beginner");
    assert_eq!(tier_intermediate, "Intermediate");
    assert_eq!(tier_advanced, "Advanced");
    assert_eq!(tier_expert, "Expert");
}

#[test]
fn test_tier_position_within_tier() {
    // Tier position should be between 1 and tier_total
    let (_, tier_pos, tier_total, _, _) = RankCalculator::calculate_tier_info(1000.0);

    assert!(tier_pos >= 1);
    assert!(tier_pos <= tier_total);
}

#[test]
fn test_overall_position() {
    // Overall position should be between 1 and overall_total
    let (_, _, _, overall_pos, overall_total) = RankCalculator::calculate_tier_info(1000.0);

    assert!(overall_pos >= 1);
    assert!(overall_pos <= overall_total);
}

#[test]
fn test_tier_total_consistent() {
    // Different scores in the same tier should have the same tier_total
    let (tier1, _, tier_total1, _, _) = RankCalculator::calculate_tier_info(500.0);
    let (tier2, _, tier_total2, _, _) = RankCalculator::calculate_tier_info(600.0);

    if tier1 == tier2 {
        assert_eq!(tier_total1, tier_total2);
    }
}

#[test]
fn test_overall_total_constant() {
    // Overall total should be the same regardless of score
    let (_, _, _, _, overall_total1) = RankCalculator::calculate_tier_info(100.0);
    let (_, _, _, _, overall_total2) = RankCalculator::calculate_tier_info(10000.0);
    let (_, _, _, _, overall_total3) = RankCalculator::calculate_tier_info(50000.0);

    assert_eq!(overall_total1, overall_total2);
    assert_eq!(overall_total2, overall_total3);
}

#[test]
fn test_higher_score_better_position() {
    // Higher scores should have better (lower) overall positions
    let (_, _, _, pos_low, _) = RankCalculator::calculate_tier_info(1000.0);
    let (_, _, _, pos_high, _) = RankCalculator::calculate_tier_info(20000.0);

    // Lower position number is better
    assert!(pos_high < pos_low);
}

#[test]
fn test_very_high_score() {
    // Very high score should be in top tier
    let (tier_name, tier_pos, tier_total, overall_pos, _) =
        RankCalculator::calculate_tier_info(100000.0);

    // Should be in Legendary or Expert tier
    assert!(tier_name == "Legendary" || tier_name == "Expert");
    // Should be at or near the top of the tier
    assert!(tier_pos <= tier_total);
    // Should be near the top overall
    assert!(overall_pos <= 20); // Top 20 ranks
}

#[test]
fn test_zero_score() {
    // Zero score should still return valid tier information
    let (tier_name, tier_pos, tier_total, overall_pos, overall_total) =
        RankCalculator::calculate_tier_info(0.0);

    assert_eq!(tier_name, "Beginner");
    assert!(tier_pos > 0);
    assert!(tier_total > 0);
    assert!(overall_pos > 0);
    assert!(overall_total > 0);
}

#[test]
fn test_negative_score() {
    // Negative score should still return valid tier information (treated as 0 or low score)
    let (tier_name, tier_pos, tier_total, overall_pos, overall_total) =
        RankCalculator::calculate_tier_info(-100.0);

    assert_eq!(tier_name, "Beginner");
    assert!(tier_pos > 0);
    assert!(tier_total > 0);
    assert!(overall_pos > 0);
    assert!(overall_total > 0);
}
