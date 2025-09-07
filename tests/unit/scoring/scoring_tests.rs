
use gittype::scoring::{RankCalculator, ScoreCalculator};

#[cfg(test)]
mod rank_calculator_tests {
    use super::*;

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
}

#[cfg(test)]
mod score_calculator_tests {
    use super::*;

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
}
