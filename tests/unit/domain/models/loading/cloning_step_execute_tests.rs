use gittype::domain::models::loading::{CloningStep, ExecutionContext, Step, StepResult};
use gittype::domain::stores::{RepositoryStore, RepositoryStoreInterface};
use gittype::GitTypeError;
use std::sync::Arc;

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
