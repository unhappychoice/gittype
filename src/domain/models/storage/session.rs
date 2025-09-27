use crate::domain::models::{Challenge, StageResult};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct StoredSession {
    pub id: i64,
    pub repository_id: Option<i64>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub branch: Option<String>,
    pub commit_hash: Option<String>,
    pub is_dirty: bool,
    pub game_mode: String,
    pub difficulty_level: Option<String>,
    pub max_stages: Option<i32>,
    pub time_limit_seconds: Option<i32>,
}

/// Session stage result data
#[derive(Debug, Clone)]
pub struct SessionStageResult {
    pub stage_number: i64,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub keystrokes: usize,
    pub mistakes: usize,
    pub duration_ms: u64,
    pub score: f64,
    pub language: Option<String>,
    pub difficulty_level: Option<String>,
    pub rank_name: Option<String>,
    pub tier_name: Option<String>,
    pub rank_position: usize,
    pub rank_total: usize,
    pub position: usize,
    pub total: usize,
    pub was_skipped: bool,
    pub was_failed: bool,
    // Challenge details
    pub file_path: Option<String>,
    pub start_line: Option<i64>,
    pub end_line: Option<i64>,
    pub code_content: Option<String>,
}

/// Detailed session stage result data with all fields
#[derive(Debug, Clone)]
pub struct DetailedSessionStageResult {
    pub stage_number: i64,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub keystrokes: usize,
    pub mistakes: usize,
    pub duration_ms: u64,
    pub score: f64,
    pub language: Option<String>,
    pub difficulty_level: Option<String>,
    pub rank_name: Option<String>,
    pub tier_name: Option<String>,
    pub rank_position: usize,
    pub rank_total: usize,
    pub position: usize,
    pub total: usize,
    pub was_skipped: bool,
    pub was_failed: bool,
    // Challenge details
    pub file_path: Option<String>,
    pub start_line: Option<i64>,
    pub end_line: Option<i64>,
    pub code_content: Option<String>,
}

/// Session result data aggregate
#[derive(Debug, Clone)]
pub struct SessionResultData {
    pub keystrokes: usize,
    pub mistakes: usize,
    pub duration_ms: u64,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub stages_completed: usize,
    pub stages_attempted: usize,
    pub stages_skipped: usize,
    pub score: f64,
    pub rank_name: Option<String>,
    pub tier_name: Option<String>,
    pub rank_position: Option<i64>,
    pub rank_total: Option<i64>,
    pub position: Option<i64>,
    pub total: Option<i64>,
}

/// Parameters for saving stage results
pub struct SaveStageParams<'a> {
    pub session_id: i64,
    pub repository_id: Option<i64>,
    pub stage_index: usize,
    pub stage_name: &'a str,
    pub stage_result: &'a StageResult,
    pub keystrokes: usize,
    pub challenge: Option<&'a Challenge>,
}
