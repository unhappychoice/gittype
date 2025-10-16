use crate::domain::error::Result;
use crate::domain::models::storage::{SessionResultData, StoredRepository, StoredSession};
use crate::domain::repositories::SessionRepository;
use crate::infrastructure::database::daos::SessionDao;
use crate::infrastructure::database::database::{Database, HasDatabase};
use std::collections::HashMap;

#[derive(Clone)]
pub struct SessionDisplayData {
    pub session: StoredSession,
    pub repository: Option<StoredRepository>,
    pub session_result: Option<SessionResultData>,
}

pub struct SessionService {
    repository: SessionRepository,
}

impl SessionService {
    pub fn new(repository: SessionRepository) -> Self {
        Self { repository }
    }

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
            let session_result = {
                let db = self.repository.db_with_lock()?;
                self.get_session_result(&db, session.id)?
            };

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

    fn get_session_result(
        &self,
        db: &Database,
        session_id: i64,
    ) -> Result<Option<SessionResultData>> {
        let dao = SessionDao::new(db);
        dao.get_session_result(session_id)
    }

    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        self.repository.get_all_repositories()
    }
}
