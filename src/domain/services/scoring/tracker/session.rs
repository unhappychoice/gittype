use crate::domain::models::StageResult;
use shaku::Interface;
use std::sync::RwLock;
use std::time::Instant;

pub trait SessionTrackerInterface: Interface {
    fn record(&self, stage_result: StageResult);
    fn get_data(&self) -> SessionTrackerData;
    fn reset(&self);
}

pub struct SessionTrackerState {
    pub session_start_time: Instant,
    pub stage_results: Vec<StageResult>,
}

impl Default for SessionTrackerState {
    fn default() -> Self {
        Self {
            session_start_time: Instant::now(),
            stage_results: Vec::new(),
        }
    }
}

/// Session level raw data tracking
#[derive(shaku::Component)]
#[shaku(interface = SessionTrackerInterface)]
pub struct SessionTracker {
    #[shaku(default)]
    state: RwLock<SessionTrackerState>,
}

impl Default for SessionTracker {
    fn default() -> Self {
        Self {
            state: RwLock::new(SessionTrackerState::default()),
        }
    }
}

impl SessionTrackerInterface for SessionTracker {
    fn record(&self, stage_result: StageResult) {
        self.state.write().unwrap().stage_results.push(stage_result);
    }

    fn get_data(&self) -> SessionTrackerData {
        let state = self.state.read().unwrap();
        SessionTrackerData {
            session_start_time: state.session_start_time,
            stage_results: state.stage_results.clone(),
        }
    }

    fn reset(&self) {
        let mut state = self.state.write().unwrap();
        state.session_start_time = Instant::now();
        state.stage_results.clear();
    }
}

#[cfg(feature = "test-mocks")]
impl SessionTracker {
    pub fn new_for_test() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct SessionTrackerData {
    pub session_start_time: Instant,
    pub stage_results: Vec<StageResult>,
}
