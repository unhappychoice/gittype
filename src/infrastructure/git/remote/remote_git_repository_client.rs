use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{Cred, FetchOptions, RemoteCallbacks};
use shaku::{Component, Interface};

use std::cell::RefCell;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::domain::error::Result;
use crate::domain::models::GitRepositoryRef;
use crate::infrastructure::git::git_repository_ref_parser::GitRepositoryRefParser;
use crate::GitTypeError;

pub trait RemoteGitRepositoryClientInterface: Interface {
    fn get_local_repo_path(&self, repo_info: &GitRepositoryRef) -> Result<PathBuf>;
    fn delete_repository(&self, repo_info: &GitRepositoryRef) -> Result<()>;
    fn is_repository_complete(&self, path: &Path) -> bool;
    fn is_repository_cached(&self, remote_url: &str) -> bool;
}

#[derive(Component, Default, Clone)]
#[shaku(interface = RemoteGitRepositoryClientInterface)]
pub struct RemoteGitRepositoryClient;

impl RemoteGitRepositoryClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_local_repo_path(&self, repo_info: &GitRepositoryRef) -> Result<PathBuf> {
        dirs::home_dir()
            .ok_or_else(|| {
                GitTypeError::InvalidRepositoryFormat(
                    "Could not determine home directory".to_string(),
                )
            })
            .map(|home_dir| {
                home_dir
                    .join(".gittype")
                    .join("repos")
                    .join(&repo_info.origin)
                    .join(&repo_info.owner)
                    .join(&repo_info.name)
            })
    }

    pub fn clone_repository<F>(&self, repo_spec: &str, progress_callback: F) -> Result<PathBuf>
    where
        F: FnMut(usize, usize),
    {
        let repo_info = GitRepositoryRefParser::parse(repo_spec)?;

        log::info!("Cloning repository: {}/{}", repo_info.owner, repo_info.name);

        let local_path = self.get_local_repo_path(&repo_info)?;

        if local_path.exists() && self.is_repository_complete(&local_path) {
            return Ok(local_path);
        }

        if local_path.exists() {
            remove_dir_all(&local_path)?;
        }

        local_path.parent().map(create_dir_all).transpose()?;

        let clone_url = repo_info.http_url();
        let mut builder = RepoBuilder::new();
        let mut fetch_options = FetchOptions::new();
        let mut remote_callbacks = RemoteCallbacks::new();

        let callback_cell = Rc::new(RefCell::new(progress_callback));
        let callback_clone = callback_cell.clone();

        remote_callbacks.pack_progress(move |_stage, current, total| {
            if total == 0 {
                return;
            }
            if let Ok(mut cb) = callback_clone.try_borrow_mut() {
                cb(current, total);
            }
        });

        let cell_clone = callback_cell.clone();
        let mut checkout_builder = CheckoutBuilder::new();
        checkout_builder.progress(move |_path, cur, total| {
            if total == 0 {
                return;
            }
            if let Ok(mut cb) = cell_clone.try_borrow_mut() {
                cb(cur, total);
            }
        });
        builder.with_checkout(checkout_builder);

        remote_callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });

        fetch_options.remote_callbacks(remote_callbacks);
        builder.fetch_options(fetch_options);
        builder.clone(&clone_url, &local_path)?;

        Ok(local_path)
    }

    pub fn is_repository_complete(&self, repo_path: &Path) -> bool {
        repo_path.join(".git").exists()
            && repo_path.join(".git/HEAD").exists()
            && repo_path.join(".git/objects").exists()
            && repo_path.join(".git/refs").exists()
    }

    pub fn delete_repository(&self, repo_info: &GitRepositoryRef) -> Result<()> {
        let local_path = self.get_local_repo_path(repo_info)?;
        if local_path.exists() {
            remove_dir_all(&local_path)?;
        }
        Ok(())
    }

    pub fn is_repository_cached(&self, remote_url: &str) -> bool {
        GitRepositoryRefParser::parse(remote_url)
            .and_then(|repo_info| {
                self.get_local_repo_path(&repo_info)
                    .map(|path| path.exists() && path.is_dir())
            })
            .unwrap_or(false)
    }
}

impl RemoteGitRepositoryClientInterface for RemoteGitRepositoryClient {
    fn get_local_repo_path(&self, repo_info: &GitRepositoryRef) -> Result<PathBuf> {
        self.get_local_repo_path(repo_info)
    }

    fn delete_repository(&self, repo_info: &GitRepositoryRef) -> Result<()> {
        self.delete_repository(repo_info)
    }

    fn is_repository_complete(&self, path: &Path) -> bool {
        self.is_repository_complete(path)
    }

    fn is_repository_cached(&self, remote_url: &str) -> bool {
        self.is_repository_cached(remote_url)
    }
}
