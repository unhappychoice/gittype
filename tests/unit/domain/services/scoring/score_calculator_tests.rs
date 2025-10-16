use gittype::domain::services::scoring::ScoreCalculator;

#[test]
fn test_basic_score_calculation() {
    // Perfect typing: 600 CPM, 100% accuracy, no mistakes, 10 seconds, 100 chars
    let score = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 10.0, 100);
    // Base score = 600 * (100/100) * 10 = 6000
    // Consistency factor at 100% = 0.7 (0.5 base + 0.2 boost)
    // Consistency bonus = 6000 * 0.7 = 4200
    // Time bonus = (100/10 - 10) * 20 = 0 (no bonus as elapsed time equals ideal)
    // Final = (6000 + 4200 + 0 - 0) * 2 + 100 = 20500
    assert_eq!(score, 20500.0);
}

#[test]
fn test_accuracy_impact() {
    // Lower accuracy should reduce score
    let score_100 = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 10.0, 100);
    let score_90 = ScoreCalculator::calculate_score_from_metrics(600.0, 90.0, 0, 10.0, 100);
    let score_70 = ScoreCalculator::calculate_score_from_metrics(600.0, 70.0, 0, 10.0, 100);

    assert!(score_100 > score_90);
    assert!(score_90 > score_70);
}

#[test]
fn test_consistency_factor_low_accuracy() {
    // At 70% or below, consistency factor should be 0
    let score = ScoreCalculator::calculate_score_from_metrics(600.0, 70.0, 0, 10.0, 100);
    // Base score = 600 * 0.7 * 10 = 4200
    // Consistency factor = 0
    // Final = (4200 + 0 + 0 - 0) * 2 + 100 = 8500
    assert_eq!(score, 8500.0);
}

#[test]
fn test_consistency_factor_medium_accuracy() {
    // Between 70% and 90%, consistency factor should be between 0 and 0.5
    let score_75 = ScoreCalculator::calculate_score_from_metrics(600.0, 75.0, 0, 10.0, 100);
    let score_85 = ScoreCalculator::calculate_score_from_metrics(600.0, 85.0, 0, 10.0, 100);

    // Both should be between the 70% and 90% scores
    let score_70 = ScoreCalculator::calculate_score_from_metrics(600.0, 70.0, 0, 10.0, 100);
    let score_90 = ScoreCalculator::calculate_score_from_metrics(600.0, 90.0, 0, 10.0, 100);

    assert!(score_75 > score_70);
    assert!(score_75 < score_90);
    assert!(score_85 > score_75);
    assert!(score_85 < score_90);
}

#[test]
fn test_consistency_factor_high_accuracy() {
    // At 95% or above, consistency factor should be between 0.5 and 0.7
    let score_95 = ScoreCalculator::calculate_score_from_metrics(600.0, 95.0, 0, 10.0, 100);
    let score_100 = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 10.0, 100);

    // 100% should have higher score than 95% due to consistency boost
    assert!(score_100 > score_95);
}

#[test]
fn test_time_bonus() {
    // Fast typing (5 seconds for 100 chars) should get time bonus
    let score_fast = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 5.0, 100);
    // Ideal time = 100/10 = 10 seconds
    // Time bonus = (10 - 5) * 20 = 100

    // Normal speed (10 seconds) should get no bonus
    let score_normal = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 10.0, 100);

    // Slow typing (15 seconds) should get no bonus
    let score_slow = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 15.0, 100);

    assert!(score_fast > score_normal);
    assert_eq!(score_normal, score_slow); // No penalty for being slow, just no bonus
}

#[test]
fn test_time_bonus_only_for_long_challenges() {
    // Time bonus should only apply when total_chars > 50
    let score_short = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 2.0, 30);
    let score_short_slow = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 5.0, 30);

    // Both should have same score (no time bonus for short challenges)
    assert_eq!(score_short, score_short_slow);
}

#[test]
fn test_mistake_penalty() {
    // Each mistake should reduce score by 5 * 2 = 10 points
    let score_no_mistakes =
        ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 0, 10.0, 100);
    let score_1_mistake = ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 1, 10.0, 100);
    let score_5_mistakes =
        ScoreCalculator::calculate_score_from_metrics(600.0, 100.0, 5, 10.0, 100);

    assert_eq!(score_no_mistakes - score_1_mistake, 10.0);
    assert_eq!(score_no_mistakes - score_5_mistakes, 50.0);
}

#[test]
fn test_zero_cpm() {
    // Zero CPM should result in low score
    let score = ScoreCalculator::calculate_score_from_metrics(0.0, 100.0, 0, 10.0, 100);
    assert_eq!(score, 100.0); // Base minimum score
}

#[test]
fn test_zero_accuracy() {
    // Zero accuracy should result in zero base score
    let score = ScoreCalculator::calculate_score_from_metrics(600.0, 0.0, 0, 10.0, 100);
    assert_eq!(score, 100.0); // Minimum score
}

#[test]
fn test_negative_score_clamped_to_zero() {
    // Very high mistakes should not result in negative score
    let score = ScoreCalculator::calculate_score_from_metrics(100.0, 50.0, 1000, 10.0, 100);
    assert!(score >= 0.0);
}

#[test]
fn test_realistic_typing_scenario() {
    // Simulate a realistic typing session: 400 CPM, 95% accuracy, 2 mistakes, 30 seconds, 200 chars
    let score = ScoreCalculator::calculate_score_from_metrics(400.0, 95.0, 2, 30.0, 200);

    // Score should be reasonable (not too high, not too low)
    assert!(score > 1000.0);
    assert!(score < 50000.0);
}

#[test]
fn test_excellent_performance() {
    // Excellent performance: 800 CPM, 99% accuracy, 0 mistakes, 15 seconds, 200 chars
    let score = ScoreCalculator::calculate_score_from_metrics(800.0, 99.0, 0, 15.0, 200);

    // Should result in high score
    assert!(score > 20000.0);
}

#[test]
fn test_poor_performance() {
    // Poor performance: 200 CPM, 75% accuracy, 10 mistakes, 60 seconds, 200 chars
    let score = ScoreCalculator::calculate_score_from_metrics(200.0, 75.0, 10, 60.0, 200);

    // Should result in low score
    assert!(score < 5000.0);
}
