use std::time::{Duration, Instant};
use crate::scoring::{TypingMetrics, ScoringEngine};

#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub session_start_time: Instant,
    pub total_session_time: Duration,
    pub total_challenges_completed: usize,
    pub total_challenges_attempted: usize,
    pub total_skips_used: usize,
    pub overall_accuracy: f64,
    pub overall_wpm: f64,
    pub overall_cpm: f64,
    // Completed stages only (for score calculation)
    pub total_keystrokes: usize,
    pub total_mistakes: usize,
    // Partial effort only (Skip + Exit stages)
    pub total_partial_effort_keystrokes: usize,
    pub total_partial_effort_mistakes: usize,
    pub best_stage_wpm: f64,
    pub worst_stage_wpm: f64,
    pub best_stage_accuracy: f64,
    pub worst_stage_accuracy: f64,
    pub session_score: f64,
}

impl SessionSummary {
    pub fn new() -> Self {
        Self {
            session_start_time: Instant::now(),
            total_session_time: Duration::default(),
            total_challenges_completed: 0,
            total_challenges_attempted: 0,
            total_skips_used: 0,
            overall_accuracy: 0.0,
            overall_wpm: 0.0,
            overall_cpm: 0.0,
            total_keystrokes: 0,
            total_mistakes: 0,
            total_partial_effort_keystrokes: 0,
            total_partial_effort_mistakes: 0,
            best_stage_wpm: 0.0,
            worst_stage_wpm: f64::MAX,
            best_stage_accuracy: 0.0,
            worst_stage_accuracy: f64::MAX,
            session_score: 0.0,
        }
    }

    pub fn add_stage_result(&mut self, _stage_name: String, metrics: TypingMetrics, engine: &ScoringEngine) {
        self.total_challenges_completed += 1;
        self.total_keystrokes += engine.total_chars();
        self.total_mistakes += metrics.mistakes;
        self.session_score += metrics.challenge_score;
        
        // Track best/worst performance
        if metrics.wpm > self.best_stage_wpm {
            self.best_stage_wpm = metrics.wpm;
        }
        if metrics.wpm < self.worst_stage_wpm {
            self.worst_stage_wpm = metrics.wpm;
        }
        if metrics.accuracy > self.best_stage_accuracy {
            self.best_stage_accuracy = metrics.accuracy;
        }
        if metrics.accuracy < self.worst_stage_accuracy {
            self.worst_stage_accuracy = metrics.accuracy;
        }
    }

    pub fn add_skip(&mut self) {
        self.total_skips_used += 1;
        self.total_challenges_attempted += 1;
    }

    pub fn add_partial_effort(&mut self, keystrokes: usize, mistakes: usize) {
        self.total_partial_effort_keystrokes += keystrokes;
        self.total_partial_effort_mistakes += mistakes;
    }

    // Calculate total effort including both completed and partial
    pub fn total_effort_keystrokes(&self) -> usize {
        self.total_keystrokes + self.total_partial_effort_keystrokes
    }

    pub fn total_effort_mistakes(&self) -> usize {
        self.total_mistakes + self.total_partial_effort_mistakes
    }

    pub fn finalize_session(&mut self) {
        self.total_session_time = self.session_start_time.elapsed();
        self.total_challenges_attempted = self.total_challenges_completed + self.total_skips_used;
        
        // Calculate overall metrics - simplified since we don't track individual stage times
        if self.total_session_time.as_secs() > 0 && self.total_keystrokes > 0 {
            self.overall_cpm = (self.total_keystrokes as f64 / self.total_session_time.as_secs_f64()) * 60.0;
            self.overall_wpm = self.overall_cpm / 5.0;
            self.overall_accuracy = ((self.total_keystrokes.saturating_sub(self.total_mistakes)) as f64 / self.total_keystrokes as f64) * 100.0;
        }
        
        // Handle edge cases for worst performance
        if self.worst_stage_wpm == f64::MAX {
            self.worst_stage_wpm = 0.0;
        }
        if self.worst_stage_accuracy == f64::MAX {
            self.worst_stage_accuracy = 0.0;
        }
    }

    pub fn get_session_completion_status(&self) -> String {
        match (self.total_challenges_completed, self.total_skips_used) {
            (0, 0) => "No challenges attempted".to_string(),
            (completed, 0) if completed > 0 => format!("Perfect session! {} challenges completed", completed),
            (completed, skips) => format!("{} completed, {} skipped", completed, skips),
        }
    }
}

impl Default for SessionSummary {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct SessionTracker {
    summary: SessionSummary,
}

impl SessionTracker {
    pub fn new() -> Self {
        Self {
            summary: SessionSummary::new(),
        }
    }

    pub fn record_stage_completion(&mut self, stage_name: String, metrics: TypingMetrics, engine: &ScoringEngine) {
        self.summary.add_stage_result(stage_name, metrics, engine);
    }

    pub fn record_skip(&mut self) {
        self.summary.add_skip();
    }

    pub fn record_partial_effort(&mut self, engine: &ScoringEngine, metrics: &TypingMetrics) {
        self.summary.add_partial_effort(engine.total_chars(), metrics.mistakes);
    }

    pub fn finalize_and_get_summary(mut self) -> SessionSummary {
        self.summary.finalize_session();
        self.summary
    }

    pub fn get_current_summary(&self) -> &SessionSummary {
        &self.summary
    }
}

impl Default for SessionTracker {
    fn default() -> Self {
        Self::new()
    }
}