use git2::Repository;
use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::loading::{ExecutionContext, ScanningStep, Step, StepResult};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::{Challenge, GitRepository};
use gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::loading_screen::{LoadingScreen, ProgressReporter};
use gittype::{GitTypeError, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;

struct StubChallengeRepository;

impl ChallengeRepositoryInterface for StubChallengeRepository {
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
        Arc::new(StubChallengeRepository),
        theme_service,
    )
}

fn create_context<'a>(
    repo_path: Option<&'a PathBuf>,
    current_repo_path: Option<PathBuf>,
    loading_screen: Option<&'a LoadingScreen>,
) -> ExecutionContext<'a> {
    ExecutionContext {
        repo_spec: None,
        repo_path,
        extraction_options: None,
        loading_screen,
        challenge_repository: None,
        current_repo_path,
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

fn create_git_repository(path: &Path) {
    Repository::init(path).unwrap();
}

#[test]
fn execute_errors_without_repository_path() {
    let mut context = create_context(None, None, None);

    let error = ScanningStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message) if message == "No repository path available"
    ));
}

#[test]
fn execute_errors_without_loading_screen() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path().to_path_buf();
    create_git_repository(&repo_path);
    let mut context = create_context(Some(&repo_path), None, None);

    let error = ScanningStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message) if message == "No loading screen available"
    ));
    assert!(matches!(
        context.git_repository.as_ref(),
        Some(repo)
            if repo.user_name == "local"
                && repo.repository_name == temp_dir.path().file_name().unwrap().to_str().unwrap()
                && repo.root_path.as_ref() == Some(&repo_path)
    ));
}

#[test]
fn execute_returns_empty_scanned_files_with_mock_file_storage() {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path().to_path_buf();
    create_git_repository(&repo_path);
    let screen = create_loading_screen();
    let mut context = create_context(Some(&repo_path), None, Some(&screen));

    let result = ScanningStep.execute(&mut context).unwrap();

    assert!(matches!(result, StepResult::ScannedFiles(files) if files.is_empty()));
    assert!(matches!(
        context.git_repository.as_ref(),
        Some(repo)
            if repo.user_name == "local"
                && repo.repository_name == temp_dir.path().file_name().unwrap().to_str().unwrap()
                && repo.root_path.as_ref() == Some(&repo_path)
    ));
}

#[test]
fn execute_propagates_file_storage_errors() {
    let missing_path = PathBuf::from("/nonexistent/path");
    let screen = create_loading_screen();
    let mut context = create_context(None, Some(missing_path), Some(&screen));

    let error = ScanningStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message) if message.contains("Path does not exist")
    ));
}
