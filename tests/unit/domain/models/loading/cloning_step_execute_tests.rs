use gittype::domain::models::loading::{CloningStep, ExecutionContext, Step, StepResult};
use gittype::domain::stores::{RepositoryStore, RepositoryStoreInterface};
use gittype::infrastructure::git::{
    git_repository_ref_parser::GitRepositoryRefParser, RemoteGitRepositoryClient,
};
use gittype::GitTypeError;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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
fn execute_uses_complete_cached_repository_and_updates_context() {
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let repo_spec = unique_repo_spec();
    let repo_ref = GitRepositoryRefParser::parse(&repo_spec).unwrap();
    let remote_client = RemoteGitRepositoryClient::new();
    let cache_path = remote_client.get_local_repo_path(&repo_ref).unwrap();
    let origin_root = cache_path
        .ancestors()
        .nth(2)
        .expect("cache path should include origin root")
        .to_path_buf();

    std::fs::create_dir_all(&cache_path).unwrap();
    let repo = git2::Repository::init(&cache_path).unwrap();
    repo.remote("origin", &repo_spec).unwrap();

    let mut context = create_context(
        Some(&repo_spec),
        Some(repository_store.clone() as Arc<dyn RepositoryStoreInterface>),
    );

    let result = CloningStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::RepoPath(path) if path == cache_path));
    assert_eq!(context.current_repo_path, Some(cache_path.clone()));
    assert_eq!(
        context
            .git_repository
            .as_ref()
            .map(|repo| repo.remote_url.as_str()),
        Some(repo_spec.as_str())
    );
    assert!(repository_store.get_repository().is_some());

    std::fs::remove_dir_all(origin_root).unwrap();
}

fn unique_repo_spec() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    format!("https://coverage-{}.example.test/owner/repo.git", nanos)
}
