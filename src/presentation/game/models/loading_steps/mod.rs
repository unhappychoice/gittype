use crate::domain::models::ExtractionOptions;
use crate::domain::services::extractor::RepositoryExtractor;
use crate::domain::models::Challenge;
use crate::Result;
use ratatui::style::Color;
use std::path::PathBuf;

pub mod cache_check_step;
pub mod cloning_step;
pub mod database_init_step;
pub mod extracting_step;
pub mod finalizing_step;
pub mod generating_step;
pub mod scanning_step;
pub mod step_manager;

pub use cache_check_step::CacheCheckStep;
pub use cloning_step::CloningStep;
pub use database_init_step::DatabaseInitStep;
pub use extracting_step::ExtractingStep;
pub use finalizing_step::FinalizingStep;
pub use generating_step::GeneratingStep;
pub use scanning_step::ScanningStep;
pub use step_manager::StepManager;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StepType {
    DatabaseInit,
    CacheCheck,
    Cloning,
    Scanning,
    Extracting,
    Generating,
    Finalizing,
    Completed,
}

// Execution context passed to steps
pub struct ExecutionContext<'a> {
    pub repo_spec: Option<&'a str>,
    pub repo_path: Option<&'a PathBuf>,
    pub extraction_options: Option<&'a ExtractionOptions>,
    pub loading_screen: Option<&'a crate::presentation::game::screens::loading_screen::LoadingScreen>,
    pub repository_loader: Option<&'a mut RepositoryExtractor>,
    pub current_repo_path: Option<PathBuf>,
    pub git_repository: Option<crate::domain::models::GitRepository>,
    pub scanned_files: Option<Vec<PathBuf>>, // Temporary storage for step results
    pub chunks: Option<Vec<crate::domain::models::CodeChunk>>, // Chunks from ExtractingStep
    pub cache_used: bool, // Flag to indicate cache was used and remaining steps should be skipped
}

#[derive(Debug)]
pub enum StepResult {
    RepoPath(PathBuf),
    Challenges(Vec<Challenge>),
    ScannedFiles(Vec<PathBuf>),
    Chunks(Vec<crate::domain::models::CodeChunk>),
    Skipped,
}

pub trait Step: Send + Sync {
    fn step_type(&self) -> StepType;
    fn step_number(&self) -> usize;
    fn description(&self) -> &str;
    fn step_name(&self) -> &str;
    fn icon(&self, is_current: bool, is_completed: bool) -> (&str, Color);
    fn supports_progress(&self) -> bool;
    fn progress_unit(&self) -> &str;
    fn format_progress(
        &self,
        processed: usize,
        total: usize,
        progress: f64,
        spinner: char,
    ) -> String;

    // Execution logic
    fn execute(&self, context: &mut ExecutionContext) -> Result<StepResult>;
    fn can_skip(&self, _context: &ExecutionContext) -> bool {
        false
    }
}
