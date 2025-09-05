use super::super::{
    daos::{
        ChallengeDao, RepositoryDao, SaveStageParams, SessionDao, StoredRepository, StoredSession,
    },
    Database, HasDatabase,
};
use crate::models::{Challenge, GitRepository, SessionResult};
use crate::scoring::{StageResult, StageTracker};
use crate::storage::daos::session_dao::SessionResultData;
use crate::{error::GitTypeError, Result};
use std::sync::{Arc, Mutex};

type StageResultTuple = (String, StageResult, usize, Option<Challenge>);

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
        stage_engines: &[(String, StageTracker)],
        challenges: &[Challenge],
    ) -> Result<i64> {
        let db = self.db_with_lock()?;

        // Repository manages the transaction boundary
        let conn = db.get_connection();
        let tx = conn.unchecked_transaction()?;

        // Create DAOs
        let repository_dao = RepositoryDao::new(&db);
        let session_dao = SessionDao::new(&db);
        let challenge_dao = ChallengeDao::new(&db);

        // 1. Get or create repository
        let repository_id = if let Some(repo) = git_repository {
            Some(repository_dao.ensure_repository_in_transaction(&tx, repo)?)
        } else {
            None
        };

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
            stage_engines,
            game_mode,
            difficulty_level,
        )?;

        // 4. Convert stage engines to stage results
        let stage_results: Result<Vec<StageResultTuple>> = stage_engines
            .iter()
            .enumerate()
            .map(|(index, (name, engine))| {
                let stage_result = crate::scoring::StageCalculator::calculate(engine, false, false);
                let keystrokes = engine.get_data().keystrokes.len();
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
        tx.commit()?;
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
        stage_engines: &[(String, StageTracker)],
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
                stage_engines,
                challenges,
            ) {
                Ok(session_id) => {
                    log::debug!("Successfully recorded session with ID: {}", session_id);
                }
                Err(e) => {
                    log::warn!("Failed to record session to database: {}", e);
                    return Err(e);
                }
            }
        } else {
            log::debug!("Session service not initialized, skipping database recording");
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
}

#[derive(Debug, Clone)]
pub struct BestRecords {
    pub todays_best: Option<SessionResultData>,
    pub weekly_best: Option<SessionResultData>,
    pub all_time_best: Option<SessionResultData>,
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
