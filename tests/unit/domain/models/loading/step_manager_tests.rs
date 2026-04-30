use crate::fixtures::models::{challenge, git_repository};
use gittype::domain::events::{EventBus, EventBusInterface};
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::loading::{ExecutionContext, StepManager, StepType};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::{Challenge, GitRepository};
use gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use gittype::domain::services::scoring::{
    SessionTracker, SessionTrackerInterface, TotalTracker, TotalTrackerInterface,
};
use gittype::domain::services::session_manager_service::{SessionManager, SessionManagerInterface};
use gittype::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::stores::{
    ChallengeStore, ChallengeStoreInterface, RepositoryStore, RepositoryStoreInterface,
    SessionStore, SessionStoreInterface,
};
use gittype::presentation::tui::screens::loading_screen::{LoadingScreen, ProgressReporter};
use gittype::{GitTypeError, Result};
use std::sync::{Arc, Mutex};

#[test]
fn new_creates_all_seven_steps() {
    let manager = StepManager::new();
    assert_eq!(manager.get_all_steps().len(), 7);
}

#[test]
fn default_creates_same_as_new() {
    let manager = StepManager::default();
    assert_eq!(manager.get_all_steps().len(), 7);
}

#[test]
fn steps_are_in_correct_order() {
    let manager = StepManager::new();
    let steps = manager.get_all_steps();
    let types: Vec<StepType> = steps.iter().map(|s| s.step_type()).collect();
    assert_eq!(
        types,
        vec![
            StepType::DatabaseInit,
            StepType::Cloning,
            StepType::CacheCheck,
            StepType::Scanning,
            StepType::Extracting,
            StepType::Generating,
            StepType::Finalizing,
        ]
    );
}

#[test]
fn get_step_by_name_finds_existing() {
    let manager = StepManager::new();
    let step = manager.get_step_by_name("Database Setup");
    assert!(step.is_some());
    assert_eq!(step.unwrap().step_type(), StepType::DatabaseInit);
}

#[test]
fn get_step_by_name_returns_none_for_unknown() {
    let manager = StepManager::new();
    assert!(manager.get_step_by_name("Nonexistent").is_none());
}

#[test]
fn get_step_by_number_finds_existing() {
    let manager = StepManager::new();
    let step = manager.get_step_by_number(1);
    assert!(step.is_some());
    assert_eq!(step.unwrap().step_type(), StepType::DatabaseInit);
}

#[test]
fn get_step_by_number_returns_none_for_unknown() {
    let manager = StepManager::new();
    assert!(manager.get_step_by_number(999).is_none());
}

#[test]
fn step_name_to_step_number_returns_correct_number() {
    let manager = StepManager::new();
    assert_eq!(manager.step_name_to_step_number("Database Setup"), 1);
    assert_eq!(manager.step_name_to_step_number("Cloning repository"), 2);
    assert_eq!(manager.step_name_to_step_number("Cache check"), 3);
    assert_eq!(manager.step_name_to_step_number("Scanning repository"), 4);
    assert_eq!(manager.step_name_to_step_number("Finalizing"), 8);
}

#[test]
fn step_name_to_step_number_returns_zero_for_unknown() {
    let manager = StepManager::new();
    assert_eq!(manager.step_name_to_step_number("Unknown"), 0);
}

#[test]
fn get_step_by_name_all_steps() {
    let manager = StepManager::new();
    let names = [
        "Database Setup",
        "Cloning repository",
        "Cache check",
        "Scanning repository",
        "Extracting functions, classes, and code blocks",
        "Generating challenges",
        "Finalizing",
    ];
    for name in &names {
        assert!(
            manager.get_step_by_name(name).is_some(),
            "Should find step '{}'",
            name
        );
    }
}

#[test]
fn get_step_by_number_all_steps() {
    let manager = StepManager::new();
    let numbers = [1, 2, 3, 4, 5, 7, 8];
    for num in &numbers {
        assert!(
            manager.get_step_by_number(*num).is_some(),
            "Should find step number {}",
            num
        );
    }
}

#[test]
fn step_number_6_does_not_exist() {
    let manager = StepManager::new();
    assert!(manager.get_step_by_number(6).is_none());
}

struct MockChallengeRepository {
    challenges: Vec<Challenge>,
    load_calls: Mutex<usize>,
    save_calls: Mutex<usize>,
}

impl MockChallengeRepository {
    fn hit(challenges: Vec<Challenge>) -> Self {
        Self {
            challenges,
            load_calls: Mutex::new(0),
            save_calls: Mutex::new(0),
        }
    }

    fn load_calls(&self) -> usize {
        *self.load_calls.lock().unwrap()
    }

    fn save_calls(&self) -> usize {
        *self.save_calls.lock().unwrap()
    }
}

impl ChallengeRepositoryInterface for MockChallengeRepository {
    fn save_challenges(
        &self,
        _repo: &GitRepository,
        _challenges: &[Challenge],
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<()> {
        *self.save_calls.lock().unwrap() += 1;
        Ok(())
    }

    fn load_challenges_with_progress(
        &self,
        _repo: &GitRepository,
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<Option<Vec<Challenge>>> {
        *self.load_calls.lock().unwrap() += 1;
        Ok(Some(self.challenges.clone()))
    }

    fn get_cache_stats(&self) -> Result<(usize, u64)> {
        Ok((0, 0))
    }

    fn clear_cache(&self) -> Result<()> {
        Ok(())
    }

    fn invalidate_repository(&self, _repo: &GitRepository) -> Result<bool> {
        Ok(false)
    }

    fn list_cache_keys(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
}

struct TestServices {
    challenge_store: Arc<ChallengeStore>,
    repository_store: Arc<RepositoryStore>,
    session_store: Arc<SessionStore>,
    stage_repository: Arc<StageRepository>,
    session_manager: Arc<SessionManager>,
}

fn create_loading_screen(repository: Arc<dyn ChallengeRepositoryInterface>) -> LoadingScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;

    LoadingScreen::new_for_test(Arc::new(EventBus::new()), repository, theme_service)
}

fn create_services() -> TestServices {
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let session_store = Arc::new(SessionStore::new_for_test());
    let stage_repository = Arc::new(StageRepository::new(
        None,
        challenge_store.clone(),
        repository_store.clone(),
        session_store.clone(),
    ));
    let session_manager = Arc::new(SessionManager::new_with_dependencies(
        Arc::new(EventBus::new()) as Arc<dyn EventBusInterface>,
        stage_repository.clone(),
        Arc::new(SessionTracker::new_for_test()) as Arc<dyn SessionTrackerInterface>,
        Arc::new(TotalTracker::new_for_test()) as Arc<dyn TotalTrackerInterface>,
    ));

    TestServices {
        challenge_store,
        repository_store,
        session_store,
        stage_repository,
        session_manager,
    }
}

#[test]
fn execute_pipeline_propagates_scanning_error_without_loading_screen() {
    let repo_path = std::env::current_dir().unwrap();
    let mut context = ExecutionContext {
        repo_spec: None,
        repo_path: Some(&repo_path),
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
    };

    let error = StepManager::new()
        .execute_pipeline(&mut context)
        .unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message) if message == "No loading screen available"
    ));
    assert!(!context.cache_used);
    assert!(matches!(
        context.git_repository.as_ref(),
        Some(repository) if repository.root_path.as_ref() == Some(&repo_path)
    ));
}

#[test]
fn execute_pipeline_skips_remaining_steps_after_cache_hit() {
    let cached_challenges = vec![
        challenge::build_easy(),
        challenge::build(),
        challenge::build_hard(),
    ];
    let challenge_repository = Arc::new(MockChallengeRepository::hit(cached_challenges.clone()));
    let screen = create_loading_screen(challenge_repository.clone());
    let services = create_services();
    let mut context = ExecutionContext {
        repo_spec: None,
        repo_path: None,
        extraction_options: None,
        loading_screen: Some(&screen),
        challenge_repository: Some(
            challenge_repository.clone() as Arc<dyn ChallengeRepositoryInterface>
        ),
        current_repo_path: None,
        git_repository: Some(git_repository::build()),
        scanned_files: None,
        chunks: None,
        cache_used: false,
        challenge_store: Some(services.challenge_store.clone() as Arc<dyn ChallengeStoreInterface>),
        repository_store: Some(
            services.repository_store.clone() as Arc<dyn RepositoryStoreInterface>
        ),
        session_store: Some(services.session_store.clone() as Arc<dyn SessionStoreInterface>),
        stage_repository: Some(
            services.stage_repository.clone() as Arc<dyn StageRepositoryInterface>
        ),
        session_manager: Some(services.session_manager.clone() as Arc<dyn SessionManagerInterface>),
    };

    StepManager::new().execute_pipeline(&mut context).unwrap();

    assert!(context.cache_used);
    assert!(context.scanned_files.is_none());
    assert!(context.chunks.is_none());
    assert_eq!(challenge_repository.load_calls(), 1);
    assert_eq!(challenge_repository.save_calls(), 0);
    assert_eq!(
        services.challenge_store.get_challenges(),
        Some(cached_challenges)
    );
    assert!(services.session_store.is_loading_completed());
    assert!(
        services
            .stage_repository
            .count_challenges_by_difficulty()
            .iter()
            .sum::<usize>()
            > 0
    );
}
