use super::super::database::{Database, DatabaseInterface};
use crate::domain::models::storage::{
    SaveStageParams, SessionResultData, SessionStageResult, StoredSession,
};
use crate::domain::models::{GitRepository, SessionResult};
use crate::domain::services::scoring::RankCalculator;
use crate::{domain::error::GitTypeError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, OptionalExtension, Transaction};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SessionDao {
    db: Arc<dyn DatabaseInterface>,
}

impl SessionDao {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
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
    #[allow(clippy::too_many_arguments)]
    pub fn save_session_result_in_transaction(
        &self,
        tx: &Transaction,
        session_id: i64,
        repository_id: Option<i64>,
        session_result: &SessionResult,
        _stage_engines: &[(String, crate::domain::services::scoring::StageTracker)],
        game_mode: &str,
        difficulty_level: Option<&str>,
    ) -> Result<()> {
        // Calculate tier and rank from session score
        let session_rank = crate::domain::models::Rank::for_score(session_result.session_score);
        let tier_name = match session_rank.tier() {
            crate::domain::models::RankTier::Beginner => "Beginner",
            crate::domain::models::RankTier::Intermediate => "Intermediate",
            crate::domain::models::RankTier::Advanced => "Advanced",
            crate::domain::models::RankTier::Expert => "Expert",
            crate::domain::models::RankTier::Legendary => "Legendary",
        };

        // Calculate position information using RankCalculator
        let (_, tier_position, tier_total, overall_position, overall_total) =
            RankCalculator::calculate_tier_info(session_result.session_score);

        tx.execute(
            "INSERT INTO session_results (
                session_id, repository_id, keystrokes, mistakes, duration_ms,
                wpm, cpm, accuracy, stages_completed, stages_attempted, stages_skipped,
                partial_effort_keystrokes, partial_effort_mistakes,
                best_stage_wpm, worst_stage_wpm, best_stage_accuracy, worst_stage_accuracy,
                score, rank_name, tier_name, rank_position, rank_total, position, total,
                game_mode, difficulty_level
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                session_id,
                repository_id.ok_or_else(|| GitTypeError::TerminalError("repository_id is required for session_results".to_string()))?,
                session_result.valid_keystrokes as i64,
                session_result.valid_mistakes as i64,
                session_result.session_duration.as_millis() as i64,
                session_result.overall_wpm,
                session_result.overall_cpm,
                session_result.overall_accuracy,
                session_result.stages_completed as i64,
                session_result.stages_attempted as i64,
                session_result.stages_skipped as i64,
                session_result.invalid_keystrokes as i64,
                session_result.invalid_mistakes as i64,
                session_result.best_stage_wpm,
                session_result.worst_stage_wpm,
                session_result.best_stage_accuracy,
                session_result.worst_stage_accuracy,
                session_result.session_score,
                session_rank.name(),
                tier_name,
                tier_position as i64,
                tier_total as i64,
                overall_position as i64,
                overall_total as i64,
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
                params
                    .repository_id
                    .ok_or_else(|| GitTypeError::TerminalError(
                        "repository_id is required for stage_results".to_string()
                    ))?,
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
        let conn = self.db.get_connection()?;
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

    /// Get best session record from today
    pub fn get_todays_best_session(&self) -> Result<Option<StoredSession>> {
        let conn = self.db.get_connection()?;
        let today = chrono::Utc::now().date_naive().format("%Y-%m-%d");

        let mut stmt = conn.prepare(
            "SELECT s.id, s.repository_id, s.started_at, s.completed_at, s.branch, s.commit_hash,
                    s.is_dirty, s.game_mode, s.difficulty_level, s.max_stages, s.time_limit_seconds
             FROM sessions s 
             JOIN session_results sr ON s.id = sr.session_id
             WHERE DATE(s.started_at) = ?
             ORDER BY sr.score DESC
             LIMIT 1",
        )?;

        let session = stmt
            .query_row(params![today.to_string()], |row| {
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
            })
            .optional()?;

        Ok(session)
    }

    /// Get best session record from past 7 days
    pub fn get_weekly_best_session(&self) -> Result<Option<StoredSession>> {
        let conn = self.db.get_connection()?;
        let week_ago = chrono::Utc::now().date_naive() - chrono::Duration::days(7);

        let mut stmt = conn.prepare(
            "SELECT s.id, s.repository_id, s.started_at, s.completed_at, s.branch, s.commit_hash,
                    s.is_dirty, s.game_mode, s.difficulty_level, s.max_stages, s.time_limit_seconds
             FROM sessions s 
             JOIN session_results sr ON s.id = sr.session_id
             WHERE DATE(s.started_at) >= ?
             ORDER BY sr.score DESC
             LIMIT 1",
        )?;

        let session = stmt
            .query_row(params![week_ago.format("%Y-%m-%d").to_string()], |row| {
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
            })
            .optional()?;

        Ok(session)
    }

    /// Get all-time best session record
    pub fn get_all_time_best_session(&self) -> Result<Option<StoredSession>> {
        let conn = self.db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT s.id, s.repository_id, s.started_at, s.completed_at, s.branch, s.commit_hash,
                    s.is_dirty, s.game_mode, s.difficulty_level, s.max_stages, s.time_limit_seconds
             FROM sessions s 
             JOIN session_results sr ON s.id = sr.session_id
             ORDER BY sr.score DESC
             LIMIT 1",
        )?;

        let session = stmt
            .query_row([], |row| {
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
            })
            .optional()?;

        Ok(session)
    }

    /// Get session result data for a specific session ID
    pub fn get_session_result(&self, session_id: i64) -> Result<Option<SessionResultData>> {
        let conn = self.db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT keystrokes, mistakes, duration_ms, wpm, cpm, accuracy, 
                    stages_completed, stages_attempted, stages_skipped, score,
                    rank_name, tier_name, rank_position, rank_total, position, total
             FROM session_results 
             WHERE session_id = ?",
        )?;

        let result = stmt
            .query_row(params![session_id], |row| {
                Ok(SessionResultData {
                    keystrokes: row.get::<_, i64>(0)? as usize,
                    mistakes: row.get::<_, i64>(1)? as usize,
                    duration_ms: row.get::<_, i64>(2)? as u64,
                    wpm: row.get(3)?,
                    cpm: row.get(4)?,
                    accuracy: row.get(5)?,
                    stages_completed: row.get::<_, i64>(6)? as usize,
                    stages_attempted: row.get::<_, i64>(7)? as usize,
                    stages_skipped: row.get::<_, i64>(8)? as usize,
                    score: row.get(9)?,
                    rank_name: row.get(10)?,
                    tier_name: row.get(11)?,
                    rank_position: row.get(12)?,
                    rank_total: row.get(13)?,
                    position: row.get(14)?,
                    total: row.get(15)?,
                })
            })
            .optional()?;

        Ok(result)
    }

    /// Get sessions with filtering and sorting
    pub fn get_sessions_filtered(
        &self,
        repository_filter: Option<i64>,
        date_filter_days: Option<i64>,
        sort_by: &str,
        sort_descending: bool,
    ) -> Result<Vec<StoredSession>> {
        let conn = self.db.get_connection()?;

        let mut query = String::from(
            "SELECT s.id, s.repository_id, s.started_at, s.completed_at, s.branch, s.commit_hash,
                    s.is_dirty, s.game_mode, s.difficulty_level, s.max_stages, s.time_limit_seconds
             FROM sessions s 
             INNER JOIN session_results sr ON s.id = sr.session_id
             WHERE s.completed_at IS NOT NULL",
        );

        let mut params = Vec::new();

        // Apply repository filter
        if let Some(repo_id) = repository_filter {
            query.push_str(" AND s.repository_id = ?");
            params.push(repo_id.to_string());
        }

        // Apply date filter
        if let Some(days) = date_filter_days {
            let cutoff_date =
                (chrono::Utc::now().date_naive() - chrono::Duration::days(days)).format("%Y-%m-%d");
            query.push_str(" AND DATE(s.started_at) >= ?");
            params.push(cutoff_date.to_string());
        }

        // Add sorting
        let sort_column = match sort_by {
            "date" => "s.started_at",
            "score" => "COALESCE(sr.score, 0)",
            "repository" => "s.repository_id",
            "duration" => "COALESCE(sr.duration_ms, 0)",
            _ => "s.started_at",
        };

        let sort_direction = if sort_descending { "DESC" } else { "ASC" };
        query.push_str(&format!(" ORDER BY {} {}", sort_column, sort_direction));

        let mut stmt = conn.prepare(&query)?;

        let sessions = stmt
            .query_map(
                params
                    .iter()
                    .map(|s| s as &dyn rusqlite::ToSql)
                    .collect::<Vec<_>>()
                    .as_slice(),
                |row| {
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
                },
            )?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    /// Get stage results for a specific session
    pub fn get_session_stage_results(&self, session_id: i64) -> Result<Vec<SessionStageResult>> {
        let conn = self.db.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT sr.wpm, sr.cpm, sr.accuracy, sr.keystrokes, sr.mistakes, sr.duration_ms, 
                    sr.score, sr.language, sr.difficulty_level, sr.rank_name, sr.tier_name,
                    sr.rank_position, sr.rank_total, sr.position, sr.total, sr.was_skipped, sr.was_failed,
                    s.stage_number,
                    c.file_path, c.start_line, c.end_line, c.code_content
             FROM stage_results sr
             JOIN stages s ON sr.stage_id = s.id
             LEFT JOIN challenges c ON s.challenge_id = c.id
             WHERE sr.session_id = ?
             ORDER BY s.stage_number",
        )?;

        let stage_results = stmt
            .query_map(params![session_id], |row| {
                Ok(SessionStageResult {
                    stage_number: row.get(17)?,
                    wpm: row.get(0)?,
                    cpm: row.get(1)?,
                    accuracy: row.get(2)?,
                    keystrokes: row.get::<_, i64>(3)? as usize,
                    mistakes: row.get::<_, i64>(4)? as usize,
                    duration_ms: row.get::<_, i64>(5)? as u64,
                    score: row.get(6)?,
                    language: row.get(7)?,
                    difficulty_level: row.get(8)?,
                    rank_name: row.get(9)?,
                    tier_name: row.get(10)?,
                    rank_position: row.get::<_, i64>(11)? as usize,
                    rank_total: row.get::<_, i64>(12)? as usize,
                    position: row.get::<_, i64>(13)? as usize,
                    total: row.get::<_, i64>(14)? as usize,
                    was_skipped: row.get(15)?,
                    was_failed: row.get(16)?,
                    // Challenge details
                    file_path: row.get(18)?,
                    start_line: row.get(19)?,
                    end_line: row.get(20)?,
                    code_content: row.get(21)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(stage_results)
    }

    /// Parse SQLite timestamp string to DateTime<Utc>
    fn parse_sqlite_timestamp(s: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_str(&format!("{} +0000", s), "%Y-%m-%d %H:%M:%S %z")
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| GitTypeError::database_error(format!("Failed to parse timestamp: {}", e)))
    }
}
