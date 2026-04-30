use git2::Repository;
use gittype::domain::models::loading::{CloningStep, ExecutionContext, Step, StepResult};
use gittype::domain::stores::{RepositoryStore, RepositoryStoreInterface};
use gittype::infrastructure::git::{GitRepositoryRefParser, RemoteGitRepositoryClient};
use gittype::GitTypeError;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

struct CachedRepositoryFixture {
    path: PathBuf,
    cleanup_root: PathBuf,
}

impl Drop for CachedRepositoryFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.cleanup_root);
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

fn unique_repo_spec() -> String {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("gittype-cloning-step-owner-{suffix}/repo-{suffix}")
}

fn create_cached_repository(repo_spec: &str) -> CachedRepositoryFixture {
    let repo_ref = GitRepositoryRefParser::parse(repo_spec).unwrap();
    let repo_path = RemoteGitRepositoryClient::new()
        .get_local_repo_path(&repo_ref)
        .unwrap();
    let cleanup_root = repo_path.parent().unwrap().to_path_buf();
    create_dir_all(&cleanup_root).unwrap();

    let repository = Repository::init(&repo_path).unwrap();
    repository
        .remote(
            "origin",
            &format!("https://github.com/{}/{}", repo_ref.owner, repo_ref.name),
        )
        .unwrap();

    CachedRepositoryFixture {
        path: repo_path,
        cleanup_root,
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
fn execute_reuses_cached_repository_and_updates_context_and_store() {
    let repo_spec = unique_repo_spec();
    let repo_ref = GitRepositoryRefParser::parse(&repo_spec).unwrap();
    let cached_repository = create_cached_repository(&repo_spec);
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let mut context = create_context(
        Some(&repo_spec),
        Some(repository_store.clone() as Arc<dyn RepositoryStoreInterface>),
    );

    let result = CloningStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::RepoPath(path) if path == cached_repository.path));
    assert_eq!(
        context.current_repo_path.as_ref(),
        Some(&cached_repository.path)
    );
    assert!(matches!(
        context.git_repository.as_ref(),
        Some(repository)
            if repository.user_name == repo_ref.owner
                && repository.repository_name == repo_ref.name
                && repository.remote_url
                    == format!("https://github.com/{}/{}", repo_ref.owner, repo_ref.name)
                && repository.root_path.as_ref() == Some(&cached_repository.path)
    ));
    assert!(matches!(
        repository_store.get_repository().as_ref(),
        Some(repository)
            if repository.user_name == repo_ref.owner
                && repository.repository_name == repo_ref.name
                && repository.root_path.as_ref() == Some(&cached_repository.path)
    ));
}
