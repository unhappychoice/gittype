use crate::domain::models::storage::StoredRepository;
use crate::domain::models::GitRepository;
use crate::infrastructure::database::daos::RepositoryDaoInterface;
use crate::Result;
use shaku::Interface;
use std::sync::Arc;

pub trait GitRepositoryRepositoryInterface: Interface {
    fn ensure_repository(&self, git_repo: &GitRepository) -> Result<i64>;
    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>>;
    fn get_repository_by_id(&self, repository_id: i64) -> Result<Option<StoredRepository>>;
    fn find_repository(
        &self,
        user_name: &str,
        repository_name: &str,
    ) -> Result<Option<StoredRepository>>;
    fn get_user_repositories(&self, user_name: &str) -> Result<Vec<StoredRepository>>;
}

/// Repository for Git repository business logic
#[derive(shaku::Component)]
#[shaku(interface = GitRepositoryRepositoryInterface)]
pub struct GitRepositoryRepository {
    #[shaku(inject)]
    repository_dao: Arc<dyn RepositoryDaoInterface>,
}

impl GitRepositoryRepositoryInterface for GitRepositoryRepository {
    /// Get or create a repository record
    fn ensure_repository(&self, git_repo: &GitRepository) -> Result<i64> {
        self.repository_dao.ensure_repository(git_repo)
    }

    /// Get all repositories
    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        self.repository_dao.get_all_repositories()
    }

    /// Get a repository by ID
    fn get_repository_by_id(&self, repository_id: i64) -> Result<Option<StoredRepository>> {
        self.repository_dao.get_repository_by_id(repository_id)
    }

    /// Find repository by user and repository name
    fn find_repository(
        &self,
        user_name: &str,
        repository_name: &str,
    ) -> Result<Option<StoredRepository>> {
        self.repository_dao
            .find_repository(user_name, repository_name)
    }

    /// Get repositories for a specific user
    fn get_user_repositories(&self, user_name: &str) -> Result<Vec<StoredRepository>> {
        let repositories = self.get_all_repositories()?;
        let user_repos = repositories
            .into_iter()
            .filter(|repo| repo.user_name == user_name)
            .collect();
        Ok(user_repos)
    }
}
