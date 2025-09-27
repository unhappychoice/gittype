use crate::repository_manager::{RepoInfo, RepositoryManager};

/// Extract host, owner path (can include `/` for subgroups), and repo name (no .git)
fn parse_git_url(remote_url: &str) -> Option<(String, String, String)> {
    fn normalize_host(h: &str) -> String {
        // Drop port if present; lowercase for consistent cache paths
        h.split(':').next().unwrap_or(h).to_ascii_lowercase()
    }

    // 1) scp-like SSH: user@host:path
    if let Some((user_host, path)) = remote_url.split_once(':') {
        if let Some((_user, host)) = user_host.rsplit_once('@') {
            let host = normalize_host(host);
            let path = path.trim_start_matches('/').split('?').next().unwrap_or("");
            let mut segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            if segs.len() >= 2 {
                let repo = segs.pop().unwrap().trim_end_matches(".git").to_string();
                let owner = segs.join("/");
                return Some((host, owner, repo));
            }
        }
    }

    // 2) ssh:// or http(s)://
    if let Some(rest) = remote_url
        .strip_prefix("ssh://")
        .or_else(|| remote_url.strip_prefix("https://"))
        .or_else(|| remote_url.strip_prefix("http://"))
    {
        let rest = rest.split('#').next().unwrap_or(rest); // drop fragment
        let rest = rest.split('?').next().unwrap_or(rest); // drop query
        if let Some((host_port, path)) = rest.split_once('/') {
            let host = normalize_host(host_port);
            let mut segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            if segs.len() >= 2 {
                let repo = segs.pop().unwrap().trim_end_matches(".git").to_string();
                let owner = segs.join("/");
                return Some((host, owner, repo));
            }
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
