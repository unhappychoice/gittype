use crate::repository_manager::{RepoInfo, RepositoryManager};

/// Extract host, owner, and repository name from a git URL
fn parse_git_url(remote_url: &str) -> Option<(String, String, String)> {
    // Handle SSH format: git@host:owner/repo
    if let Some(ssh_part) = remote_url.strip_prefix("git@") {
        if let Some(colon_pos) = ssh_part.find(':') {
            let host = &ssh_part[..colon_pos];
            let path = &ssh_part[colon_pos + 1..];
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() >= 2 {
                let owner = parts[0];
                let repo = parts[1].trim_end_matches(".git");
                return Some((host.to_string(), owner.to_string(), repo.to_string()));
            }
        }
    }

    // Handle ssh:// format: ssh://git@host/owner/repo
    if let Some(ssh_url) = remote_url.strip_prefix("ssh://") {
        if let Some(at_pos) = ssh_url.find('@') {
            let host_path = &ssh_url[at_pos + 1..];
            let parts: Vec<&str> = host_path.split('/').collect();
            if parts.len() >= 3 {
                let host = parts[0];
                let owner = parts[1];
                let repo = parts[2].trim_end_matches(".git");
                return Some((host.to_string(), owner.to_string(), repo.to_string()));
            }
        }
    }

    // Handle HTTP(S) format: https://host/owner/repo
    if let Some(url_without_protocol) = remote_url
        .strip_prefix("https://")
        .or_else(|| remote_url.strip_prefix("http://"))
    {
        let parts: Vec<&str> = url_without_protocol.split('/').collect();
        if parts.len() >= 3 {
            let host = parts[0];
            let owner = parts[1];
            let repo = parts[2].trim_end_matches(".git");
            return Some((host.to_string(), owner.to_string(), repo.to_string()));
        }
    }

    None
}

/// Convert various git URL formats to HTTP URLs
pub fn format_http_url(remote_url: &str) -> String {
    if let Some((host, owner, repo)) = parse_git_url(remote_url) {
        format!("https://{}/{}/{}", host, owner, repo)
    } else {
        // Fallback: return as-is
        remote_url.to_string()
    }
}

/// Check if a repository is cached locally
pub fn is_repository_cached(remote_url: &str) -> bool {
    if let Some((host, owner, repo)) = parse_git_url(remote_url) {
        let repo_info = RepoInfo {
            origin: host,
            owner,
            name: repo,
        };

        match RepositoryManager::get_local_repo_path(&repo_info) {
            Ok(path) => path.exists() && path.is_dir(),
            Err(_) => false,
        }
    } else {
        false
    }
}
