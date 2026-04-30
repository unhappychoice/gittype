#[cfg(test)]
mod tests {
    use git2::Repository;
    use gittype::domain::models::GitRepositoryRef;
    use gittype::infrastructure::git::remote::remote_git_repository_client::RemoteGitRepositoryClientInterface;
    use gittype::infrastructure::git::{GitRepositoryRefParser, RemoteGitRepositoryClient};
    use std::fs::create_dir_all;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct CachedRepositoryFixture {
        path: PathBuf,
        repo_info: GitRepositoryRef,
        cleanup_root: PathBuf,
    }

    impl Drop for CachedRepositoryFixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.cleanup_root);
        }
    }

    fn unique_repo_spec() -> String {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("gittype-remote-client-owner-{suffix}/repo-{suffix}")
    }

    fn unique_https_repo_spec() -> String {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("https://127.0.0.1:1/owner-{suffix}/repo-{suffix}.git")
    }

    fn create_cached_repository(repo_spec: &str) -> CachedRepositoryFixture {
        let client = RemoteGitRepositoryClient::new();
        let repo_info = GitRepositoryRefParser::parse(repo_spec).unwrap();
        let path = client.get_local_repo_path(&repo_info).unwrap();
        let cleanup_root = path.parent().unwrap().to_path_buf();

        create_dir_all(&cleanup_root).unwrap();
        Repository::init(&path).unwrap();

        CachedRepositoryFixture {
            path,
            repo_info,
            cleanup_root,
        }
    }

    #[test]
    fn test_is_repository_complete_without_git_dir() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let client = RemoteGitRepositoryClient::new();
        let is_complete = client.is_repository_complete(temp_dir.path());
        assert!(!is_complete);
    }

    #[test]
    fn test_is_repository_complete_with_git_only() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(temp_dir.path().join(".git")).unwrap();

        let client = RemoteGitRepositoryClient::new();
        let is_complete = client.is_repository_complete(temp_dir.path());
        assert!(!is_complete);
    }

    #[test]
    fn test_is_repository_complete_with_complete_git_structure() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let git_dir = temp_dir.path().join(".git");

        std::fs::create_dir_all(&git_dir).unwrap();
        std::fs::write(git_dir.join("HEAD"), "ref: refs/heads/main").unwrap();
        std::fs::create_dir_all(git_dir.join("objects")).unwrap();
        std::fs::create_dir_all(git_dir.join("refs")).unwrap();

        let client = RemoteGitRepositoryClient::new();
        let is_complete = client.is_repository_complete(temp_dir.path());
        assert!(is_complete);
    }

    #[test]
    fn test_get_local_repo_path_uses_home_directory_structure() {
        let client = RemoteGitRepositoryClient::new();
        let repo_info = GitRepositoryRef {
            origin: "github.com".to_string(),
            owner: "octocat".to_string(),
            name: "hello-world".to_string(),
        };

        let path = client.get_local_repo_path(&repo_info).unwrap();
        let expected = dirs::home_dir()
            .unwrap()
            .join(".gittype")
            .join("repos")
            .join("github.com")
            .join("octocat")
            .join("hello-world");

        assert_eq!(path, expected);
    }

    #[test]
    fn test_delete_repository_removes_cached_directory() {
        let repo_spec = unique_repo_spec();
        let cached_repository = create_cached_repository(&repo_spec);
        let client = RemoteGitRepositoryClient::new();

        client
            .delete_repository(&cached_repository.repo_info)
            .unwrap();

        assert!(!cached_repository.path.exists());
    }

    #[test]
    fn test_is_repository_cached_returns_true_for_existing_directory() {
        let repo_spec = unique_repo_spec();
        let cached_repository = create_cached_repository(&repo_spec);
        let client = RemoteGitRepositoryClient::new();

        assert!(client.is_repository_cached(&repo_spec));
        assert!(cached_repository.path.exists());
    }

    #[test]
    fn test_is_repository_cached_returns_false_for_invalid_spec() {
        let client = RemoteGitRepositoryClient::new();

        assert!(!client.is_repository_cached("invalid repository spec"));
    }

    #[test]
    fn test_clone_repository_reuses_complete_cached_repository() {
        let repo_spec = unique_repo_spec();
        let cached_repository = create_cached_repository(&repo_spec);
        let client = RemoteGitRepositoryClient::new();
        let mut progress_updates = Vec::new();

        let path = client
            .clone_repository(&repo_spec, |current, total| {
                progress_updates.push((current, total));
            })
            .unwrap();

        assert_eq!(path, cached_repository.path);
        assert!(progress_updates.is_empty());
    }

    #[test]
    fn test_delete_repository_succeeds_when_repository_is_missing() {
        let client = RemoteGitRepositoryClient::new();
        let repo_info = GitRepositoryRefParser::parse(&unique_repo_spec()).unwrap();
        let path = client.get_local_repo_path(&repo_info).unwrap();

        client.delete_repository(&repo_info).unwrap();

        assert!(!path.exists());
    }

    #[test]
    fn test_trait_methods_delegate_to_inherent_implementations() {
        let repo_spec = unique_repo_spec();
        let cached_repository = create_cached_repository(&repo_spec);
        let client = RemoteGitRepositoryClient::new();
        let temp_dir = tempfile::TempDir::new().unwrap();

        let path =
            <RemoteGitRepositoryClient as RemoteGitRepositoryClientInterface>::get_local_repo_path(
                &client,
                &cached_repository.repo_info,
            )
            .unwrap();
        let is_complete =
            <RemoteGitRepositoryClient as RemoteGitRepositoryClientInterface>::is_repository_complete(
                &client,
                cached_repository.path.as_path(),
            );
        let is_cached =
            <RemoteGitRepositoryClient as RemoteGitRepositoryClientInterface>::is_repository_cached(
                &client, &repo_spec,
            );

        assert_eq!(path, cached_repository.path);
        assert!(is_complete);
        assert!(is_cached);

        <RemoteGitRepositoryClient as RemoteGitRepositoryClientInterface>::delete_repository(
            &client,
            &cached_repository.repo_info,
        )
        .unwrap();

        assert!(!cached_repository.path.exists());
        assert!(
            !<RemoteGitRepositoryClient as RemoteGitRepositoryClientInterface>::is_repository_complete(
                &client,
                temp_dir.path(),
            )
        );
    }

    #[test]
    fn test_clone_repository_removes_incomplete_directory_before_clone_attempt() {
        let client = RemoteGitRepositoryClient::new();
        let repo_spec = unique_https_repo_spec();
        let repo_info = GitRepositoryRefParser::parse(&repo_spec).unwrap();
        let local_path = client.get_local_repo_path(&repo_info).unwrap();
        let cleanup_root = local_path.parent().unwrap().to_path_buf();
        let marker_path = local_path.join("marker.txt");

        create_dir_all(&local_path).unwrap();
        std::fs::write(&marker_path, "stale-cache").unwrap();

        let result = client.clone_repository(&repo_spec, |_, _| {});

        assert!(result.is_err());
        assert!(!marker_path.exists());

        let _ = std::fs::remove_dir_all(cleanup_root);
    }
}
