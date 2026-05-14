use crate::fixtures::models::challenge;
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::models::loading::{ExecutionContext, FinalizingStep, Step, StepResult};
use gittype::domain::models::{Challenge, DifficultyLevel, SessionConfig, SessionState};
use gittype::domain::services::scoring::{
    SessionTracker, SessionTrackerInterface, TotalTracker, TotalTrackerInterface,
};
use gittype::domain::services::session_manager_service::{SessionManager, SessionManagerInterface};
use gittype::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use gittype::domain::stores::{
    ChallengeStore, ChallengeStoreInterface, RepositoryStore, RepositoryStoreInterface,
    SessionStore, SessionStoreInterface,
};
use gittype::GitTypeError;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct TestServices {
    challenge_store: Arc<ChallengeStore>,
    stage_repository: Arc<StageRepository>,
    session_manager: Arc<SessionManager>,
}

fn create_context<'a>(
    challenge_store: Option<Arc<dyn ChallengeStoreInterface>>,
    stage_repository: Option<Arc<dyn StageRepositoryInterface>>,
    session_manager: Option<Arc<dyn SessionManagerInterface>>,
) -> ExecutionContext<'a> {
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
        challenge_store,
        repository_store: None,
        session_store: None,
        stage_repository,
        session_manager,
    }
}

fn create_challenges() -> Vec<Challenge> {
    vec![
        challenge::build_easy(),
        challenge::build(),
        challenge::build_hard(),
    ]
}

fn create_services(challenges: Vec<Challenge>) -> TestServices {
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    challenge_store.set_challenges(challenges);

    let repository_store =
        Arc::new(RepositoryStore::new_for_test()) as Arc<dyn RepositoryStoreInterface>;
    let session_store = Arc::new(SessionStore::new_for_test()) as Arc<dyn SessionStoreInterface>;
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store.clone(),
        repository_store,
        session_store,
    ));
    let session_manager = Arc::new(SessionManager::new_with_dependencies(
        Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>,
        stage_repository.clone(),
        Arc::new(SessionTracker::new_for_test()) as Arc<dyn SessionTrackerInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
    ));

    TestServices {
        challenge_store,
        stage_repository,
        session_manager,
    }
}

#[test]
fn execute_errors_when_challenge_store_is_missing() {
    let mut context = create_context(None, None, None);

    let error = FinalizingStep.execute(&mut context).unwrap_err();

    match error {
        GitTypeError::TerminalError(message) => {
            assert_eq!(message, "ChallengeStore not available");
        }
        other => panic!("Expected TerminalError, got {other:?}"),
    }
}

#[test]
fn execute_errors_when_challenge_store_is_empty() {
    let challenge_store =
        Arc::new(ChallengeStore::new_for_test()) as Arc<dyn ChallengeStoreInterface>;
    let mut context = create_context(Some(challenge_store), None, None);

    let error = FinalizingStep.execute(&mut context).unwrap_err();

    match error {
        GitTypeError::ExtractionFailed(message) => {
            assert_eq!(message, "No challenges available for finalization");
        }
        other => panic!("Expected ExtractionFailed, got {other:?}"),
    }
}

#[test]
fn execute_succeeds_without_optional_services() {
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    challenge_store.set_challenges(vec![challenge::build()]);
    let mut context = create_context(Some(challenge_store), None, None);

    let result = FinalizingStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
}

#[test]
fn execute_resolves_git_repository_from_current_repo_path() {
    let temp_dir = tempfile::tempdir().unwrap();
    git2::Repository::init(temp_dir.path()).unwrap();
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    challenge_store.set_challenges(vec![challenge::build()]);
    let mut context = create_context(Some(challenge_store), None, None);
    context.current_repo_path = Some(temp_dir.path().to_path_buf());

    let result = FinalizingStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
}

#[test]
fn execute_builds_indices_and_resets_session_manager() {
    let services = create_services(create_challenges());
    services
        .session_manager
        .set_state(SessionState::InProgress {
            current_stage: 2,
            started_at: Instant::now(),
        });
    services.session_manager.set_config(SessionConfig {
        max_stages: 5,
        session_timeout: Some(Duration::from_secs(30)),
        difficulty: DifficultyLevel::Hard,
        max_skips: 1,
    });

    let mut context = create_context(
        Some(services.challenge_store.clone()),
        Some(services.stage_repository.clone()),
        Some(services.session_manager.clone()),
    );

    let result = FinalizingStep.execute(&mut context).unwrap();

    services.challenge_store.clear();

    assert!(matches!(result, StepResult::Skipped));
    assert_eq!(
        services.stage_repository.count_challenges_by_difficulty(),
        [1, 1, 1, 0, 0]
    );
    assert!(matches!(
        services.session_manager.get_state(),
        SessionState::NotStarted
    ));
    assert_eq!(
        services.session_manager.get_difficulty(),
        DifficultyLevel::Normal
    );
    assert_eq!(services.session_manager.get_skips_remaining().unwrap(), 3);
    assert_eq!(services.session_manager.get_stage_info().unwrap(), (0, 3));
}
