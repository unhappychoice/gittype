use gittype::domain::services::scoring::calculator::RealTimeCalculator;
use std::time::Duration;

const EPSILON: f64 = 0.001;

#[test]
fn test_calculate_basic() {
    let current_position = 100;
    let mistakes = 0;
    let elapsed_time = Duration::from_secs(60);
    let result = RealTimeCalculator::calculate(current_position, mistakes, elapsed_time);

    assert!((result.wpm - 20.0).abs() < EPSILON);
    assert!((result.cpm - 100.0).abs() < EPSILON);
    assert!((result.accuracy - 100.0).abs() < EPSILON);
    assert_eq!(result.mistakes, 0);
}

#[test]
fn test_calculate_with_mistakes() {
    let current_position = 100;
    let mistakes = 10;
    let elapsed_time = Duration::from_secs(60);
    let result = RealTimeCalculator::calculate(current_position, mistakes, elapsed_time);

    assert!((result.wpm - 20.0).abs() < EPSILON); // CPM is based on total characters, not correct ones
    assert!((result.cpm - 100.0).abs() < EPSILON);
    assert!((result.accuracy - 90.0).abs() < EPSILON);
    assert_eq!(result.mistakes, 10);
}

#[test]
fn test_calculate_zero_position() {
    let current_position = 0;
    let mistakes = 0;
    let elapsed_time = Duration::from_secs(10);
    let result = RealTimeCalculator::calculate(current_position, mistakes, elapsed_time);

    assert!((result.wpm - 0.0).abs() < EPSILON);
    assert!((result.cpm - 0.0).abs() < EPSILON);
    assert!((result.accuracy - 0.0).abs() < EPSILON);
    assert_eq!(result.mistakes, 0);
}

#[test]
fn test_calculate_zero_elapsed_time() {
    let current_position = 50;
    let mistakes = 0;
    let elapsed_time = Duration::from_millis(0);
    let result = RealTimeCalculator::calculate(current_position, mistakes, elapsed_time);

    // elapsed_secs is max(0.1), so 50 / 0.1 * 60 = 30000
    assert!((result.cpm - 30000.0).abs() < EPSILON);
    assert!((result.wpm - 6000.0).abs() < EPSILON);
    assert!((result.accuracy - 100.0).abs() < EPSILON);
    assert_eq!(result.mistakes, 0);
}

#[test]
fn test_calculate_high_mistakes() {
    let current_position = 10;
    let mistakes = 10;
    let elapsed_time = Duration::from_secs(1);
    let result = RealTimeCalculator::calculate(current_position, mistakes, elapsed_time);

    assert!((result.wpm - 120.0).abs() < EPSILON); // CPM is based on total characters, not correct ones
    assert!((result.cpm - 600.0).abs() < EPSILON);
    assert!((result.accuracy - 0.0).abs() < EPSILON);
    assert_eq!(result.mistakes, 10);
}
