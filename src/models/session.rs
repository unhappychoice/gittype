use super::stage::{Stage, StageResult};
use crate::scoring::ScoringEngine;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Session {
    pub stages: Vec<Stage>,
    pub session_start_time: Instant,
}

#[derive(Debug, Clone)]
pub struct SessionResult {
    pub session_start_time: Instant,
    pub session_duration: Duration,
    pub total_session_time: Duration, // Alias for session_duration for backward compatibility
    pub stages_completed: usize,
    pub stages_attempted: usize,
    pub stages_skipped: usize,
    pub total_challenges_completed: usize, // Alias for stages_completed
    pub total_challenges_attempted: usize, // Alias for stages_attempted  
    pub total_skips_used: usize, // Alias for stages_skipped
    pub stage_results: Vec<StageResult>,
    pub overall_accuracy: f64,
    pub overall_wpm: f64,
    pub overall_cpm: f64,
    pub total_keystrokes: usize,
    pub total_mistakes: usize,
    pub total_partial_effort_keystrokes: usize,
    pub total_partial_effort_mistakes: usize,
    pub best_stage_wpm: f64,
    pub worst_stage_wpm: f64,
    pub best_stage_accuracy: f64,
    pub worst_stage_accuracy: f64,
    pub session_score: f64,
}

impl Session {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self {
            stages,
            session_start_time: Instant::now(),
        }
    }
}

impl SessionResult {
    pub fn new() -> Self {
        Self {
            session_start_time: Instant::now(),
            session_duration: Duration::default(),
            total_session_time: Duration::default(),
            stages_completed: 0,
            stages_attempted: 0,
            stages_skipped: 0,
            total_challenges_completed: 0,
            total_challenges_attempted: 0,
            total_skips_used: 0,
            stage_results: Vec::new(),
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

    pub fn add_stage_result(
        &mut self,
        _stage_name: String,
        stage_result: StageResult,
        engine: &ScoringEngine,
    ) {
        self.total_challenges_completed += 1;
        self.stages_completed = self.total_challenges_completed;
        self.total_keystrokes += engine.total_chars();
        self.total_mistakes += stage_result.mistakes;
        self.session_score += stage_result.challenge_score;

        // Track best/worst performance
        if stage_result.wpm > self.best_stage_wpm {
            self.best_stage_wpm = stage_result.wpm;
        }
        if stage_result.wpm < self.worst_stage_wpm {
            self.worst_stage_wpm = stage_result.wpm;
        }
        if stage_result.accuracy > self.best_stage_accuracy {
            self.best_stage_accuracy = stage_result.accuracy;
        }
        if stage_result.accuracy < self.worst_stage_accuracy {
            self.worst_stage_accuracy = stage_result.accuracy;
        }
    }

    pub fn add_skip(&mut self) {
        self.total_skips_used += 1;
        self.stages_skipped = self.total_skips_used;
        self.total_challenges_attempted += 1;
        self.stages_attempted = self.total_challenges_attempted;
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
        self.session_duration = self.session_start_time.elapsed();
        self.total_session_time = self.session_duration;
        self.total_challenges_attempted = self.total_challenges_completed + self.total_skips_used;
        self.stages_attempted = self.total_challenges_attempted;

        // Calculate overall metrics
        if self.session_duration.as_secs() > 0 && self.total_keystrokes > 0 {
            self.overall_cpm =
                (self.total_keystrokes as f64 / self.session_duration.as_secs_f64()) * 60.0;
            self.overall_wpm = self.overall_cpm / 5.0;
            self.overall_accuracy = ((self.total_keystrokes.saturating_sub(self.total_mistakes))
                as f64
                / self.total_keystrokes as f64)
                * 100.0;
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
            (completed, 0) if completed > 0 => {
                format!("Perfect session! {} challenges completed", completed)
            }
            (completed, skips) => format!("{} completed, {} skipped", completed, skips),
        }
    }
}

impl Default for SessionResult {
    fn default() -> Self {
        Self::new()
    }
}