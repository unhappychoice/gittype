use crate::extractor::{ExtractionOptions, RepositoryLoader};
use crate::models::Challenge;
use crate::Result;
use ratatui::style::Color;
use std::path::PathBuf;

pub mod cloning_step;
pub mod database_init_step;
pub mod extracting_step;
pub mod finalizing_step;
pub mod generating_step;
pub mod scanning_step;
pub mod step_manager;

pub use cloning_step::CloningStep;
pub use database_init_step::DatabaseInitStep;
pub use extracting_step::ExtractingStep;
pub use finalizing_step::FinalizingStep;
pub use generating_step::GeneratingStep;
pub use scanning_step::ScanningStep;
pub use step_manager::StepManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepType {
    DatabaseInit,
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
    pub loading_screen: Option<&'a crate::game::screens::loading_screen::LoadingScreen>,
    pub repository_loader: Option<&'a mut RepositoryLoader>,
    pub current_repo_path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum StepResult {
    RepoPath(PathBuf),
    Challenges(Vec<Challenge>),
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
