use crate::fixtures::models::{challenge, git_repository};
use gittype::domain::models::loading::{CacheCheckStep, ExecutionContext, Step, StepResult};
use gittype::domain::models::{Challenge, GitRepository};
use gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use gittype::domain::stores::{
    ChallengeStore, ChallengeStoreInterface, RepositoryStore, RepositoryStoreInterface,
    SessionStore, SessionStoreInterface,
};
use gittype::presentation::tui::screens::loading_screen::ProgressReporter;
use gittype::{GitTypeError, Result};
use std::sync::{Arc, Mutex};

enum LoadBehavior {
    Hit(Vec<Challenge>),
    Miss,
    Error(String),
}

struct MockChallengeRepository {
    behavior: LoadBehavior,
    load_calls: Mutex<usize>,
}

impl MockChallengeRepository {
    fn hit(challenges: Vec<Challenge>) -> Self {
        Self {
            behavior: LoadBehavior::Hit(challenges),
            load_calls: Mutex::new(0),
        }
    }

    fn miss() -> Self {
        Self {
            behavior: LoadBehavior::Miss,
            load_calls: Mutex::new(0),
        }
    }

    fn error(message: &str) -> Self {
        Self {
            behavior: LoadBehavior::Error(message.to_string()),
            load_calls: Mutex::new(0),
        }
    }

    fn load_calls(&self) -> usize {
        *self.load_calls.lock().unwrap()
    }
}

impl ChallengeRepositoryInterface for MockChallengeRepository {
    fn save_challenges(
        &self,
        _repo: &GitRepository,
        _challenges: &[Challenge],
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<()> {
        Ok(())
    }

    fn load_challenges_with_progress(
        &self,
        _repo: &GitRepository,
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<Option<Vec<Challenge>>> {
        *self.load_calls.lock().unwrap() += 1;
        match &self.behavior {
            LoadBehavior::Hit(challenges) => Ok(Some(challenges.clone())),
            LoadBehavior::Miss => Ok(None),
            LoadBehavior::Error(message) => {
                Err(GitTypeError::ExtractionFailed(message.to_string()))
            }
        }
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

fn create_context<'a>(
    git_repository: Option<GitRepository>,
    challenge_repository: Option<Arc<dyn ChallengeRepositoryInterface>>,
    challenge_store: Option<Arc<dyn ChallengeStoreInterface>>,
    repository_store: Option<Arc<dyn RepositoryStoreInterface>>,
    session_store: Option<Arc<dyn SessionStoreInterface>>,
) -> ExecutionContext<'a> {
    ExecutionContext {
        repo_spec: None,
        repo_path: None,
        extraction_options: None,
        loading_screen: None,
        challenge_repository,
        current_repo_path: None,
        git_repository,
        scanned_files: None,
        chunks: None,
        cache_used: false,
        challenge_store,
        repository_store,
        session_store,
        stage_repository: None,
        session_manager: None,
    }
}

#[test]
fn execute_skips_without_git_repository() {
    let repository = Arc::new(MockChallengeRepository::miss());
    let mut context = create_context(
        None,
        Some(repository.clone() as Arc<dyn ChallengeRepositoryInterface>),
        None,
        None,
        None,
    );

    let result = CacheCheckStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
    assert_eq!(repository.load_calls(), 0);
    assert!(!context.cache_used);
}

#[test]
fn execute_skips_dirty_repository_without_cache_lookup() {
    let repository = Arc::new(MockChallengeRepository::miss());
    let mut context = create_context(
        Some(git_repository::build_dirty()),
        Some(repository.clone() as Arc<dyn ChallengeRepositoryInterface>),
        None,
        None,
        None,
    );

    let result = CacheCheckStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
    assert_eq!(repository.load_calls(), 0);
    assert!(!context.cache_used);
}

#[test]
fn execute_skips_when_challenge_repository_is_missing() {
    let mut context = create_context(Some(git_repository::build()), None, None, None, None);

    let result = CacheCheckStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
    assert!(!context.cache_used);
}

#[test]
fn execute_skips_when_cache_lookup_misses_or_errors() {
    let git_repository = git_repository::build();
    let miss_repository = Arc::new(MockChallengeRepository::miss());
    let error_repository = Arc::new(MockChallengeRepository::error("cache failed"));

    let mut miss_context = create_context(
        Some(git_repository.clone()),
        Some(miss_repository.clone() as Arc<dyn ChallengeRepositoryInterface>),
        None,
        None,
        None,
    );
    let mut error_context = create_context(
        Some(git_repository),
        Some(error_repository.clone() as Arc<dyn ChallengeRepositoryInterface>),
        None,
        None,
        None,
    );

    assert!(matches!(
        CacheCheckStep.execute(&mut miss_context).unwrap(),
        StepResult::Skipped
    ));
    assert!(matches!(
        CacheCheckStep.execute(&mut error_context).unwrap(),
        StepResult::Skipped
    ));
    assert_eq!(miss_repository.load_calls(), 1);
    assert_eq!(error_repository.load_calls(), 1);
    assert!(!miss_context.cache_used);
    assert!(!error_context.cache_used);
}

#[test]
fn execute_populates_stores_when_cache_hits() {
    let git_repository = git_repository::build();
    let challenges = vec![challenge::build(), challenge::build_with_id("cached-2")];
    let repository = Arc::new(MockChallengeRepository::hit(challenges.clone()));
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let session_store = Arc::new(SessionStore::new_for_test());
    let mut context = create_context(
        Some(git_repository.clone()),
        Some(repository.clone() as Arc<dyn ChallengeRepositoryInterface>),
        Some(challenge_store.clone() as Arc<dyn ChallengeStoreInterface>),
        Some(repository_store.clone() as Arc<dyn RepositoryStoreInterface>),
        Some(session_store.clone() as Arc<dyn SessionStoreInterface>),
    );

    let result = CacheCheckStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
    assert_eq!(repository.load_calls(), 1);
    assert!(context.cache_used);
    assert_eq!(challenge_store.get_challenges(), Some(challenges));
    assert_eq!(repository_store.get_repository(), Some(git_repository));
    assert!(session_store.is_loading_completed());
}
