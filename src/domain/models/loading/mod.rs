use crate::domain::models::ExtractionOptions;
use crate::domain::models::{Challenge, CodeChunk, GitRepository};
use crate::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use crate::domain::stores::{
    ChallengeStoreInterface, RepositoryStoreInterface, SessionStoreInterface,
};
use crate::presentation::tui::screens::LoadingScreen;
use crate::Result;
use ratatui::style::Color;
use std::path::PathBuf;
use std::sync::Arc;

pub mod cache_check_step;
pub mod cloning_step;
pub mod database_init_step;
pub mod extracting_step;
pub mod finalizing_step;
pub mod generating_step;
pub mod scanning_step;
pub mod step_manager;

use crate::presentation::ui::Colors;
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
    pub loading_screen: Option<&'a LoadingScreen>,
    pub challenge_repository: Option<Arc<dyn ChallengeRepositoryInterface>>,
    pub current_repo_path: Option<PathBuf>,
    pub git_repository: Option<GitRepository>,
    pub scanned_files: Option<Vec<PathBuf>>, // Temporary storage for step results
    pub chunks: Option<Vec<CodeChunk>>,      // Chunks from ExtractingStep
    pub cache_used: bool, // Flag to indicate cache was used and remaining steps should be skipped
    pub challenge_store: Option<Arc<dyn ChallengeStoreInterface>>,
    pub repository_store: Option<Arc<dyn RepositoryStoreInterface>>,
    pub session_store: Option<Arc<dyn SessionStoreInterface>>,
    pub stage_repository: Option<Arc<dyn crate::domain::services::stage_builder_service::StageRepositoryInterface>>,
    pub session_manager: Option<Arc<dyn crate::domain::services::session_manager_service::SessionManagerInterface>>,
}

#[derive(Debug)]
pub enum StepResult {
    RepoPath(PathBuf),
    Challenges(Vec<Challenge>),
    ScannedFiles(Vec<PathBuf>),
    Chunks(Vec<CodeChunk>),
    Skipped,
}

pub trait Step: Send + Sync {
    fn step_type(&self) -> StepType;
    fn step_number(&self) -> usize;
    fn description(&self) -> &str;
    fn step_name(&self) -> &str;
    fn icon(&self, is_current: bool, is_completed: bool, colors: &Colors) -> (&str, Color);
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
