use crate::domain::error::Result;
use crate::domain::models::storage::repository::{StoredRepository, StoredRepositoryWithLanguages};
use crate::infrastructure::git::remote::remote_git_repository_client::RemoteGitRepositoryClient;
use crate::infrastructure::database::daos::RepositoryDao;
use crate::infrastructure::database::database::Database;
use std::path::PathBuf;
use std::sync::Arc;

pub struct RepositoryService {
    db: Arc<Database>,
}

impl RepositoryService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        let repo_dao = RepositoryDao::new(&self.db);
        repo_dao.get_all_repositories()
    }

    pub fn get_all_repositories_with_languages(&self) -> Result<Vec<StoredRepositoryWithLanguages>> {
        let repo_dao = RepositoryDao::new(&self.db);
        repo_dao.get_all_repositories_with_languages()
    }

    pub fn get_all_repositories_with_cache_status(
        &self,
    ) -> Result<Vec<(StoredRepositoryWithLanguages, bool)>> {
        let repositories = self.get_all_repositories_with_languages()?;

        let repositories_with_cache = repositories
            .into_iter()
            .map(|repo| {
                let is_cached = RemoteGitRepositoryClient::is_repository_cached(&repo.remote_url);
                (repo, is_cached)
            })
            .collect();

        Ok(repositories_with_cache)
    }

    pub fn get_cache_directory() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".gittype").join("repos")
    }
}
