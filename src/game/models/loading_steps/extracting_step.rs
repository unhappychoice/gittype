use super::{ExecutionContext, Step, StepResult, StepType};
use crate::extractor::CodeExtractor;
use crate::ui::Colors;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct ExtractingStep;

impl Step for ExtractingStep {
    fn step_type(&self) -> StepType {
        StepType::Extracting
    }
    fn step_number(&self) -> usize {
        4
    }
    fn description(&self) -> &str {
        "Extracting functions, classes, and code blocks"
    }
    fn step_name(&self) -> &str {
        "Extracting functions, classes, and code blocks"
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

        let options = context.extraction_options.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No extraction options available".to_string())
        })?;

        let screen = context.loading_screen.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No loading screen available".to_string())
        })?;

        if let Some(loader) = &mut context.repository_loader {
            let challenges = loader.load_challenges_from_repository_with_progress(
                repo_path,
                Some(options.clone()),
                screen,
            )?;

            // Set git repository in loading screen if available (for local paths)
            if let Some(git_repository) = loader.get_git_repository() {
                let _ = screen.set_git_repository(git_repository);
            }

            Ok(StepResult::Challenges(challenges))
        } else {
            // Fallback to direct CodeExtractor
            let mut extractor = CodeExtractor::new()?;
            let _chunks =
                extractor.extract_chunks_with_progress(repo_path, options.clone(), screen)?;
            Ok(StepResult::Challenges(vec![])) // Would need conversion logic
        }
    }
}
