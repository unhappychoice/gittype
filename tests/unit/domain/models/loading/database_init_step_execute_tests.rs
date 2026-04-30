use gittype::domain::models::loading::{DatabaseInitStep, ExecutionContext, Step, StepResult};
use gittype::domain::repositories::SessionRepository;

fn create_context<'a>() -> ExecutionContext<'a> {
    ExecutionContext {
        repo_spec: None,
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
        repository_store: None,
        session_store: None,
        stage_repository: None,
        session_manager: None,
    }
}

#[test]
fn execute_initializes_database_and_global_session_repository() {
    let mut context = create_context();

    let result = DatabaseInitStep.execute(&mut context).unwrap();
    let global = SessionRepository::global();
    let guard = global.lock().unwrap();

    assert!(matches!(result, StepResult::Skipped));
    assert!(guard.is_some());
}
