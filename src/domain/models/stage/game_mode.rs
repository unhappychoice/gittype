use crate::domain::models::DifficultyLevel;

#[derive(Debug, Clone)]
pub enum GameMode {
    Normal,     // Random selection of few challenges
    TimeAttack, // Time limit with all challenges
    Custom {
        // Custom configuration
        max_stages: Option<usize>,
        time_limit: Option<u64>, // seconds
        difficulty: DifficultyLevel,
    },
}
