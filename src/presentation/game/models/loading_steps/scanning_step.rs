use super::{ExecutionContext, Step, StepResult, StepType};
use crate::domain::services::source_file_extractor::SourceFileExtractor;
use crate::infrastructure::git::LocalGitRepositoryClient;
use crate::presentation::ui::Colors;
use crate::{GitTypeError, Result};
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct ScanningStep;

impl Step for ScanningStep {
    fn step_type(&self) -> StepType {
        StepType::Scanning
    }
    fn step_number(&self) -> usize {
        4
    }
    fn description(&self) -> &str {
        "Scanning repository files"
    }
    fn step_name(&self) -> &str {
        "Scanning repository"
    }

    fn icon(&self, is_current: bool, is_completed: bool) -> (&str, Color) {
        if is_completed {
            ("✓", Colors::success())
        } else if is_current {
            ("⚡", Colors::warning())
        } else {
            ("◦", Colors::text_secondary())
        }
    }

    fn supports_progress(&self) -> bool {
        true
    }
    fn progress_unit(&self) -> &str {
        "files"
    }

    fn format_progress(
        &self,
        processed: usize,
        total: usize,
        progress: f64,
        spinner: char,
    ) -> String {
        format!(
            "{} {:.1}% {}/{} files",
            spinner,
            progress * 100.0,
            processed,
            total
        )
    }

    fn execute(&self, context: &mut ExecutionContext) -> Result<StepResult> {
        let repo_path = context
            .current_repo_path
            .as_ref()
            .or(context.repo_path)
            .ok_or_else(|| {
                GitTypeError::ExtractionFailed("No repository path available".to_string())
            })?;

        // If git_repository is not set yet, try to create it from repo_path
        if context.git_repository.is_none() {
            if let Some(repo_path) = context.repo_path {
                context.git_repository =
                    LocalGitRepositoryClient::create_from_local_path(repo_path).ok();
            }
        }

        let screen = context.loading_screen.ok_or_else(|| {
            GitTypeError::ExtractionFailed("No loading screen available".to_string())
        })?;

        SourceFileExtractor::new()
            .collect_with_progress(repo_path, screen)
            .map(StepResult::ScannedFiles)
    }
}
