use super::session::{Session, SessionResult};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Total {
    pub sessions: Vec<Session>,
    pub start_time: Instant,
}

#[derive(Debug, Clone)]
pub struct TotalResult {
    pub start_time: Instant,
    pub total_duration: Duration,
    pub total_sessions_completed: usize,
    pub total_sessions_attempted: usize,
    pub session_results: Vec<SessionResult>,
    pub total_stages_completed: usize,
    pub total_stages_attempted: usize,
    pub total_stages_skipped: usize,
    pub overall_accuracy: f64,
    pub overall_wpm: f64,
    pub overall_cpm: f64,
    pub total_keystrokes: usize,
    pub total_mistakes: usize,
    pub best_session_wpm: f64,
    pub worst_session_wpm: f64,
    pub best_session_accuracy: f64,
    pub worst_session_accuracy: f64,
    pub total_score: f64,
}

impl Total {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }
}

impl TotalResult {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_duration: Duration::default(),
            total_sessions_completed: 0,
            total_sessions_attempted: 0,
            session_results: Vec::new(),
            total_stages_completed: 0,
            total_stages_attempted: 0,
            total_stages_skipped: 0,
            overall_accuracy: 0.0,
            overall_wpm: 0.0,
            overall_cpm: 0.0,
            total_keystrokes: 0,
            total_mistakes: 0,
            best_session_wpm: 0.0,
            worst_session_wpm: f64::MAX,
            best_session_accuracy: 0.0,
            worst_session_accuracy: f64::MAX,
            total_score: 0.0,
        }
    }

    pub fn finalize(&mut self) {
        self.total_duration = self.start_time.elapsed();
        
        // Handle edge cases for worst performance
        if self.worst_session_wpm == f64::MAX {
            self.worst_session_wpm = 0.0;
        }
        if self.worst_session_accuracy == f64::MAX {
            self.worst_session_accuracy = 0.0;
        }
    }

    pub fn get_completion_status(&self) -> String {
        match (self.total_sessions_completed, self.total_sessions_attempted) {
            (0, 0) => "No sessions attempted".to_string(),
            (completed, attempted) if completed == attempted => {
                format!("Perfect! {} sessions completed", completed)
            }
            (completed, attempted) => format!("{}/{} sessions completed", completed, attempted),
        }
    }
}

impl Default for Total {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TotalResult {
    fn default() -> Self {
        Self::new()
    }
}