#[cfg(test)]
mod tests {
    use gittype::domain::models::GitRepositoryRef;
    use gittype::infrastructure::git::remote::remote_git_repository_client::RemoteGitRepositoryClientInterface;
    use gittype::infrastructure::git::{GitRepositoryRefParser, RemoteGitRepositoryClient};
    use std::time::{SystemTime, UNIX_EPOCH};

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
    fn test_is_repository_cached_returns_false_for_invalid_spec() {
        let client = RemoteGitRepositoryClient::new();

        assert!(!client.is_repository_cached("invalid repository spec"));
    }

    #[test]
    fn test_is_repository_cached_returns_true_for_existing_directory() {
        let client = RemoteGitRepositoryClient::new();
        let repo_info = test_repo_info("cached");
        let path = client.get_local_repo_path(&repo_info).unwrap();
        std::fs::create_dir_all(&path).unwrap();

        assert!(client.is_repository_cached(&format!("{}/{}", repo_info.owner, repo_info.name)));

        std::fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn test_trait_delete_repository_removes_existing_directory() {
        let client = RemoteGitRepositoryClient::new();
        let repo_info = test_repo_info("delete");
        let path = client.get_local_repo_path(&repo_info).unwrap();
        std::fs::create_dir_all(&path).unwrap();

        RemoteGitRepositoryClientInterface::delete_repository(&client, &repo_info).unwrap();

        assert!(!path.exists());
    }

    #[test]
    fn test_clone_repository_returns_existing_complete_cache() {
        let client = RemoteGitRepositoryClient::new();
        let repo_info = test_repo_info("cached-clone");
        let path = client.get_local_repo_path(&repo_info).unwrap();
        create_complete_git_structure(&path);
        let mut progress_calls = 0;

        let result = client
            .clone_repository(
                &format!("{}/{}", repo_info.owner, repo_info.name),
                |_, _| {
                    progress_calls += 1;
                },
            )
            .unwrap();

        assert_eq!(result, path);
        assert_eq!(progress_calls, 0);

        std::fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn test_trait_methods_delegate_cache_and_complete_checks() {
        let client = RemoteGitRepositoryClient::new();
        let repo_info = test_repo_info("trait-cache");
        let path =
            RemoteGitRepositoryClientInterface::get_local_repo_path(&client, &repo_info).unwrap();
        create_complete_git_structure(&path);

        assert_eq!(path, client.get_local_repo_path(&repo_info).unwrap());
        assert!(RemoteGitRepositoryClientInterface::is_repository_complete(
            &client, &path
        ));
        assert!(RemoteGitRepositoryClientInterface::is_repository_cached(
            &client,
            &format!("{}/{}", repo_info.owner, repo_info.name)
        ));

        std::fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn test_parse_repo_spec_for_https_url() {
        let parsed = GitRepositoryRefParser::parse("https://github.com/octocat/hello-world.git");
        assert!(parsed.is_ok());
        let repo_info = parsed.unwrap();
        assert_eq!(repo_info.owner, "octocat");
        assert_eq!(repo_info.name, "hello-world");
    }

    fn test_repo_info(prefix: &str) -> GitRepositoryRef {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        GitRepositoryRef {
            origin: "github.com".to_string(),
            owner: "gittype-test".to_string(),
            name: format!("{}-{}", prefix, nanos),
        }
    }

    fn create_complete_git_structure(path: &std::path::Path) {
        let git_dir = path.join(".git");

        std::fs::create_dir_all(&git_dir).unwrap();
        std::fs::write(git_dir.join("HEAD"), "ref: refs/heads/main").unwrap();
        std::fs::create_dir_all(git_dir.join("objects")).unwrap();
        std::fs::create_dir_all(git_dir.join("refs")).unwrap();
    }
}
