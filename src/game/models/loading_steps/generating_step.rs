use super::{ExecutionContext, Step, StepResult, StepType};
use crate::ui::Colors;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct GeneratingStep;

impl Step for GeneratingStep {
    fn step_type(&self) -> StepType {
        StepType::Generating
    }
    fn step_number(&self) -> usize {
        6
    }
    fn description(&self) -> &str {
        "Generating challenges across difficulty levels"
    }
    fn step_name(&self) -> &str {
        "Generating challenges"
    }

    fn icon(&self, is_current: bool, is_completed: bool) -> (&str, Color) {
        if is_completed {
            ("✓", Colors::SUCCESS)
        } else if is_current {
            ("⚡", Colors::WARNING)
        } else {
            ("◦", Colors::MUTED)
        }
    }

    fn supports_progress(&self) -> bool {
        true
    }
    fn progress_unit(&self) -> &str {
        "challenges"
    }

    fn format_progress(
        &self,
        processed: usize,
        total: usize,
        progress: f64,
        spinner: char,
    ) -> String {
        format!(
            "{} {:.1}% {}/{} challenges",
            spinner,
            progress * 100.0,
            processed,
            total
        )
    }

    fn execute(&self, context: &mut ExecutionContext) -> Result<StepResult> {
        let chunks = context.chunks.take().ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed(
                "No chunks available from ExtractingStep".to_string(),
            )
        })?;

        let file_paths = context.scanned_files.take().ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed(
                "No scanned files available from ScanningStep".to_string(),
            )
        })?;

        let screen = context.loading_screen.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No loading screen available".to_string())
        })?;

        let loader = context.repository_loader.as_ref().ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No repository loader available".to_string())
        })?;

        // Get git_root from GitRepository if available
        let git_root = context
            .git_repository
            .as_ref()
            .and_then(|repo| repo.root_path.as_ref())
            .map(|path| path.as_path());

        // Convert both chunks and files with unified progress tracking (using move, not clone)
        let challenges = loader.convert_chunks_and_files_to_challenges_with_progress(
            chunks, file_paths, git_root, screen,
        );

        // Store challenges in GameData (using move, not clone)
        use crate::game::GameData;
        GameData::set_results(challenges, context.git_repository.take())?;

        Ok(StepResult::Skipped)
    }
}
