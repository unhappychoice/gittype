use super::super::Database;
use crate::models::{Challenge, GitRepository, SessionResult, StageResult};
use crate::{error::GitTypeError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SaveStageParams<'a> {
    pub session_id: i64,
    pub repository_id: Option<i64>,
    pub stage_index: usize,
    pub stage_name: &'a str,
    pub stage_result: &'a StageResult,
    pub keystrokes: usize,
    pub challenge: Option<&'a Challenge>,
}

pub struct SessionDao<'a> {
    db: &'a Database,
}

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

impl<'a> SessionDao<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Create session record within an existing transaction
    pub fn create_session_in_transaction(
        &self,
        tx: &Transaction,
        repository_id: Option<i64>,
        _session_result: &SessionResult,
        git_repo: Option<&GitRepository>,
        game_mode: &str,
        difficulty_level: Option<&str>,
    ) -> Result<i64> {
        let started_at = Self::system_time_to_sqlite_timestamp(SystemTime::now()); // Use current time
        let completed_at = Some(Self::system_time_to_sqlite_timestamp(SystemTime::now())); // Mark as completed now

        tx.execute(
            "INSERT INTO sessions (
                repository_id, started_at, completed_at, branch, commit_hash, is_dirty,
                game_mode, difficulty_level, max_stages, time_limit_seconds
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                repository_id,
                started_at,
                completed_at,
                git_repo.and_then(|r| r.branch.as_ref()),
                git_repo.and_then(|r| r.commit_hash.as_ref()),
                git_repo.map(|r| r.is_dirty).unwrap_or(false),
                game_mode,
                difficulty_level,
                None::<i32>, // max_stages - not available in SessionResult
                None::<i32>  // time_limit_seconds - not available in SessionResult
            ],
        )?;

        Ok(tx.last_insert_rowid())
    }

    /// Save session result within an existing transaction
    pub fn save_session_result_in_transaction(
        &self,
        tx: &Transaction,
        session_id: i64,
        repository_id: Option<i64>,
        session_result: &SessionResult,
        game_mode: &str,
        difficulty_level: Option<&str>,
    ) -> Result<()> {
        tx.execute(
            "INSERT INTO session_results (
                session_id, repository_id, keystrokes, mistakes, duration_ms,
                wpm, cpm, accuracy, stages_completed, stages_attempted, stages_skipped,
                partial_effort_keystrokes, partial_effort_mistakes,
                best_stage_wpm, worst_stage_wpm, best_stage_accuracy, worst_stage_accuracy,
                score, game_mode, difficulty_level
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                session_id,
                repository_id.unwrap_or(0), // repository_id is NOT NULL, use 0 as default
                session_result.total_keystrokes as i64,
                session_result.total_mistakes as i64,
                session_result.session_duration.as_millis() as i64,
                session_result.overall_wpm,
                session_result.overall_cpm,
                session_result.overall_accuracy,
                session_result.stages_completed as i64,
                session_result.stages_attempted as i64,
                session_result.stages_skipped as i64,
                session_result.total_partial_effort_keystrokes as i64,
                session_result.total_partial_effort_mistakes as i64,
                session_result.best_stage_wpm,
                session_result.worst_stage_wpm,
                session_result.best_stage_accuracy,
                session_result.worst_stage_accuracy,
                session_result.session_score,
                game_mode,
                difficulty_level
            ],
        )?;
        Ok(())
    }

    /// Save stage result within an existing transaction  
    pub fn save_stage_result_in_transaction(
        &self,
        tx: &Transaction,
        params: SaveStageParams,
    ) -> Result<()> {
        // Create a dummy stage record first
        tx.execute(
            "INSERT INTO stages (session_id, challenge_id, stage_number, started_at, completed_at)
             VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![
                params.session_id,
                params.challenge.map(|c| c.id.as_str()).unwrap_or("dummy"), // challenge_id is TEXT and NOT NULL
                (params.stage_index + 1) as i64, // stage_number - 1-based index
                Self::system_time_to_sqlite_timestamp(SystemTime::now()),
                Self::system_time_to_sqlite_timestamp(SystemTime::now())
            ],
        )?;
        let stage_id = tx.last_insert_rowid();

        // Now insert the stage result
        tx.execute(
            "INSERT INTO stage_results (
                stage_id, session_id, repository_id, keystrokes, mistakes, duration_ms, 
                wpm, cpm, accuracy, consistency_streaks, score, rank_name, tier_name, 
                rank_position, rank_total, position, total,
                was_skipped, was_failed, completed_at, language, difficulty_level
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                stage_id,
                params.session_id,
                params.repository_id.unwrap_or(0), // repository_id is NOT NULL in schema
                params.keystrokes as i64,
                params.stage_result.mistakes as i64,
                params.stage_result.completion_time.as_millis() as i64,
                params.stage_result.wpm,
                params.stage_result.cpm,
                params.stage_result.accuracy,
                serde_json::to_string(&params.stage_result.consistency_streaks).unwrap_or_default(),
                params.stage_result.challenge_score,
                params.stage_result.rank_name,
                params.stage_result.tier_name,
                params.stage_result.tier_position as i64,
                params.stage_result.tier_total as i64,
                params.stage_result.overall_position as i64,
                params.stage_result.overall_total as i64,
                params.stage_result.was_skipped,
                params.stage_result.was_failed,
                Self::system_time_to_sqlite_timestamp(SystemTime::now()),
                params.challenge.and_then(|c| c.language.clone()),
                params
                    .challenge
                    .and_then(|c| c.difficulty_level.as_ref().map(|d| format!("{:?}", d)))
            ],
        )?;

        Ok(())
    }

    /// Get session history for a repository
    pub fn get_repository_sessions(&self, repository_id: i64) -> Result<Vec<StoredSession>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, repository_id, started_at, completed_at, branch, commit_hash,
                    is_dirty, game_mode, difficulty_level, max_stages, time_limit_seconds
             FROM sessions 
             WHERE repository_id = ? 
             ORDER BY started_at DESC",
        )?;

        let sessions = stmt
            .query_map(params![repository_id], |row| {
                let started_at_str: String = row.get(2)?;
                let started_at = Self::parse_sqlite_timestamp(&started_at_str)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                let completed_at = row
                    .get::<_, Option<String>>(3)?
                    .map(|s| {
                        Self::parse_sqlite_timestamp(&s)
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
                    })
                    .transpose()?;

                Ok(StoredSession {
                    id: row.get(0)?,
                    repository_id: row.get(1)?,
                    started_at,
                    completed_at,
                    branch: row.get(4)?,
                    commit_hash: row.get(5)?,
                    is_dirty: row.get(6)?,
                    game_mode: row.get(7)?,
                    difficulty_level: row.get(8)?,
                    max_stages: row.get(9)?,
                    time_limit_seconds: row.get(10)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    /// Convert SystemTime to SQLite timestamp string
    fn system_time_to_sqlite_timestamp(time: SystemTime) -> String {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        let datetime = DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0).unwrap();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Parse SQLite timestamp string to DateTime<Utc>
    fn parse_sqlite_timestamp(s: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_str(&format!("{} +0000", s), "%Y-%m-%d %H:%M:%S %z")
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| GitTypeError::database_error(format!("Failed to parse timestamp: {}", e)))
    }
}
