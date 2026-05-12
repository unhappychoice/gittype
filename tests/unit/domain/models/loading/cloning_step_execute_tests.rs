use gittype::domain::models::loading::{CloningStep, ExecutionContext, Step, StepResult};
use gittype::domain::stores::{RepositoryStore, RepositoryStoreInterface};
use gittype::infrastructure::git::{GitRepositoryRefParser, RemoteGitRepositoryClient};
use gittype::GitTypeError;
use std::path::PathBuf;
use std::sync::Arc;

struct RepoPathCleanup(PathBuf);

impl Drop for RepoPathCleanup {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn create_context<'a>(
    repo_spec: Option<&'a str>,
    repository_store: Option<Arc<dyn RepositoryStoreInterface>>,
) -> ExecutionContext<'a> {
    ExecutionContext {
        repo_spec,
        repo_path: None,
        extraction_options: None,
        loading_screen: None,
        challenge_repository: None,
        current_repo_path: None,
        git_repository: None,
        scanned_files: None,
        chunks: None,
        cache_used: false,
        challenge_store: None,
        repository_store,
        session_store: None,
        stage_repository: None,
        session_manager: None,
    }
}

#[test]
fn execute_skips_without_repo_spec() {
    let mut context = create_context(None, None);

    let result = CloningStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
    assert!(context.current_repo_path.is_none());
    assert!(context.git_repository.is_none());
}

#[test]
fn execute_returns_invalid_repository_error_for_unsupported_spec() {
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let mut context = create_context(
        Some("invalid repository spec"),
        Some(repository_store.clone() as Arc<dyn RepositoryStoreInterface>),
    );

    let error = CloningStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::InvalidRepositoryFormat(message)
            if message.contains("Unsupported repository format")
    ));
    assert!(context.current_repo_path.is_none());
    assert!(context.git_repository.is_none());
    assert!(repository_store.get_repository().is_none());
}

#[test]
fn execute_uses_complete_cached_repository() {
    let process_id = std::process::id();
    let repo_name = format!("cloning-step-cache-{}", process_id);
    let repo_spec = format!("https://github.com/coverage-owner/{}", repo_name);
    let repo_info = GitRepositoryRefParser::parse(&repo_spec).unwrap();
    let remote_client = RemoteGitRepositoryClient::new();
    let repo_path = remote_client.get_local_repo_path(&repo_info).unwrap();
    let _ = std::fs::remove_dir_all(&repo_path);
    let _cleanup = RepoPathCleanup(repo_path.clone());
    std::fs::create_dir_all(repo_path.parent().unwrap()).unwrap();

    let git_repo = git2::Repository::init(&repo_path).unwrap();
    git_repo.remote("origin", &repo_spec).unwrap();

    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let mut context = create_context(
        Some(&repo_spec),
        Some(repository_store.clone() as Arc<dyn RepositoryStoreInterface>),
    );

    let result = CloningStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::RepoPath(path) if path == repo_path));
    assert_eq!(context.current_repo_path, Some(repo_path.clone()));
    assert_eq!(
        context
            .git_repository
            .as_ref()
            .map(|repo| repo.repository_name.as_str()),
        Some(repo_name.as_str())
    );
    assert!(repository_store.get_repository().is_some());
}
