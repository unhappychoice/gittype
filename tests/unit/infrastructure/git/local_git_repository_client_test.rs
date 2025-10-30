#[cfg(test)]
mod tests {
    use gittype::infrastructure::git::LocalGitRepositoryClient;

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
}
