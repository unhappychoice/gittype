use std::collections::HashMap;
use std::sync::Arc;

use crate::domain::error::Result;
use crate::domain::models::storage::{SessionResultData, StoredRepository, StoredSession};
use crate::domain::repositories::session_repository::SessionRepositoryTrait;

#[derive(Clone)]
pub struct SessionDisplayData {
    pub session: StoredSession,
    pub repository: Option<StoredRepository>,
    pub session_result: Option<SessionResultData>,
}

impl Default for SessionDisplayData {
    fn default() -> Self {
        Self {
            session: StoredSession {
                id: 0,
                repository_id: None,
                started_at: chrono::Utc::now(),
                completed_at: Some(chrono::Utc::now()),
                branch: None,
                commit_hash: None,
                is_dirty: false,
                game_mode: "default".to_string(),
                difficulty_level: None,
                max_stages: None,
                time_limit_seconds: None,
            },
            repository: None,
            session_result: None,
        }
    }
}

pub trait SessionServiceInterface: shaku::Interface {
    fn get_sessions_with_display_data(
        &self,
        repository_filter: Option<i64>,
        date_filter_days: Option<i64>,
        sort_by: &str,
        sort_descending: bool,
    ) -> Result<Vec<SessionDisplayData>>;
    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>>;
}

#[derive(shaku::Component)]
#[shaku(interface = SessionServiceInterface)]
pub struct SessionService {
    #[shaku(inject)]
    repository: Arc<dyn SessionRepositoryTrait>,
}

impl SessionServiceInterface for SessionService {
    fn get_sessions_with_display_data(
        &self,
        repository_filter: Option<i64>,
        date_filter_days: Option<i64>,
        sort_by: &str,
        sort_descending: bool,
    ) -> Result<Vec<SessionDisplayData>> {
        SessionService::get_sessions_with_display_data(
            self,
            repository_filter,
            date_filter_days,
            sort_by,
            sort_descending,
        )
    }

    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        SessionService::get_all_repositories(self)
    }
}

impl SessionService {
    pub fn get_sessions_with_display_data(
        &self,
        repository_filter: Option<i64>,
        date_filter_days: Option<i64>,
        sort_by: &str,
        sort_descending: bool,
    ) -> Result<Vec<SessionDisplayData>> {
        let repositories = self.repository.get_all_repositories()?;
        let sessions = self.repository.get_sessions_filtered(
            repository_filter,
            date_filter_days,
            sort_by,
            sort_descending,
        )?;

        let repository_map: HashMap<i64, StoredRepository> = repositories
            .iter()
            .map(|repo| (repo.id, repo.clone()))
            .collect();

        let mut session_display_data = Vec::new();

        for session in sessions {
            let session_result = self.repository.get_session_result(session.id)?;

            let repository = session
                .repository_id
                .and_then(|id| repository_map.get(&id).cloned());

            session_display_data.push(SessionDisplayData {
                session,
                repository,
                session_result,
            });
        }

        Ok(session_display_data)
    }

    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        self.repository.get_all_repositories()
    }

    /// Create a new SessionService instance. This method is primarily for testing.
    /// In production code, use the DI container to resolve SessionService.
    pub fn new(repository: crate::domain::repositories::SessionRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }
}
