use gittype::domain::events::EventBus;
use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::loading::{ExecutionContext, ExtractingStep, Step};
use gittype::domain::models::theme::Theme;
use gittype::domain::models::{Challenge, ExtractionOptions, GitRepository};
use gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use gittype::domain::services::theme_service::{ThemeService, ThemeServiceInterface};
use gittype::presentation::tui::screens::loading_screen::{LoadingScreen, ProgressReporter};
use gittype::{GitTypeError, Result};
use std::path::PathBuf;
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
    extraction_options: Option<&'a ExtractionOptions>,
    loading_screen: Option<&'a LoadingScreen>,
    scanned_files: Option<Vec<PathBuf>>,
) -> ExecutionContext<'a> {
    ExecutionContext {
        repo_spec: None,
        repo_path: None,
        extraction_options,
        loading_screen,
        challenge_repository: None,
        current_repo_path: None,
        git_repository: None,
        scanned_files,
        chunks: None,
        cache_used: false,
        challenge_store: None,
        repository_store: None,
        session_store: None,
        stage_repository: None,
        session_manager: None,
    }
}

fn fixture_path(file_name: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join(file_name)
}

#[test]
fn execute_errors_without_extraction_options() {
    let mut context = create_context(None, None, None);

    let error = ExtractingStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message) if message == "No extraction options available"
    ));
}

#[test]
fn execute_errors_without_loading_screen() {
    let options = ExtractionOptions::default();
    let mut context = create_context(Some(&options), None, None);

    let error = ExtractingStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message) if message == "No loading screen available"
    ));
}

#[test]
fn execute_errors_without_scanned_files() {
    let options = ExtractionOptions::default();
    let screen = create_loading_screen();
    let mut context = create_context(Some(&options), Some(&screen), None);

    let error = ExtractingStep.execute(&mut context).unwrap_err();

    assert!(matches!(
        error,
        GitTypeError::ExtractionFailed(message)
            if message == "No scanned files available from ScanningStep"
    ));
}

#[test]
fn execute_returns_no_supported_files_when_all_files_are_filtered_out() {
    let file_path = fixture_path("complex_commented_rust.rs");
    let screen = create_loading_screen();
    let options = ExtractionOptions {
        max_file_size_bytes: 0,
        ..ExtractionOptions::default()
    };
    let mut context = create_context(Some(&options), Some(&screen), Some(vec![file_path]));

    let error = ExtractingStep.execute(&mut context).unwrap_err();

    assert!(matches!(error, GitTypeError::NoSupportedFiles));
}

#[test]
fn execute_errors_when_scanned_files_have_no_supported_language() {
    let file_path = std::env::current_dir().unwrap().join("Cargo.toml");
    let screen = create_loading_screen();
    let options = ExtractionOptions::default();
    let mut context = create_context(Some(&options), Some(&screen), Some(vec![file_path]));

    let error = ExtractingStep.execute(&mut context).unwrap_err();

    assert!(matches!(error, GitTypeError::ExtractionFailed(_)));
}

#[test]
fn execute_errors_when_scanned_file_has_no_extension() {
    let file_path = std::env::current_dir().unwrap().join("README");
    let screen = create_loading_screen();
    let options = ExtractionOptions::default();
    let mut context = create_context(Some(&options), Some(&screen), Some(vec![file_path]));

    let error = ExtractingStep.execute(&mut context).unwrap_err();

    assert!(matches!(error, GitTypeError::ExtractionFailed(_)));
}
