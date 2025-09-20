use super::{ExecutionContext, Step, StepResult, StepType};
use crate::ui::Colors;
use crate::Result;
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
            ("◦", Colors::muted())
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
                crate::GitTypeError::ExtractionFailed("No repository path available".to_string())
            })?;

        let screen = context.loading_screen.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No loading screen available".to_string())
        })?;

        let loader = context.repository_loader.as_ref().ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No repository loader available".to_string())
        })?;

        // Extract git repository info if not already set (for local paths) and set in GameData
        if context.git_repository.is_none() {
            if let Ok(Some(git_repository)) =
                crate::extractor::GitRepositoryExtractor::extract_git_repository(repo_path)
            {
                // Set git repository info in GameData directly
                let _ = crate::game::GameData::set_git_repository(Some(git_repository.clone()));
                // Also set in context for consistency
                context.git_repository = Some(git_repository);
            }
        }

        // Use RepositoryExtractor to perform file scanning with progress
        let scanned_files = loader.collect_source_files_with_progress(repo_path, screen)?;
        Ok(StepResult::ScannedFiles(scanned_files))
    }
}
