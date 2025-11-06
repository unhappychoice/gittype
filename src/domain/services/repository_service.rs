use crate::domain::error::Result;
use crate::domain::models::storage::repository::{StoredRepository, StoredRepositoryWithLanguages};
use crate::infrastructure::database::daos::RepositoryDaoInterface;
use crate::infrastructure::git::remote::remote_git_repository_client::RemoteGitRepositoryClient;
use crate::infrastructure::storage::app_data_provider::AppDataProvider;
use shaku::Interface;
use std::path::PathBuf;
use std::sync::Arc;

pub trait RepositoryServiceInterface: Interface {
    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>>;
    fn get_all_repositories_with_languages(&self) -> Result<Vec<StoredRepositoryWithLanguages>>;
    fn get_all_repositories_with_cache_status(
        &self,
    ) -> Result<Vec<(StoredRepositoryWithLanguages, bool)>>;
}

#[derive(shaku::Component)]
#[shaku(interface = RepositoryServiceInterface)]
pub struct RepositoryService {
    #[shaku(inject)]
    repository_dao: Arc<dyn RepositoryDaoInterface>,
    #[shaku(default)]
    remote_git_client: RemoteGitRepositoryClient,
}

impl RepositoryService {
    pub fn new(
        repository_dao: Arc<dyn RepositoryDaoInterface>,
        remote_git_client: RemoteGitRepositoryClient,
    ) -> Self {
        Self {
            repository_dao,
            remote_git_client,
        }
    }
}

impl RepositoryServiceInterface for RepositoryService {
    fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        self.repository_dao.get_all_repositories()
    }

    fn get_all_repositories_with_languages(&self) -> Result<Vec<StoredRepositoryWithLanguages>> {
        self.repository_dao.get_all_repositories_with_languages()
    }

    fn get_all_repositories_with_cache_status(
        &self,
    ) -> Result<Vec<(StoredRepositoryWithLanguages, bool)>> {
        let repositories = self.get_all_repositories_with_languages()?;

        let repositories_with_cache = repositories
            .into_iter()
            .map(|repo| {
                let is_cached = self
                    .remote_git_client
                    .is_repository_cached(&repo.remote_url);
                (repo, is_cached)
            })
            .collect();

        Ok(repositories_with_cache)
    }
}

impl AppDataProvider for RepositoryService {}

impl RepositoryService {
    pub fn get_cache_directory() -> PathBuf {
        Self::get_app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("repos")
    }
}
