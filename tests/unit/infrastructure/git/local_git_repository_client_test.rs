#[cfg(test)]
mod tests {
    use git2::{Repository, Signature};
    use gittype::infrastructure::git::local::local_git_repository_client::LocalGitRepositoryClientInterface;
    use gittype::infrastructure::git::LocalGitRepositoryClient;
    use gittype::GitTypeError;
    use std::sync::Arc;

    fn commit_file(repo: &Repository, name: &str, content: &str) -> String {
        let workdir = repo.workdir().unwrap();
        std::fs::write(workdir.join(name), content).unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(std::path::Path::new(name)).unwrap();
        index.write().unwrap();

        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let signature = Signature::now("Test User", "test@example.com").unwrap();
        let commit_id = repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Initial commit",
                &tree,
                &[],
            )
            .unwrap();

        commit_id.to_string()
    }

    #[test]
    fn test_is_git_repository_without_git_dir() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let client = LocalGitRepositoryClient::new();
        let is_git_repo = client.is_git_repository(temp_dir.path());
        assert!(!is_git_repo);
    }

    #[test]
    fn test_is_git_repository_with_git_dir() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(temp_dir.path().join(".git")).unwrap();

        let client = LocalGitRepositoryClient::new();
        let is_git_repo = client.is_git_repository(temp_dir.path());
        assert!(is_git_repo);
    }

    #[test]
    fn test_get_repository_root_finds_parent_git_directory() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("src").join("nested");
        std::fs::create_dir_all(temp_dir.path().join(".git")).unwrap();
        std::fs::create_dir_all(&nested_dir).unwrap();

        let client = LocalGitRepositoryClient::new();
        let root = client.get_repository_root(&nested_dir);

        assert_eq!(root, Some(temp_dir.path().to_path_buf()));
    }

    #[test]
    fn test_get_repository_root_returns_none_outside_git_repository() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("src").join("nested");
        std::fs::create_dir_all(&nested_dir).unwrap();

        let client = LocalGitRepositoryClient::new();
        let root = client.get_repository_root(&nested_dir);

        assert!(root.is_none());
    }

    #[test]
    fn test_extract_git_repository_returns_error_for_missing_path() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let missing_path = temp_dir.path().join("missing");

        let client = LocalGitRepositoryClient::new();
        let result = client.extract_git_repository(&missing_path);

        assert!(matches!(
            result,
            Err(GitTypeError::ExtractionFailed(message))
            if message == "Path canonicalization failed"
        ));
    }

    #[test]
    fn test_extract_git_repository_returns_error_outside_git_repository() {
        let temp_dir = tempfile::TempDir::new().unwrap();

        let client = LocalGitRepositoryClient::new();
        let result = client.extract_git_repository(temp_dir.path());

        assert!(matches!(
            result,
            Err(GitTypeError::ExtractionFailed(message))
            if message == "Git repository not found"
        ));
    }

    #[test]
    fn test_extract_git_repository_returns_error_for_invalid_git_directory() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(temp_dir.path().join(".git")).unwrap();

        let client = LocalGitRepositoryClient::new();
        let result = client.extract_git_repository(temp_dir.path());

        assert!(matches!(
            result,
            Err(GitTypeError::ExtractionFailed(message))
                if message.starts_with("Failed to open git repository")
        ));
    }

    #[test]
    fn test_extract_git_repository_reads_origin_branch_commit_and_status() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        repo.remote("origin", "https://github.com/octocat/hello-world.git")
            .unwrap();
        let commit_hash = commit_file(&repo, "README.md", "hello");

        let client = LocalGitRepositoryClient::new();
        let git_repository = client.extract_git_repository(temp_dir.path()).unwrap();

        assert_eq!(git_repository.user_name, "octocat");
        assert_eq!(git_repository.repository_name, "hello-world");
        assert_eq!(
            git_repository.remote_url,
            "https://github.com/octocat/hello-world.git"
        );
        assert!(git_repository.branch.is_some());
        assert_eq!(git_repository.commit_hash, Some(commit_hash));
        assert!(!git_repository.is_dirty);
        assert_eq!(
            git_repository.root_path,
            Some(temp_dir.path().to_path_buf())
        );
    }

    #[test]
    fn test_create_from_local_path_uses_file_url_without_origin() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        Repository::init(temp_dir.path()).unwrap();

        let client = LocalGitRepositoryClient::new();
        let git_repository = client.create_from_local_path(temp_dir.path()).unwrap();

        assert_eq!(git_repository.user_name, "local");
        assert_eq!(
            git_repository.repository_name,
            temp_dir.path().file_name().unwrap().to_str().unwrap()
        );
        assert_eq!(
            git_repository.remote_url,
            format!("file://{}", temp_dir.path().display())
        );
        assert_eq!(
            git_repository.root_path,
            Some(temp_dir.path().to_path_buf())
        );
    }

    #[test]
    fn test_extract_git_repository_returns_error_when_origin_missing() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        commit_file(&repo, "README.md", "hello");

        let client = LocalGitRepositoryClient::new();
        let result = client.extract_git_repository(temp_dir.path());

        let message = match result {
            Err(GitTypeError::ExtractionFailed(msg)) => msg,
            other => panic!("expected ExtractionFailed, got {:?}", other),
        };
        assert!(
            message.starts_with("Failed to find origin remote"),
            "unexpected message: {message}"
        );
    }

    #[test]
    fn test_extract_git_repository_returns_error_for_unparseable_remote_url() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        repo.remote("origin", "not-a-valid-remote-url").unwrap();
        commit_file(&repo, "README.md", "hello");

        let client = LocalGitRepositoryClient::new();
        let result = client.extract_git_repository(temp_dir.path());

        assert!(matches!(
            result,
            Err(GitTypeError::ExtractionFailed(msg))
            if msg == "Failed to parse remote URL"
        ));
    }

    #[test]
    fn test_extract_git_repository_reports_dirty_working_tree() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        repo.remote("origin", "https://github.com/octocat/hello-world.git")
            .unwrap();
        commit_file(&repo, "README.md", "hello");
        std::fs::write(temp_dir.path().join("dirty.txt"), "uncommitted").unwrap();

        let client = LocalGitRepositoryClient::new();
        let git_repository = client.extract_git_repository(temp_dir.path()).unwrap();

        assert!(
            git_repository.is_dirty,
            "expected uncommitted file to mark repository as dirty"
        );
    }

    #[test]
    fn test_create_from_local_path_uses_origin_url_when_available() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        repo.remote("origin", "https://github.com/octocat/hello-world.git")
            .unwrap();
        let commit_hash = commit_file(&repo, "README.md", "hello");

        let client = LocalGitRepositoryClient::new();
        let git_repository = client.create_from_local_path(temp_dir.path()).unwrap();

        assert_eq!(git_repository.user_name, "octocat");
        assert_eq!(git_repository.repository_name, "hello-world");
        assert_eq!(
            git_repository.remote_url,
            "https://github.com/octocat/hello-world.git"
        );
        assert_eq!(git_repository.commit_hash, Some(commit_hash));
        assert!(git_repository.branch.is_some());
        assert!(!git_repository.is_dirty);
    }

    #[test]
    fn test_create_from_local_path_falls_back_to_unknown_for_unparseable_origin() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        repo.remote("origin", "totally-bogus-origin").unwrap();

        let client = LocalGitRepositoryClient::new();
        let git_repository = client.create_from_local_path(temp_dir.path()).unwrap();

        assert_eq!(git_repository.user_name, "unknown");
        assert_eq!(git_repository.repository_name, "unknown");
        assert_eq!(git_repository.remote_url, "totally-bogus-origin");
    }

    #[test]
    fn test_create_from_local_path_returns_error_for_non_git_directory() {
        let temp_dir = tempfile::TempDir::new().unwrap();

        let client = LocalGitRepositoryClient::new();
        let result = client.create_from_local_path(temp_dir.path());

        assert!(matches!(
            result,
            Err(GitTypeError::ExtractionFailed(msg))
            if msg.starts_with("Failed to open git repository")
        ));
    }

    #[test]
    fn test_interface_dispatch_delegates_to_inherent_methods() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();
        repo.remote("origin", "https://github.com/octocat/hello-world.git")
            .unwrap();
        commit_file(&repo, "README.md", "hello");

        let client: Arc<dyn LocalGitRepositoryClientInterface> =
            Arc::new(LocalGitRepositoryClient::new());

        assert!(client.is_git_repository(temp_dir.path()));

        let nested = temp_dir.path().join("nested");
        std::fs::create_dir_all(&nested).unwrap();
        assert_eq!(
            client.get_repository_root(&nested),
            Some(temp_dir.path().to_path_buf())
        );

        let git_repository = client.extract_git_repository(temp_dir.path()).unwrap();
        assert_eq!(git_repository.user_name, "octocat");
        assert_eq!(git_repository.repository_name, "hello-world");
    }
}
