use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct StoredStageResult {
    pub id: i64,
    pub repository_id: Option<i64>,
    pub repository_name: Option<String>,
    pub user_name: Option<String>,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub keystrokes: i64,
    pub mistakes: i64,
    pub duration_ms: i64,
    pub score: f64,
    pub language: Option<String>,
    pub difficulty_level: Option<String>,
    pub completed_at: DateTime<Utc>,
    pub rank_name: Option<String>,
    pub tier_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StageStatistics {
    pub total_completed: i64,
    pub avg_wpm: f64,
    pub max_wpm: f64,
    pub min_wpm: f64,
    pub avg_accuracy: f64,
    pub max_accuracy: f64,
    pub min_accuracy: f64,
    pub avg_score: f64,
    pub total_keystrokes: i64,
    pub total_mistakes: i64,
}

#[derive(Debug, Clone)]
pub struct LanguageStats {
    pub language: String,
    pub stage_count: i64,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub avg_score: f64,
}

#[derive(Debug, Clone)]
pub struct DifficultyStats {
    pub difficulty_level: String,
    pub stage_count: i64,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub avg_score: f64,
}