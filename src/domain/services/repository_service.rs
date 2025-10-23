use crate::domain::error::Result;
use crate::domain::models::storage::repository::{StoredRepository, StoredRepositoryWithLanguages};
use crate::infrastructure::database::daos::RepositoryDao;
use crate::infrastructure::database::database::Database;
use crate::infrastructure::git::remote::remote_git_repository_client::RemoteGitRepositoryClient;
use crate::infrastructure::storage::file_storage::FileStorage;
use std::path::PathBuf;

pub struct RepositoryService {
    db: Database,
}

impl RepositoryService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        let repo_dao = RepositoryDao::new(&self.db);
        repo_dao.get_all_repositories()
    }

    pub fn get_all_repositories_with_languages(
        &self,
    ) -> Result<Vec<StoredRepositoryWithLanguages>> {
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
        let file_storage = FileStorage::new();
        file_storage
            .get_app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("repos")
    }
}
