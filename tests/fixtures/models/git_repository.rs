//! Test fixtures for GitRepository model

use gittype::domain::models::GitRepository;
use std::path::PathBuf;

/// Creates a default test GitRepository
pub fn build() -> GitRepository {
    GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123def456".to_string()),
        is_dirty: false,
        root_path: None,
    }
}

/// Creates a GitRepository with custom user and repository name
pub fn build_with_names(user_name: &str, repository_name: &str) -> GitRepository {
    GitRepository {
        user_name: user_name.to_string(),
        repository_name: repository_name.to_string(),
        remote_url: format!("https://github.com/{}/{}", user_name, repository_name),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123def456".to_string()),
        is_dirty: false,
        root_path: None,
    }
}

/// Creates a GitRepository with custom root path
pub fn build_with_root_path(root_path: PathBuf) -> GitRepository {
    GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123def456".to_string()),
        is_dirty: false,
        root_path: Some(root_path),
    }
}

/// Creates a dirty GitRepository (with uncommitted changes)
pub fn build_dirty() -> GitRepository {
    GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123def456".to_string()),
        is_dirty: true,
        root_path: None,
    }
}
