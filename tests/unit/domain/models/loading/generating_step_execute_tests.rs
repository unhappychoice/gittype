use crate::fixtures::models::git_repository;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::loading::{ExecutionContext, GeneratingStep, Step, StepResult};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::{Challenge, ChunkType, CodeChunk, GitRepository};
use gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::domain::stores::{
    ChallengeStore, ChallengeStoreInterface, RepositoryStore, RepositoryStoreInterface,
    SessionStore, SessionStoreInterface,
};
use gittype::presentation::tui::screens::loading_screen::{LoadingScreen, ProgressReporter};
use gittype::{GitTypeError, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

enum SaveBehavior {
    Success,
    Error(String),
}

struct MockChallengeRepository {
    behavior: SaveBehavior,
    save_calls: Mutex<Vec<(String, usize)>>,
}

impl MockChallengeRepository {
    fn successful() -> Self {
        Self {
            behavior: SaveBehavior::Success,
            save_calls: Mutex::new(vec![]),
        }
    }

    fn failing(message: &str) -> Self {
        Self {
            behavior: SaveBehavior::Error(message.to_string()),
            save_calls: Mutex::new(vec![]),
        }
    }

    fn save_calls(&self) -> Vec<(String, usize)> {
        self.save_calls.lock().unwrap().clone()
    }
}

impl ChallengeRepositoryInterface for MockChallengeRepository {
    fn save_challenges(
        &self,
        repo: &GitRepository,
        challenges: &[Challenge],
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<()> {
        self.save_calls
            .lock()
            .unwrap()
            .push((repo.remote_url.clone(), challenges.len()));

        match &self.behavior {
            SaveBehavior::Success => Ok(()),
            SaveBehavior::Error(message) => {
                Err(GitTypeError::ExtractionFailed(message.to_string()))
            }
        }
    }

    fn load_challenges_with_progress(
        &self,
        _repo: &GitRepository,
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<Option<Vec<Challenge>>> {
        Ok(None)
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

fn create_loading_screen() -> LoadingScreen {
    let theme_service = Arc::new(ThemeService::new_for_test(
        Theme::default(),
        ColorMode::Dark,
    )) as Arc<dyn ThemeServiceInterface>;

    LoadingScreen::new_for_test(
        Arc::new(EventBus::new()),
        Arc::new(MockChallengeRepository::successful()),
        theme_service,
    )
}

fn create_chunk() -> CodeChunk {
    CodeChunk {
        content: "fn calculate_total(values: &[i32]) -> i32 {\n    let filtered: Vec<i32> = values.iter().copied().filter(|value| *value > 0).collect();\n    filtered.iter().sum()\n}".to_string(),
        file_path: PathBuf::from("src/sample.rs"),
        start_line: 1,
        end_line: 4,
        language: "rust".to_string(),
        chunk_type: ChunkType::Function,
        name: "calculate_total".to_string(),
        comment_ranges: vec![],
        original_indentation: 0,
    }
}

fn create_context<'a>(
    loading_screen: Option<&'a LoadingScreen>,
    chunks: Option<Vec<CodeChunk>>,
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
        loading_screen,
        challenge_repository,
        current_repo_path: None,
        git_repository,
        scanned_files: None,
        chunks,
        cache_used: false,
        challenge_store,
        repository_store,
        session_store,
        stage_repository: None,
        session_manager: None,
    }
}

#[test]
fn execute_errors_without_chunks() {
    let screen = create_loading_screen();
    let mut context = create_context(Some(&screen), None, None, None, None, None, None);

    let error = GeneratingStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message)
            if message == "No chunks available from ExtractingStep"
    ));
}

#[test]
fn execute_errors_without_loading_screen() {
    let mut context = create_context(
        None,
        Some(vec![create_chunk()]),
        None,
        None,
        None,
        None,
        None,
    );

    let error = GeneratingStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message) if message == "No loading screen available"
    ));
}

#[test]
fn execute_populates_stores_and_saves_generated_challenges() {
    let screen = create_loading_screen();
    let git_repository = git_repository::build();
    let repository = Arc::new(MockChallengeRepository::successful());
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let session_store = Arc::new(SessionStore::new_for_test());
    let mut context = create_context(
        Some(&screen),
        Some(vec![create_chunk()]),
        Some(git_repository.clone()),
        Some(repository.clone() as Arc<dyn ChallengeRepositoryInterface>),
        Some(challenge_store.clone() as Arc<dyn ChallengeStoreInterface>),
        Some(repository_store.clone() as Arc<dyn RepositoryStoreInterface>),
        Some(session_store.clone() as Arc<dyn SessionStoreInterface>),
    );

    let result = GeneratingStep.execute(&mut context).unwrap();
    let generated = challenge_store
        .get_challenges()
        .expect("generated challenges should be stored");

    assert!(matches!(result, StepResult::Skipped));
    assert!(!generated.is_empty());
    assert_eq!(
        repository.save_calls(),
        vec![(git_repository.remote_url.clone(), generated.len())]
    );
    assert_eq!(repository_store.get_repository(), Some(git_repository));
    assert!(session_store.is_loading_completed());
    assert!(generated
        .iter()
        .all(|challenge| challenge.source_file_path.as_deref() == Some("src/sample.rs")));
}

#[test]
fn execute_continues_when_cache_save_fails() {
    let screen = create_loading_screen();
    let git_repository = git_repository::build();
    let repository = Arc::new(MockChallengeRepository::failing("cache save failed"));
    let challenge_store = Arc::new(ChallengeStore::new_for_test());
    let repository_store = Arc::new(RepositoryStore::new_for_test());
    let session_store = Arc::new(SessionStore::new_for_test());
    let mut context = create_context(
        Some(&screen),
        Some(vec![create_chunk()]),
        Some(git_repository.clone()),
        Some(repository.clone() as Arc<dyn ChallengeRepositoryInterface>),
        Some(challenge_store.clone() as Arc<dyn ChallengeStoreInterface>),
        Some(repository_store.clone() as Arc<dyn RepositoryStoreInterface>),
        Some(session_store.clone() as Arc<dyn SessionStoreInterface>),
    );

    let result = GeneratingStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::Skipped));
    assert_eq!(repository.save_calls().len(), 1);
    assert!(challenge_store.get_challenges().is_some());
    assert_eq!(repository_store.get_repository(), Some(git_repository));
    assert!(session_store.is_loading_completed());
}
