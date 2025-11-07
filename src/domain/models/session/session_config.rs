use std::time::Duration;

use crate::domain::models::DifficultyLevel;

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub max_stages: usize,
    pub session_timeout: Option<Duration>,
    pub difficulty: DifficultyLevel,
    pub max_skips: usize,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_stages: 3,
            session_timeout: None,
            difficulty: DifficultyLevel::Normal,
            max_skips: 3,
        }
    }
}
