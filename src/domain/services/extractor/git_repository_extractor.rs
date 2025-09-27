use crate::domain::models::GitRepository;
use crate::{GitTypeError, Result};
use std::path::Path;
use std::process::Command;

// Re-export for backward compatibility - removed duplicate import

pub struct GitRepositoryExtractor;

impl Default for GitRepositoryExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl GitRepositoryExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_git_repository(repo_path: &Path) -> Result<Option<GitRepository>> {
        // Canonicalize the path to handle relative paths like ../../
        let canonical_path = match repo_path.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                // If canonicalization fails, the path might not exist
                return Ok(None);
            }
        };

        // Find git repository root (may be parent directory)
        let git_root = match Self::find_git_repository_root(&canonical_path) {
            Some(root) => root,
            None => return Ok(None),
        };

        let remote_url = Self::get_remote_url(&git_root)?;
        if let Some((user_name, repository_name)) = Self::parse_remote_url(&remote_url) {
            let branch = Self::get_current_branch(&git_root).ok();
            let commit_hash = Self::get_current_commit_hash(&git_root).ok();
            let is_dirty = Self::is_working_directory_dirty(&git_root).unwrap_or(false);

            Ok(Some(GitRepository {
                user_name,
                repository_name,
                remote_url,
                branch,
                commit_hash,
                is_dirty,
                root_path: Some(git_root),
            }))
        } else {
            Ok(None)
        }
    }

    fn find_git_repository_root(start_path: &Path) -> Option<std::path::PathBuf> {
        let mut current_path = start_path;

        loop {
            let git_dir = current_path.join(".git");
            if git_dir.exists() {
                return Some(current_path.to_path_buf());
            }

            // Move to parent directory
            match current_path.parent() {
                Some(parent) => current_path = parent,
                None => return None, // Reached root directory without finding .git
            }
        }
    }

    fn get_remote_url(repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["remote", "get-url", "origin"])
            .output()
            .map_err(GitTypeError::IoError)?;

        if !output.status.success() {
            return Err(GitTypeError::ExtractionFailed(
                "Failed to get remote URL".to_string(),
            ));
        }

        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(url)
    }

    fn parse_remote_url(url: &str) -> Option<(String, String)> {
        // Handle HTTPS URLs like https://github.com/user/repo.git
        if url.starts_with("https://github.com/") {
            let path = url.strip_prefix("https://github.com/")?;
            let path = path.strip_suffix(".git").unwrap_or(path);
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() == 2 {
                return Some((parts[0].to_string(), parts[1].to_string()));
            }
        }

        // Handle SSH URLs like git@github.com:user/repo.git
        if url.starts_with("git@github.com:") {
            let path = url.strip_prefix("git@github.com:")?;
            let path = path.strip_suffix(".git").unwrap_or(path);
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() == 2 {
                return Some((parts[0].to_string(), parts[1].to_string()));
            }
        }

        // Handle SSH URLs like ssh://git@github.com/user/repo.git or ssh://git@github.com/user/repo
        if url.starts_with("ssh://git@github.com/") {
            let path = url.strip_prefix("ssh://git@github.com/")?;
            let path = path.strip_suffix(".git").unwrap_or(path);
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() == 2 {
                return Some((parts[0].to_string(), parts[1].to_string()));
            }
        }

        None
    }

    fn get_current_branch(repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["branch", "--show-current"])
            .output()
            .map_err(GitTypeError::IoError)?;

        if !output.status.success() {
            return Err(GitTypeError::ExtractionFailed(
                "Failed to get current branch".to_string(),
            ));
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(branch)
    }

    fn get_current_commit_hash(repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["rev-parse", "HEAD"])
            .output()
            .map_err(GitTypeError::IoError)?;

        if !output.status.success() {
            return Err(GitTypeError::ExtractionFailed(
                "Failed to get current commit hash".to_string(),
            ));
        }

        let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(hash)
    }

    fn is_working_directory_dirty(repo_path: &Path) -> Result<bool> {
        let output = Command::new("git")
            .current_dir(repo_path)
            .args(["status", "--porcelain"])
            .output()
            .map_err(GitTypeError::IoError)?;

        if !output.status.success() {
            return Err(GitTypeError::ExtractionFailed(
                "Failed to check working directory status".to_string(),
            ));
        }

        let status = String::from_utf8_lossy(&output.stdout);
        Ok(!status.trim().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_git_repository_root_basic() {
        // Skip test if running in a Nix build environment
        if std::env::var("IN_NIX_SHELL").is_ok() || std::env::var("NIX_BUILD_CORES").is_ok() {
            eprintln!("Skipping test in Nix build environment.");
            return;
        }

        // Test with current project directory
        let current_dir = std::env::current_dir().unwrap();
        let git_root = GitRepositoryExtractor::find_git_repository_root(&current_dir);

        // Should find the git root of the current project
        assert!(git_root.is_some());
        let root = git_root.unwrap();
        assert!(root.join(".git").exists());
    }
}
