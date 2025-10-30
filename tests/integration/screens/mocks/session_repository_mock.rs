use gittype::domain::models::storage::{SessionStageResult, StoredRepository, StoredSession};
use gittype::domain::repositories::session_repository::SessionRepositoryTrait;
use gittype::Result;

pub struct MockSessionRepository {}

impl MockSessionRepository {
    pub fn new() -> Self {
        MockSessionRepository {}
    }
}

impl SessionRepositoryTrait for MockSessionRepository {
    fn get_session_stage_results(&self, _session_id: i64) -> Result<Vec<SessionStageResult>> {
        Ok(vec![
            SessionStageResult {
                stage_number: 1,
                wpm: 75.0,
                cpm: 375.0,
                accuracy: 96.7,
                keystrokes: 150,
                mistakes: 5,
                duration_ms: 20000,
                score: 400.0,
                language: Some("Rust".to_string()),
                difficulty_level: Some("Normal".to_string()),
                rank_name: Some("Advanced".to_string()),
                tier_name: Some("Gold".to_string()),
                rank_position: 5,
                rank_total: 100,
                position: 5,
                total: 100,
                was_skipped: false,
                was_failed: false,
                file_path: Some("src/main.rs".to_string()),
                start_line: Some(1),
                end_line: Some(20),
                code_content: Some("fn main() { ... }".to_string()),
            },
            SessionStageResult {
                stage_number: 2,
                wpm: 72.0,
                cpm: 360.0,
                accuracy: 95.4,
                keystrokes: 175,
                mistakes: 8,
                duration_ms: 22000,
                score: 380.0,
                language: Some("Rust".to_string()),
                difficulty_level: Some("Normal".to_string()),
                rank_name: Some("Intermediate".to_string()),
                tier_name: Some("Silver".to_string()),
                rank_position: 15,
                rank_total: 100,
                position: 15,
                total: 100,
                was_skipped: false,
                was_failed: false,
                file_path: Some("src/lib.rs".to_string()),
                start_line: Some(10),
                end_line: Some(30),
                code_content: Some("pub fn test() { ... }".to_string()),
            },
            SessionStageResult {
                stage_number: 3,
                wpm: 78.0,
                cpm: 390.0,
                accuracy: 96.0,
                keystrokes: 175,
                mistakes: 7,
                duration_ms: 18000,
                score: 420.0,
                language: Some("Rust".to_string()),
                difficulty_level: Some("Normal".to_string()),
                rank_name: Some("Expert".to_string()),
                tier_name: Some("Platinum".to_string()),
                rank_position: 3,
                rank_total: 100,
                position: 3,
                total: 100,
                was_skipped: false,
                was_failed: false,
                file_path: Some("src/domain/mod.rs".to_string()),
                start_line: Some(5),
                end_line: Some(25),
                code_content: Some("pub mod models;".to_string()),
            },
        ])
    }

    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        Ok(vec![])
    }

    fn get_sessions_filtered(
        &self,
        _repository_filter: Option<i64>,
        _date_filter_days: Option<i64>,
        _sort_by: &str,
        _sort_descending: bool,
    ) -> Result<Vec<StoredSession>> {
        Ok(vec![])
    }

    fn get_session_result(
        &self,
        _session_id: i64,
    ) -> Result<Option<gittype::domain::models::storage::SessionResultData>> {
        Ok(None)
    }
}
