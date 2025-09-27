use crate::domain::models::SessionResult;
use crate::domain::services::scoring::{ScoreCalculator, SessionTracker};
use std::time::Duration;

/// Session level result calculation
pub struct SessionCalculator;

impl SessionCalculator {
    /// Calculate session result using SessionTracker data only
    pub fn calculate(tracker: &SessionTracker) -> SessionResult {
        let data = tracker.get_data();

        // Calculate valid session duration (completed stages only)
        let valid_session_duration: Duration = data
            .stage_results
            .iter()
            .filter(|sr| !sr.was_skipped && !sr.was_failed)
            .map(|sr| sr.completion_time)
            .sum();

        // Calculate invalid session duration (skipped/failed stages)
        let invalid_session_duration: Duration = data
            .stage_results
            .iter()
            .filter(|sr| sr.was_skipped || sr.was_failed)
            .map(|sr| sr.completion_time)
            .sum();

        // Total session duration for backward compatibility
        let session_duration = valid_session_duration + invalid_session_duration;

        // Calculate metrics from stage_results
        let stages_completed = data
            .stage_results
            .iter()
            .filter(|sr| !sr.was_skipped && !sr.was_failed)
            .count();

        let stages_skipped = data
            .stage_results
            .iter()
            .filter(|sr| sr.was_skipped)
            .count();

        let stages_attempted = data.stage_results.len();

        let valid_keystrokes: usize = data
            .stage_results
            .iter()
            .filter(|sr| !sr.was_skipped && !sr.was_failed)
            .map(|sr| sr.keystrokes)
            .sum();

        let valid_mistakes: usize = data
            .stage_results
            .iter()
            .filter(|sr| !sr.was_skipped && !sr.was_failed)
            .map(|sr| sr.mistakes)
            .sum();

        // Calculate session score from valid keystrokes and mistakes
        let session_score = if valid_keystrokes > 0 {
            let elapsed_secs = valid_session_duration.as_secs_f64().max(0.1);
            let cpm = (valid_keystrokes as f64 / elapsed_secs) * 60.0;
            let accuracy = ((valid_keystrokes.saturating_sub(valid_mistakes)) as f64
                / valid_keystrokes as f64)
                * 100.0;

            ScoreCalculator::calculate_score_from_metrics(
                cpm,
                accuracy,
                valid_mistakes,
                elapsed_secs,
                valid_keystrokes,
            )
        } else {
            0.0
        };

        // Find best and worst stage by challenge_score
        let best_stage = data.stage_results.iter().max_by(|a, b| {
            a.challenge_score
                .partial_cmp(&b.challenge_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let worst_stage = data.stage_results.iter().min_by(|a, b| {
            a.challenge_score
                .partial_cmp(&b.challenge_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let best_stage_wpm = best_stage.map(|s| s.wpm).unwrap_or(0.0);
        let best_stage_accuracy = best_stage.map(|s| s.accuracy).unwrap_or(0.0);
        let worst_stage_wpm = worst_stage.map(|s| s.wpm).unwrap_or(0.0);
        let worst_stage_accuracy = worst_stage.map(|s| s.accuracy).unwrap_or(0.0);

        // Session is successful if no stage failed
        let session_successful = !data.stage_results.iter().any(|sr| sr.was_failed);

        // Calculate invalid effort metrics from skipped/failed stages
        let invalid_keystrokes: usize = data
            .stage_results
            .iter()
            .filter(|sr| sr.was_skipped || sr.was_failed)
            .map(|sr| sr.keystrokes)
            .sum();

        let invalid_mistakes: usize = data
            .stage_results
            .iter()
            .filter(|sr| sr.was_skipped || sr.was_failed)
            .map(|sr| sr.mistakes)
            .sum();

        // Calculate overall metrics using valid session duration
        let (overall_wpm, overall_cpm, overall_accuracy) =
            if valid_session_duration.as_secs() > 0 && valid_keystrokes > 0 {
                let cpm = (valid_keystrokes as f64 / valid_session_duration.as_secs_f64()) * 60.0;
                let wpm = cpm / 5.0;
                let accuracy = ((valid_keystrokes.saturating_sub(valid_mistakes)) as f64
                    / valid_keystrokes as f64)
                    * 100.0;
                (wpm, cpm, accuracy)
            } else {
                (0.0, 0.0, 0.0)
            };

        SessionResult {
            session_start_time: data.session_start_time,
            session_duration,
            valid_session_duration,
            invalid_session_duration,
            stages_completed,
            stages_attempted,
            stages_skipped,
            stage_results: data.stage_results,
            overall_accuracy,
            overall_wpm,
            overall_cpm,
            valid_keystrokes,
            valid_mistakes,
            invalid_keystrokes,
            invalid_mistakes,
            best_stage_wpm,
            worst_stage_wpm,
            best_stage_accuracy,
            worst_stage_accuracy,
            session_score,
            session_successful,
        }
    }
}
