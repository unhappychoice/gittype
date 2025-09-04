use crate::models::{SessionResult, TotalResult};

#[derive(Clone)]
pub struct TotalTracker {
    total_result: TotalResult,
}

impl TotalTracker {
    pub fn new() -> Self {
        Self {
            total_result: TotalResult::new(),
        }
    }

    /// Record a completed session
    pub fn record_session_completion(&mut self, session_result: &SessionResult) {
        self.total_result.total_sessions_attempted += 1;
        self.total_result.total_sessions_completed += 1;

        self.total_result.total_stages_attempted += session_result.stages_attempted;
        self.total_result.total_stages_completed += session_result.stages_completed;
        self.total_result.total_stages_skipped += session_result.stages_skipped;

        self.total_result.total_keystrokes += session_result.total_effort_keystrokes();
        self.total_result.total_mistakes += session_result.total_effort_mistakes();

        self.total_result
            .session_results
            .push(session_result.clone());

        // Update best/worst session metrics
        if session_result.overall_wpm > self.total_result.best_session_wpm {
            self.total_result.best_session_wpm = session_result.overall_wpm;
        }
        if session_result.overall_wpm < self.total_result.worst_session_wpm {
            self.total_result.worst_session_wpm = session_result.overall_wpm;
        }
        if session_result.overall_accuracy > self.total_result.best_session_accuracy {
            self.total_result.best_session_accuracy = session_result.overall_accuracy;
        }
        if session_result.overall_accuracy < self.total_result.worst_session_accuracy {
            self.total_result.worst_session_accuracy = session_result.overall_accuracy;
        }

        // Update total score
        self.total_result.total_score += session_result.session_score;

        // Recalculate overall metrics
        self.recalculate_overall_metrics();
    }

    /// Record a failed session attempt (session was started but not completed)
    pub fn record_session_attempt(&mut self, session_result: &SessionResult) {
        self.total_result.total_sessions_attempted += 1;
        // Don't increment completed count for failed sessions

        self.total_result.total_stages_attempted += session_result.stages_attempted;
        self.total_result.total_stages_completed += session_result.stages_completed;
        self.total_result.total_stages_skipped += session_result.stages_skipped;

        self.total_result.total_keystrokes += session_result.total_effort_keystrokes();
        self.total_result.total_mistakes += session_result.total_effort_mistakes();

        // Still add to session results for tracking
        self.total_result
            .session_results
            .push(session_result.clone());

        // Update metrics even for failed sessions
        if session_result.stages_completed > 0 {
            if session_result.overall_wpm > self.total_result.best_session_wpm {
                self.total_result.best_session_wpm = session_result.overall_wpm;
            }
            if session_result.overall_wpm < self.total_result.worst_session_wpm {
                self.total_result.worst_session_wpm = session_result.overall_wpm;
            }
            if session_result.overall_accuracy > self.total_result.best_session_accuracy {
                self.total_result.best_session_accuracy = session_result.overall_accuracy;
            }
            if session_result.overall_accuracy < self.total_result.worst_session_accuracy {
                self.total_result.worst_session_accuracy = session_result.overall_accuracy;
            }

            self.total_result.total_score += session_result.session_score;
        }

        self.recalculate_overall_metrics();
    }

    fn recalculate_overall_metrics(&mut self) {
        if self.total_result.total_keystrokes > 0 {
            let total_duration_secs: f64 = self
                .total_result
                .session_results
                .iter()
                .map(|s| s.total_session_time.as_secs_f64())
                .sum();

            if total_duration_secs > 0.0 {
                self.total_result.overall_cpm =
                    (self.total_result.total_keystrokes as f64 / total_duration_secs) * 60.0;
                self.total_result.overall_wpm = self.total_result.overall_cpm / 5.0;
            }

            self.total_result.overall_accuracy = ((self
                .total_result
                .total_keystrokes
                .saturating_sub(self.total_result.total_mistakes))
                as f64
                / self.total_result.total_keystrokes as f64)
                * 100.0;
        }
    }

    pub fn finalize_and_get_total(mut self) -> TotalResult {
        self.total_result.finalize();
        self.total_result
    }

    pub fn get_current_total(&self) -> &TotalResult {
        &self.total_result
    }
}

impl Default for TotalTracker {
    fn default() -> Self {
        Self::new()
    }
}
