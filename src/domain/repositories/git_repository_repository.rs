use crate::domain::models::storage::StoredRepository;
use crate::domain::models::GitRepository;
use crate::infrastructure::database::daos::RepositoryDao;
use crate::infrastructure::database::database::{Database, HasDatabase};
use crate::Result;
use std::sync::{Arc, Mutex};

/// Repository for Git repository business logic
pub struct GitRepositoryRepository {
    database: Arc<Mutex<Database>>,
}

impl GitRepositoryRepository {
    pub fn new() -> Result<Self> {
        let database = Database::new()?;
        #[cfg(feature = "test-mocks")]
        database.init()?;
        Ok(Self {
            database: Arc::new(Mutex::new(database)),
        })
    }

    /// Get or create a repository record
    pub fn ensure_repository(&self, git_repo: &GitRepository) -> Result<i64> {
        let db = self.db_with_lock()?;

        let dao = RepositoryDao::new(&db);
        dao.ensure_repository(git_repo)
    }

    /// Get all repositories
    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        let db = self.db_with_lock()?;

        let dao = RepositoryDao::new(&db);
        dao.get_all_repositories()
    }

    /// Get a repository by ID
    pub fn get_repository_by_id(&self, repository_id: i64) -> Result<Option<StoredRepository>> {
        let db = self.db_with_lock()?;

        let dao = RepositoryDao::new(&db);
        dao.get_repository_by_id(repository_id)
    }

    /// Find repository by user and repository name
    pub fn find_repository(
        &self,
        user_name: &str,
        repository_name: &str,
    ) -> Result<Option<StoredRepository>> {
        let db = self.db_with_lock()?;

        let dao = RepositoryDao::new(&db);
        dao.find_repository(user_name, repository_name)
    }

    /// Get repositories for a specific user
    pub fn get_user_repositories(&self, user_name: &str) -> Result<Vec<StoredRepository>> {
        let repositories = self.get_all_repositories()?;
        let user_repos = repositories
            .into_iter()
            .filter(|repo| repo.user_name == user_name)
            .collect();
        Ok(user_repos)
    }
}

impl HasDatabase for GitRepositoryRepository {
    fn database(&self) -> &Arc<Mutex<Database>> {
        &self.database
    }
}

impl Default for GitRepositoryRepository {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            log::warn!("Failed to initialize RepositoryRepository: {}", e);
            // Return a dummy repository that will fail gracefully
            Self {
                database: Arc::new(Mutex::new(
                    Database::new().expect("Failed to create fallback database"),
                )),
            }
        })
    }
}
