use shaku::Interface;

use std::path::PathBuf;
use std::sync::RwLock;

use crate::domain::models::{ExtractionOptions, GitRepository};

pub trait RepositoryStoreInterface: Interface {
    fn get_repository(&self) -> Option<GitRepository>;
    fn set_repository(&self, repository: GitRepository);
    fn clear_repository(&self);

    fn get_repo_spec(&self) -> Option<String>;
    fn set_repo_spec(&self, spec: String);

    fn get_repo_path(&self) -> Option<PathBuf>;
    fn set_repo_path(&self, path: PathBuf);

    fn get_extraction_options(&self) -> Option<ExtractionOptions>;
    fn set_extraction_options(&self, options: ExtractionOptions);

    fn clear(&self);
}

#[derive(shaku::Component)]
#[shaku(interface = RepositoryStoreInterface)]
pub struct RepositoryStore {
    #[shaku(default)]
    git_repository: RwLock<Option<GitRepository>>,
    #[shaku(default)]
    repo_spec: RwLock<Option<String>>,
    #[shaku(default)]
    repo_path: RwLock<Option<PathBuf>>,
    #[shaku(default)]
    extraction_options: RwLock<Option<ExtractionOptions>>,
}

impl RepositoryStore {
    #[cfg(feature = "test-mocks")]
    pub fn new_for_test() -> Self {
        Self {
            git_repository: RwLock::new(None),
            repo_spec: RwLock::new(None),
            repo_path: RwLock::new(None),
            extraction_options: RwLock::new(None),
        }
    }
}

impl Default for RepositoryStore {
    fn default() -> Self {
        Self {
            git_repository: RwLock::new(None),
            repo_spec: RwLock::new(None),
            repo_path: RwLock::new(None),
            extraction_options: RwLock::new(None),
        }
    }
}

impl RepositoryStoreInterface for RepositoryStore {
    fn get_repository(&self) -> Option<GitRepository> {
        self.git_repository.read().unwrap().clone()
    }

    fn set_repository(&self, repository: GitRepository) {
        *self.git_repository.write().unwrap() = Some(repository);
    }

    fn clear_repository(&self) {
        *self.git_repository.write().unwrap() = None;
    }

    fn get_repo_spec(&self) -> Option<String> {
        self.repo_spec.read().unwrap().clone()
    }

    fn set_repo_spec(&self, spec: String) {
        *self.repo_spec.write().unwrap() = Some(spec);
    }

    fn get_repo_path(&self) -> Option<PathBuf> {
        self.repo_path.read().unwrap().clone()
    }

    fn set_repo_path(&self, path: PathBuf) {
        *self.repo_path.write().unwrap() = Some(path);
    }

    fn get_extraction_options(&self) -> Option<ExtractionOptions> {
        self.extraction_options.read().unwrap().clone()
    }

    fn set_extraction_options(&self, options: ExtractionOptions) {
        *self.extraction_options.write().unwrap() = Some(options);
    }

    fn clear(&self) {
        *self.git_repository.write().unwrap() = None;
        *self.repo_spec.write().unwrap() = None;
        *self.repo_path.write().unwrap() = None;
        *self.extraction_options.write().unwrap() = None;
    }
}
