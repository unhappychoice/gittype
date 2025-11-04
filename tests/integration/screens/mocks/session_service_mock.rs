use gittype::domain::models::storage::StoredRepository;
use gittype::domain::services::session_service::{SessionDisplayData, SessionServiceInterface};
use gittype::Result;

pub struct MockSessionService;

impl MockSessionService {
    pub fn new() -> Self {
        MockSessionService
    }
}

impl SessionServiceInterface for MockSessionService {
    fn get_sessions_with_display_data(
        &self,
        _repository_filter: Option<i64>,
        _date_filter_days: Option<i64>,
        _sort_by: &str,
        _sort_descending: bool,
    ) -> Result<Vec<SessionDisplayData>> {
        Ok(vec![])
    }

    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        Ok(vec![])
    }
}
