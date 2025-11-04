use crate::domain::error::{GitTypeError, Result};
use crate::domain::models::GitRepository;
use crate::infrastructure::git::git_repository_ref_parser::GitRepositoryRefParser;
use git2::Repository;
use std::path::{Path, PathBuf};

pub struct LocalGitRepositoryClient;

impl LocalGitRepositoryClient {
    pub fn is_git_repository(path: &Path) -> bool {
        let git_dir = path.join(".git");
        git_dir.exists()
    }

    pub fn get_repository_root(path: &Path) -> Option<PathBuf> {
        let mut current_path = path.to_path_buf();

        loop {
            if Self::is_git_repository(&current_path) {
                return Some(current_path);
            }

            if !current_path.pop() {
                break;
            }
        }

        None
    }

    pub fn extract_git_repository(repo_path: &Path) -> Result<GitRepository> {
        let canonical_path = repo_path.canonicalize().map_err(|_| {
            GitTypeError::ExtractionFailed("Path canonicalization failed".to_string())
        })?;

        let git_root = Self::get_repository_root(&canonical_path).ok_or_else(|| {
            GitTypeError::ExtractionFailed("Git repository not found".to_string())
        })?;

        let repo = Repository::open(&git_root).map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Failed to open git repository: {}", e))
        })?;

        let remote_url = Self::get_remote_url(&repo)?;
        let repo_ref = GitRepositoryRefParser::parse(&remote_url).map_err(|_| {
            GitTypeError::ExtractionFailed("Failed to parse remote URL".to_string())
        })?;

        let branch = Self::get_current_branch(&repo).ok();
        let commit_hash = Self::get_current_commit_hash(&repo).ok();
        let is_dirty = Self::is_working_directory_dirty(&repo).unwrap_or(false);

        Ok(GitRepository {
            user_name: repo_ref.owner,
            repository_name: repo_ref.name,
            remote_url,
            branch,
            commit_hash,
            is_dirty,
            root_path: Some(git_root),
        })
    }

    fn get_remote_url(repo: &Repository) -> Result<String> {
        repo.find_remote("origin")
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to find origin remote: {}", e))
            })
            .map(|remote| remote.url().map(str::to_string))
            .and_then(|url_opt| {
                url_opt.ok_or_else(|| {
                    GitTypeError::ExtractionFailed("Remote URL is not valid UTF-8".to_string())
                })
            })
    }

    fn get_current_branch(repo: &Repository) -> Result<String> {
        repo.head()
            .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to get HEAD: {}", e)))
            .map(|head| head.shorthand().map(str::to_string))
            .and_then(|name_opt| {
                name_opt.ok_or_else(|| {
                    GitTypeError::ExtractionFailed("Branch name is not valid UTF-8".to_string())
                })
            })
    }

    fn get_current_commit_hash(repo: &Repository) -> Result<String> {
        repo.head()
            .map_err(|e| GitTypeError::ExtractionFailed(format!("Failed to get HEAD: {}", e)))
            .map(|head| head.target().map(|oid| oid.to_string()))
            .and_then(|oid_opt| {
                oid_opt.ok_or_else(|| {
                    GitTypeError::ExtractionFailed("HEAD does not point to a commit".to_string())
                })
            })
    }

    fn is_working_directory_dirty(repo: &Repository) -> Result<bool> {
        repo.statuses(None)
            .map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Failed to get repository status: {}", e))
            })
            .map(|statuses| !statuses.is_empty())
    }

    /// Create a GitRepository from a local path
    pub fn create_from_local_path(path: &Path) -> Result<GitRepository> {
        let repo = Repository::open(path).map_err(|e| {
            GitTypeError::ExtractionFailed(format!("Failed to open git repository: {}", e))
        })?;

        // Get remote URL (origin)
        let remote_url = repo
            .find_remote("origin")
            .ok()
            .and_then(|remote| remote.url().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("file://{}", path.display()));

        // Extract user_name and repository_name from path or URL
        let (user_name, repository_name) = if remote_url.starts_with("file://") {
            // Use directory name as repository name
            let repo_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            ("local".to_string(), repo_name)
        } else {
            GitRepositoryRefParser::parse(&remote_url)
                .map(|repo_ref| (repo_ref.owner, repo_ref.name))
                .unwrap_or_else(|_| ("unknown".to_string(), "unknown".to_string()))
        };

        // Get current branch
        let branch = repo
            .head()
            .ok()
            .and_then(|head| head.shorthand().map(|s| s.to_string()));

        // Get current commit hash
        let commit_hash = repo
            .head()
            .ok()
            .and_then(|head| head.target())
            .map(|oid| oid.to_string());

        // Check if repository is dirty
        let is_dirty = repo
            .statuses(None)
            .map(|statuses| !statuses.is_empty())
            .unwrap_or(false);

        Ok(GitRepository {
            user_name,
            repository_name,
            remote_url,
            branch,
            commit_hash,
            is_dirty,
            root_path: Some(path.to_path_buf()),
        })
    }
}
