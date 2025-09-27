use crate::domain::models::TotalResult;
use crate::domain::services::scoring::TotalTracker;
use std::time::{Duration, Instant};

/// Total level result calculation
pub struct TotalCalculator;

impl TotalCalculator {
    pub fn calculate(tracker: &TotalTracker) -> TotalResult {
        let data = tracker.get_data();
        let session_results = &data.session_results;

        let total_sessions_attempted = session_results.len();
        let total_sessions_completed = session_results
            .iter()
            .filter(|sr| sr.session_successful)
            .count();

        let total_stages_attempted: usize =
            session_results.iter().map(|sr| sr.stages_attempted).sum();
        let total_stages_completed: usize =
            session_results.iter().map(|sr| sr.stages_completed).sum();
        let total_stages_skipped: usize = session_results.iter().map(|sr| sr.stages_skipped).sum();

        let total_keystrokes: usize = session_results
            .iter()
            .map(|sr| sr.valid_keystrokes + sr.invalid_keystrokes)
            .sum();
        let total_mistakes: usize = session_results
            .iter()
            .map(|sr| sr.valid_mistakes + sr.invalid_mistakes)
            .sum();

        let total_time_secs: f64 = session_results
            .iter()
            .map(|sr| sr.session_duration.as_secs_f64())
            .sum();

        let total_score: f64 = session_results
            .iter()
            .filter(|sr| sr.stages_completed > 0)
            .map(|sr| sr.session_score)
            .sum();

        // Calculate overall metrics
        let (overall_cpm, overall_wpm) = if total_time_secs > 0.0 && total_keystrokes > 0 {
            let cpm = (total_keystrokes as f64 / total_time_secs) * 60.0;
            let wpm = cpm / 5.0;
            (cpm, wpm)
        } else {
            (0.0, 0.0)
        };

        let overall_accuracy = if total_keystrokes > 0 {
            ((total_keystrokes.saturating_sub(total_mistakes)) as f64 / total_keystrokes as f64)
                * 100.0
        } else {
            0.0
        };

        // Calculate best/worst session metrics from sessions with best/worst scores
        let valid_sessions: Vec<_> = session_results
            .iter()
            .filter(|sr| sr.stages_completed > 0)
            .collect();

        let (best_session_wpm, best_session_accuracy) = if valid_sessions.is_empty() {
            (0.0, 0.0)
        } else {
            // Find session with highest score
            let best_session = valid_sessions
                .iter()
                .max_by(|a, b| a.session_score.partial_cmp(&b.session_score).unwrap())
                .unwrap();
            (best_session.overall_wpm, best_session.overall_accuracy)
        };

        let (worst_session_wpm, worst_session_accuracy) = if valid_sessions.is_empty() {
            (0.0, 0.0)
        } else {
            // Find session with lowest score
            let worst_session = valid_sessions
                .iter()
                .min_by(|a, b| a.session_score.partial_cmp(&b.session_score).unwrap())
                .unwrap();
            (worst_session.overall_wpm, worst_session.overall_accuracy)
        };

        TotalResult {
            start_time: Instant::now(), // This should ideally be tracked properly
            total_duration: Duration::from_secs_f64(total_time_secs),
            total_sessions_attempted,
            total_sessions_completed,
            total_stages_attempted,
            total_stages_completed,
            total_stages_skipped,
            total_keystrokes,
            total_mistakes,
            overall_accuracy,
            overall_wpm,
            overall_cpm,
            best_session_wpm,
            worst_session_wpm,
            best_session_accuracy,
            worst_session_accuracy,
            total_score,
            session_results: session_results.clone(),
        }
    }
}
