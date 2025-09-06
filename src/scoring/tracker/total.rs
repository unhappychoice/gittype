use crate::models::SessionResult;

/// Total level raw data tracking
#[derive(Clone)]
pub struct TotalTracker {
    session_results: Vec<SessionResult>,
}

impl TotalTracker {
    pub fn new() -> Self {
        Self {
            session_results: Vec::new(),
        }
    }

    pub fn record(&mut self, session_result: SessionResult) {
        self.session_results.push(session_result);
    }

    pub fn get_data(&self) -> TotalTrackerData {
        TotalTrackerData {
            session_results: self.session_results.clone(),
        }
    }
}

impl Default for TotalTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TotalTrackerData {
    pub session_results: Vec<SessionResult>,
}
