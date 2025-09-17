use gittype::models::git_repository::GitRepository;

#[test]
fn git_repository_equality_depends_on_all_fields() {
    let repo = GitRepository {
        user_name: "user".into(),
        repository_name: "repo".into(),
        remote_url: "https://example.com/repo.git".into(),
        branch: Some("main".into()),
        commit_hash: Some("abc".into()),
        is_dirty: false,
        root_path: None,
    };

    let mut same = repo.clone();
    assert_eq!(repo, same);

    same.is_dirty = true;
    assert_ne!(repo, same);
}

#[test]
fn test_cache_key_https_url() {
    let repo = GitRepository {
        user_name: "user".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/owner/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };
    assert_eq!(repo.cache_key(), "github_com_owner_repo");
}

#[test]
fn test_cache_key_ssh_url() {
    let repo = GitRepository {
        user_name: "user".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "git@github.com:owner/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };
    assert_eq!(repo.cache_key(), "github_com_owner_repo");
}

#[test]
fn test_cache_key_ssh_protocol_url() {
    let repo = GitRepository {
        user_name: "user".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "ssh://git@github.com/owner/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };
    assert_eq!(repo.cache_key(), "github_com_owner_repo");
}

#[test]
fn test_cache_key_with_git_suffix() {
    let repo = GitRepository {
        user_name: "user".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/owner/repo.git".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };
    assert_eq!(repo.cache_key(), "github_com_owner_repo");
}

#[test]
fn test_cache_key_different_hosts() {
    let github_repo = GitRepository {
        user_name: "user".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://github.com/owner/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let gitlab_repo = GitRepository {
        user_name: "user".to_string(),
        repository_name: "repo".to_string(),
        remote_url: "https://gitlab.com/owner/repo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    assert_eq!(github_repo.cache_key(), "github_com_owner_repo");
    assert_eq!(gitlab_repo.cache_key(), "gitlab_com_owner_repo");
    assert_ne!(github_repo.cache_key(), gitlab_repo.cache_key());
}
