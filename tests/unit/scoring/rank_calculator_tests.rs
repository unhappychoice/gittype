use gittype::scoring::RankCalculator;

#[test]
fn test_calculate_tier_info_beginner() {
    let score = 100.0; // Maps to "Hello World"
    let (tier_name, tier_position, tier_total, overall_position, overall_total) =
        RankCalculator::calculate_tier_info(score);

    assert_eq!(tier_name, "Beginner");
    assert_eq!(tier_position, 1);
    assert_eq!(tier_total, 12); // 12 ranks in Beginner tier
    assert_eq!(overall_position, 1);
    assert_eq!(overall_total, 63); // Total ranks
}

#[test]
fn test_calculate_tier_info_expert() {
    let score = 9600.0; // Maps to "Compiler"
    let (tier_name, tier_position, tier_total, overall_position, overall_total) =
        RankCalculator::calculate_tier_info(score);

    assert_eq!(tier_name, "Expert");
    assert_eq!(tier_position, 1);
    assert_eq!(tier_total, 12); // 12 ranks in Expert tier
    assert_eq!(overall_position, 37); // 12 Beginner + 12 Intermediate + 12 Advanced + 1 Expert
    assert_eq!(overall_total, 63);
}

#[test]
fn test_calculate_tier_info_legendary() {
    let score = 12000.0; // Maps to "DNS Overlord"
    let (tier_name, tier_position, tier_total, overall_position, overall_total) =
        RankCalculator::calculate_tier_info(score);

    assert_eq!(tier_name, "Legendary");
    assert_eq!(tier_position, 2); // "DNS Overlord" is the 2nd rank in Legendary tier
    assert_eq!(tier_total, 15); // 15 ranks in Legendary tier
    assert_eq!(overall_position, 50); // 12 Beginner + 12 Intermediate + 12 Advanced + 12 Expert + 2 Legendary
    assert_eq!(overall_total, 63);
}