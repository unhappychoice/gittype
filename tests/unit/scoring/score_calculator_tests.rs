use gittype::scoring::{ScoreCalculator};


const EPSILON: f64 = 0.001;

#[test]
fn test_calculate_score_from_metrics_basic() {
    let cpm = 300.0;
    let accuracy = 95.0;
    let mistakes = 0;
    let elapsed_secs = 60.0;
    let total_chars = 300;
    let score = ScoreCalculator::calculate_score_from_metrics(
        cpm,
        accuracy,
        mistakes,
        elapsed_secs,
        total_chars,
    );
    assert!((score - 8650.0).abs() < EPSILON);
}

#[test]
fn test_calculate_score_from_metrics_with_mistakes() {
    let cpm = 200.0;
    let accuracy = 90.0;
    let mistakes = 5;
    let elapsed_secs = 60.0;
    let total_chars = 200;
    let score = ScoreCalculator::calculate_score_from_metrics(
        cpm,
        accuracy,
        mistakes,
        elapsed_secs,
        total_chars,
    );
    assert!((score - 5450.0).abs() < EPSILON);
}

#[test]
fn test_calculate_score_from_metrics_low_accuracy() {
    let cpm = 100.0;
    let accuracy = 50.0;
    let mistakes = 0;
    let elapsed_secs = 60.0;
    let total_chars = 100;
    let score = ScoreCalculator::calculate_score_from_metrics(
        cpm,
        accuracy,
        mistakes,
        elapsed_secs,
        total_chars,
    );
    assert!((score - 1100.0).abs() < EPSILON);
}

#[test]
fn test_calculate_score_from_metrics_with_time_bonus() {
    let cpm = 300.0;
    let accuracy = 99.0;
    let mistakes = 0;
    let elapsed_secs = 20.0;
    let total_chars = 300;
    let score = ScoreCalculator::calculate_score_from_metrics(
        cpm,
        accuracy,
        mistakes,
        elapsed_secs,
        total_chars,
    );
    let expected_score = {
        let base_score = cpm * (accuracy / 100.0) * 10.0;

        let a = accuracy.clamp(0.0, 100.0) / 100.0;
        let consistency_factor = if a <= 0.7 {
            0.0
        } else if a < 0.9 {
            let t = (a - 0.7) / 0.2;
            let s = t * t * (3.0 - 2.0 * t);
            0.5 * s
        } else if a < 0.95 {
            0.5
        } else {
            let t = (a - 0.95) / 0.05;
            let s = t * t * (3.0 - 2.0 * t);
            0.5 + 0.2 * s
        };
        let consistency_bonus = base_score * consistency_factor;

        let time_bonus = if total_chars > 50 {
            let ideal_time = total_chars as f64 / 10.0;
            if elapsed_secs < ideal_time {
                (ideal_time - elapsed_secs) * 20.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        let mistake_penalty = mistakes as f64 * 5.0;
        let raw_score = base_score + consistency_bonus + time_bonus - mistake_penalty;
        (raw_score * 2.0 + 100.0).max(0.0)
    };
    assert!((score - expected_score).abs() < EPSILON);
}
