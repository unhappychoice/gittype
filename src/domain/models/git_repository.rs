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
