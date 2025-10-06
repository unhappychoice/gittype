use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitRepository {
    pub user_name: String,
    pub repository_name: String,
    pub remote_url: String,
    pub branch: Option<String>,
    pub commit_hash: Option<String>,
    pub is_dirty: bool,
    pub root_path: Option<PathBuf>,
}

impl GitRepository {
    /// Create a GitRepository from a local path
    pub fn new_local(path: &PathBuf) -> Result<Self, crate::GitTypeError> {
        let repo = git2::Repository::open(path)
            .map_err(|e| crate::GitTypeError::TerminalError(format!("Failed to open git repository: {}", e)))?;

        // Get remote URL (origin)
        let remote_url = repo.find_remote("origin")
            .ok()
            .and_then(|remote| remote.url().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("file://{}", path.display()));

        // Extract user_name and repository_name from path or URL
        let (user_name, repository_name) = if remote_url.starts_with("file://") {
            // Use directory name as repository name
            let repo_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            ("local".to_string(), repo_name)
        } else {
            Self::parse_repo_info(&remote_url)
        };

        // Get current branch
        let branch = repo.head().ok()
            .and_then(|head| head.shorthand().map(|s| s.to_string()));

        // Get current commit hash
        let commit_hash = repo.head().ok()
            .and_then(|head| head.target())
            .map(|oid| oid.to_string());

        // Check if repository is dirty
        let is_dirty = repo.statuses(None)
            .map(|statuses| !statuses.is_empty())
            .unwrap_or(false);

        Ok(Self {
            user_name,
            repository_name,
            remote_url,
            branch,
            commit_hash,
            is_dirty,
            root_path: Some(path.clone()),
        })
    }

    /// Parse repository info from URL
    fn parse_repo_info(url: &str) -> (String, String) {
        // Try to extract owner/repo from URL
        if let Some(ssh_part) = url.strip_prefix("git@") {
            if let Some(colon_pos) = ssh_part.find(':') {
                let path = &ssh_part[colon_pos + 1..];
                let parts: Vec<&str> = path.trim_end_matches(".git").split('/').collect();
                if parts.len() >= 2 {
                    return (parts[0].to_string(), parts[1].to_string());
                }
            }
        }

        if let Some(url_without_protocol) = url.strip_prefix("https://").or_else(|| url.strip_prefix("http://")) {
            let parts: Vec<&str> = url_without_protocol.split('/').collect();
            if parts.len() >= 3 {
                return (parts[1].to_string(), parts[2].trim_end_matches(".git").to_string());
            }
        }

        ("unknown".to_string(), "unknown".to_string())
    }

    /// Generate a cache key from the repository URL.
    /// Supports multiple URL formats:
    /// - https://github.com/owner/repo -> github_com_owner_repo
    /// - git@github.com:owner/repo -> github_com_owner_repo
    /// - ssh://git@github.com/owner/repo -> github_com_owner_repo
    pub fn cache_key(&self) -> String {
        Self::extract_cache_key(&self.remote_url)
    }

    fn extract_cache_key(repo_url: &str) -> String {
        // Handle SSH format: git@host:owner/repo
        if let Some(ssh_part) = repo_url.strip_prefix("git@") {
            if let Some(colon_pos) = ssh_part.find(':') {
                let host = &ssh_part[..colon_pos];
                let path = &ssh_part[colon_pos + 1..];
                let parts: Vec<&str> = path.split('/').collect();
                if parts.len() >= 2 {
                    let host_clean = host.replace('.', "_");
                    let owner = parts[0];
                    let repo = parts[1].trim_end_matches(".git");
                    return format!("{}_{}_{}", host_clean, owner, repo);
                }
            }
        }

        // Handle ssh:// format: ssh://git@host/owner/repo
        if let Some(ssh_url) = repo_url.strip_prefix("ssh://") {
            if let Some(at_pos) = ssh_url.find('@') {
                let host_path = &ssh_url[at_pos + 1..];
                let parts: Vec<&str> = host_path.split('/').collect();
                if parts.len() >= 3 {
                    let host = parts[0].replace('.', "_");
                    let owner = parts[1];
                    let repo = parts[2].trim_end_matches(".git");
                    return format!("{}_{}_{}", host, owner, repo);
                }
            }
        }

        // Handle HTTP(S) format: https://github.com/owner/repo
        if let Some(url_without_protocol) = repo_url
            .strip_prefix("https://")
            .or_else(|| repo_url.strip_prefix("http://"))
        {
            let parts: Vec<&str> = url_without_protocol.split('/').collect();
            if parts.len() >= 3 {
                let host = parts[0].replace('.', "_");
                let owner = parts[1];
                let repo = parts[2].trim_end_matches(".git");
                return format!("{}_{}_{}", host, owner, repo);
            }
        }

        // Fallback for malformed URLs
        repo_url.replace(['/', ':', '.'], "_")
    }
}
