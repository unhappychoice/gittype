use crate::domain::models::storage::{
    SaveStageParams, SessionResultData, SessionStageResult, StoredRepository, StoredSession,
};
use crate::domain::models::{Challenge, GitRepository, SessionResult};
use crate::domain::services::scoring::{StageCalculator, StageResult, StageTracker};
use crate::infrastructure::database::daos::{ChallengeDao, RepositoryDao, SessionDao};
use crate::infrastructure::database::database::{Database, HasDatabase};
use crate::{domain::error::GitTypeError, Result};
use std::sync::{Arc, Mutex};

type StageResultTuple = (String, StageResult, usize, Option<Challenge>);

pub trait SessionRepositoryTrait: Send {
    fn get_session_stage_results(&self, session_id: i64) -> Result<Vec<SessionStageResult>>;
}

/// Repository for session business logic
pub struct SessionRepository {
    database: Arc<Mutex<Database>>,
}

impl SessionRepository {
    pub fn new() -> Result<Self> {
        let database = Database::new()?;
        Ok(Self {
            database: Arc::new(Mutex::new(database)),
        })
    }

    /// Record a completed session to the database
    pub fn record_session(
        &self,
        session_result: &SessionResult,
        git_repository: Option<&GitRepository>,
        game_mode: &str,
        difficulty_level: Option<&str>,
        stage_trackers: &[(String, StageTracker)],
        challenges: &[Challenge],
    ) -> Result<i64> {
        log::debug!("Starting session recording...");
        let db = self.db_with_lock()?;

        // Repository manages the transaction boundary
        let conn = db.get_connection();
        let tx = conn.unchecked_transaction()?;
        log::debug!("Database transaction started");

        // Create DAOs
        let repository_dao = RepositoryDao::new(&db);
        let session_dao = SessionDao::new(&db);
        let challenge_dao = ChallengeDao::new(&db);

        // 1. Get or create repository
        let repository_id = if let Some(repo) = git_repository {
            log::debug!(
                "Recording session for repository: {}/{}",
                repo.user_name,
                repo.repository_name
            );
            Some(repository_dao.ensure_repository_in_transaction(&tx, repo)?)
        } else {
            log::debug!("Recording session without repository");
            None
        };
        log::debug!("Repository ID: {:?}", repository_id);

        // 2. Create session record
        let session_id = session_dao.create_session_in_transaction(
            &tx,
            repository_id,
            session_result,
            git_repository,
            game_mode,
            difficulty_level,
        )?;

        // 3. Save session result
        session_dao.save_session_result_in_transaction(
            &tx,
            session_id,
            repository_id,
            session_result,
            stage_trackers,
            game_mode,
            difficulty_level,
        )?;

        // 4. Convert stage trackers to stage results
        let stage_results: Result<Vec<StageResultTuple>> = stage_trackers
            .iter()
            .enumerate()
            .map(|(index, (name, tracker))| {
                let stage_result = StageCalculator::calculate(tracker);
                let keystrokes = tracker.get_data().keystrokes.len();
                let challenge = challenges.get(index).cloned();
                Ok((name.clone(), stage_result, keystrokes, challenge))
            })
            .collect();
        let stage_results = stage_results?;

        // 5. Save stage results
        for (stage_index, (stage_name, stage_result, keystrokes, challenge)) in
            stage_results.into_iter().enumerate()
        {
            // Ensure challenge exists if provided
            let _challenge_id = if let Some(challenge) = &challenge {
                Some(challenge_dao.ensure_challenge_in_transaction(&tx, challenge)?)
            } else {
                None
            };

            session_dao.save_stage_result_in_transaction(
                &tx,
                SaveStageParams {
                    session_id,
                    repository_id,
                    stage_index,
                    stage_name: &stage_name,
                    stage_result: &stage_result,
                    keystrokes,
                    challenge: challenge.as_ref(),
                },
            )?;
        }

        // Repository commits the transaction
        log::debug!("Committing transaction for session ID: {}", session_id);
        tx.commit()?;
        log::debug!("Transaction committed successfully");
        Ok(session_id)
    }

    /// Get session history for a specific repository
    pub fn get_repository_history(&self, repository_id: i64) -> Result<Vec<StoredSession>> {
        let db = self.db_with_lock()?;

        let dao = SessionDao::new(&db);
        dao.get_repository_sessions(repository_id)
    }

    /// Get all repositories
    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        let db = self.db_with_lock()?;

        let dao = RepositoryDao::new(&db);
        dao.get_all_repositories()
    }

    /// Get language performance statistics
    pub fn get_language_stats(&self, _days: Option<i64>) -> Result<Vec<(String, f64, usize)>> {
        let db = self.db_with_lock()?;
        let conn = db.get_connection();

        // Query with proper time filtering for last 30 days
        let query = "SELECT language, AVG(cpm) as avg_cpm, COUNT(*) as session_count
                     FROM stage_results 
                     WHERE language IS NOT NULL 
                     AND language != ''
                     AND cpm > 0
                     AND completed_at >= datetime('now', '-7 days')
                     GROUP BY language 
                     ORDER BY avg_cpm DESC";

        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            let language: String = row.get(0)?;
            let avg_cpm: f64 = row.get(1)?;
            let session_count: i64 = row.get(2)?;
            Ok((language, avg_cpm, session_count as usize))
        })?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }

        Ok(results)
    }

    /// Get filtered and sorted sessions for history display
    pub fn get_sessions_filtered(
        &self,
        repository_filter: Option<i64>,
        date_filter_days: Option<i64>,
        sort_by: &str,
        sort_descending: bool,
    ) -> Result<Vec<StoredSession>> {
        let db = self.db_with_lock()?;
        let dao = SessionDao::new(&db);
        dao.get_sessions_filtered(
            repository_filter,
            date_filter_days,
            sort_by,
            sort_descending,
        )
    }

    /// Get stage results for a specific session
    pub fn get_session_stage_results(&self, session_id: i64) -> Result<Vec<SessionStageResult>> {
        let db = self.db_with_lock()?;
        let dao = SessionDao::new(&db);
        dao.get_session_stage_results(session_id)
    }

    /// Create a global singleton instance
    pub fn global() -> &'static Arc<Mutex<Option<SessionRepository>>> {
        static INSTANCE: std::sync::OnceLock<Arc<Mutex<Option<SessionRepository>>>> =
            std::sync::OnceLock::new();

        INSTANCE.get_or_init(|| Arc::new(Mutex::new(None)))
    }

    /// Initialize the global session service
    pub fn initialize_global() -> Result<()> {
        let repository = Self::new()?;
        let global = Self::global();
        let mut guard = global
            .lock()
            .map_err(|e| GitTypeError::database_error(format!("Failed to acquire lock: {}", e)))?;
        *guard = Some(repository);
        Ok(())
    }

    /// Record session using the global instance
    pub fn record_session_global(
        session_result: &SessionResult,
        git_repository: Option<&GitRepository>,
        game_mode: &str,
        difficulty_level: Option<&str>,
        stage_trackers: &[(String, StageTracker)],
        challenges: &[Challenge],
    ) -> Result<()> {
        let global = Self::global();
        let guard = global
            .lock()
            .map_err(|e| GitTypeError::database_error(format!("Failed to acquire lock: {}", e)))?;

        if let Some(service) = guard.as_ref() {
            match service.record_session(
                session_result,
                git_repository,
                game_mode,
                difficulty_level,
                stage_trackers,
                challenges,
            ) {
                Ok(session_id) => {
                    log::info!("Successfully recorded session with ID: {}", session_id);
                }
                Err(e) => {
                    log::error!("Failed to record session to database: {}", e);
                    return Err(e);
                }
            }
        } else {
            log::warn!("Session service not initialized, skipping database recording");
        }

        Ok(())
    }

    /// Get best records for comparison display
    pub fn get_best_records(&self) -> Result<BestRecords> {
        let db = self.db_with_lock()?;
        let dao = SessionDao::new(&db);

        let todays_best = dao.get_todays_best_session()?;
        let weekly_best = dao.get_weekly_best_session()?;
        let all_time_best = dao.get_all_time_best_session()?;

        let todays_best_data = if let Some(ref session) = todays_best {
            dao.get_session_result(session.id)?
        } else {
            None
        };

        let weekly_best_data = if let Some(ref session) = weekly_best {
            dao.get_session_result(session.id)?
        } else {
            None
        };

        let all_time_best_data = if let Some(ref session) = all_time_best {
            dao.get_session_result(session.id)?
        } else {
            None
        };

        Ok(BestRecords {
            todays_best: todays_best_data,
            weekly_best: weekly_best_data,
            all_time_best: all_time_best_data,
        })
    }

    /// Get best records using the global instance
    pub fn get_best_records_global() -> Result<Option<BestRecords>> {
        let global = Self::global();
        let guard = global
            .lock()
            .map_err(|e| GitTypeError::database_error(format!("Failed to acquire lock: {}", e)))?;

        if let Some(service) = guard.as_ref() {
            service.get_best_records().map(Some)
        } else {
            Ok(None)
        }
    }

    /// Determine best status for a session using session start records
    pub fn determine_best_status_with_start_records(
        session_score: f64,
        best_records_at_start: Option<&BestRecords>,
    ) -> BestStatus {
        let mut best_status = BestStatus::new();

        log::debug!("determine_best_status_with_start_records: session_score={}, best_records_at_start={:?}", 
                   session_score, best_records_at_start);

        if let Some(best_records) = best_records_at_start {
            // Store all current best scores from session start
            best_status.todays_best_score = best_records
                .todays_best
                .as_ref()
                .map(|t| t.score)
                .unwrap_or(0.0);
            best_status.weekly_best_score = best_records
                .weekly_best
                .as_ref()
                .map(|w| w.score)
                .unwrap_or(0.0);
            best_status.all_time_best_score = best_records
                .all_time_best
                .as_ref()
                .map(|a| a.score)
                .unwrap_or(0.0);

            log::debug!(
                "Best scores from start: today={}, weekly={}, all_time={}",
                best_status.todays_best_score,
                best_status.weekly_best_score,
                best_status.all_time_best_score
            );

            // Check each best type independently
            // Check all-time best
            if let Some(ref all_time) = best_records.all_time_best {
                if session_score >= all_time.score {
                    best_status.is_all_time_best = true;
                    best_status.best_type = Some("ALL TIME".to_string());
                }
            }

            // Check weekly best
            if let Some(ref weekly) = best_records.weekly_best {
                if session_score >= weekly.score {
                    best_status.is_weekly_best = true;
                    // Only set as best_type if not already all-time best
                    if best_status.best_type.is_none() {
                        best_status.best_type = Some("WEEKLY".to_string());
                    }
                }
            }

            // Check today's best
            if let Some(ref today) = best_records.todays_best {
                if session_score >= today.score {
                    best_status.is_todays_best = true;
                    // Only set as best_type if not already all-time or weekly best
                    if best_status.best_type.is_none() {
                        best_status.best_type = Some("TODAY'S".to_string());
                    }
                }
            } else {
                // No today's best record exists, this is automatically today's best
                best_status.is_todays_best = true;
                if best_status.best_type.is_none() {
                    best_status.best_type = Some("TODAY'S".to_string());
                }
            }

            // If no previous records exist at all, this is automatically today's best
            if best_records.all_time_best.is_none()
                && best_records.weekly_best.is_none()
                && best_records.todays_best.is_none()
            {
                best_status.is_todays_best = true;
                best_status.best_type = Some("TODAY'S".to_string());
            }
        } else {
            // No records available, this is automatically today's best
            best_status.is_todays_best = true;
            best_status.best_type = Some("TODAY'S".to_string());
        }

        log::debug!("Final best_status: {:?}", best_status);
        best_status
    }

    /// Determine best status for a session before saving it to database
    pub fn determine_best_status_instance(&self, session_score: f64) -> Result<BestStatus> {
        // Get current best records before the new session is saved
        if let Ok(best_records) = self.get_best_records() {
            Ok(Self::determine_best_status_with_start_records(
                session_score,
                Some(&best_records),
            ))
        } else {
            Ok(Self::determine_best_status_with_start_records(
                session_score,
                None,
            ))
        }
    }

    /// Determine best status using global instance
    pub fn determine_best_status_global(session_score: f64) -> Result<Option<BestStatus>> {
        let global = Self::global();
        let guard = global
            .lock()
            .map_err(|e| GitTypeError::database_error(format!("Failed to acquire lock: {}", e)))?;

        if let Some(service) = guard.as_ref() {
            service
                .determine_best_status_instance(session_score)
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// Get session result data for analytics
    pub fn get_session_result_for_analytics(
        &self,
        session_id: i64,
    ) -> Result<Option<SessionResultData>> {
        let db = self.db_with_lock()?;
        let dao = SessionDao::new(&db);
        dao.get_session_result(session_id)
    }
}

#[derive(Debug, Clone)]
pub struct BestRecords {
    pub todays_best: Option<SessionResultData>,
    pub weekly_best: Option<SessionResultData>,
    pub all_time_best: Option<SessionResultData>,
}

#[derive(Debug, Clone)]
pub struct BestStatus {
    pub is_todays_best: bool,
    pub is_weekly_best: bool,
    pub is_all_time_best: bool,
    pub best_type: Option<String>,
    pub todays_best_score: f64,
    pub weekly_best_score: f64,
    pub all_time_best_score: f64,
}

impl Default for BestStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl BestStatus {
    pub fn new() -> Self {
        Self {
            is_todays_best: false,
            is_weekly_best: false,
            is_all_time_best: false,
            best_type: None,
            todays_best_score: 0.0,
            weekly_best_score: 0.0,
            all_time_best_score: 0.0,
        }
    }
}

impl SessionRepositoryTrait for SessionRepository {
    fn get_session_stage_results(&self, session_id: i64) -> Result<Vec<SessionStageResult>> {
        SessionRepository::get_session_stage_results(self, session_id)
    }
}

impl HasDatabase for SessionRepository {
    fn database(&self) -> &Arc<Mutex<Database>> {
        &self.database
    }
}

impl Default for SessionRepository {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            log::warn!("Failed to initialize SessionRepository: {}", e);
            // Return a dummy repository that will fail gracefully
            Self {
                database: Arc::new(Mutex::new(
                    Database::new().expect("Failed to create fallback database"),
                )),
            }
        })
    }
}
