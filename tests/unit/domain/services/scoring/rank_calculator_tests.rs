use gittype::domain::services::scoring::RankCalculator;

#[test]
fn test_calculate_tier_info_beginner() {
    let score = 100.0; // Maps to "Hello World"
    let (tier_name, tier_position, tier_total, overall_position, overall_total) =
        RankCalculator::calculate_tier_info(score);

    assert_eq!(tier_name, "Beginner");
    assert_eq!(tier_position, 12);
    assert_eq!(tier_total, 12);
    assert_eq!(overall_position, 63);
    assert_eq!(overall_total, 63);
}

#[test]
fn test_calculate_tier_info_expert() {
    let score = 9600.0; // Maps to "Compiler"
    let (tier_name, tier_position, tier_total, overall_position, overall_total) =
        RankCalculator::calculate_tier_info(score);

    assert_eq!(tier_name, "Expert");
    assert_eq!(tier_position, 12);
    assert_eq!(tier_total, 12);
    assert_eq!(overall_position, 27);
    assert_eq!(overall_total, 63);
}

#[test]
fn test_calculate_tier_info_legendary() {
    let score = 12000.0;
    let (tier_name, tier_position, tier_total, overall_position, overall_total) =
        RankCalculator::calculate_tier_info(score);

    assert_eq!(tier_name, "Legendary");
    assert_eq!(tier_position, 14);
    assert_eq!(tier_total, 15);
    assert_eq!(overall_position, 14);
    assert_eq!(overall_total, 63);
}

#[test]
fn test_calculate_tier_info_extreme_score() {
    let score = 1_000_000.0;
    let (tier_name, tier_position, tier_total, overall_position, overall_total) =
        RankCalculator::calculate_tier_info(score);

    assert_eq!(tier_name, "Legendary");
    assert_eq!(tier_position, 1);
    assert_eq!(tier_total, 15);
    assert_eq!(overall_position, 1);
    assert_eq!(overall_total, 63);
}
