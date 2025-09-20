use super::{ExecutionContext, Step, StepResult, StepType};
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
        5
    }
    fn description(&self) -> &str {
        "Extracting functions, classes, and code blocks"
    }
    fn step_name(&self) -> &str {
        "Extracting functions, classes, and code blocks"
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
        let options = context.extraction_options.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No extraction options available".to_string())
        })?;

        let screen = context.loading_screen.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No loading screen available".to_string())
        })?;

        let loader = context.repository_loader.as_mut().ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No repository loader available".to_string())
        })?;

        let scanned_files = context.scanned_files.as_ref().ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed(
                "No scanned files available from ScanningStep".to_string(),
            )
        })?;

        // Use pre-scanned files for extraction
        let chunks = loader.extract_chunks_from_scanned_files_with_progress(
            scanned_files,
            options,
            screen,
        )?;

        if chunks.is_empty() {
            return Err(crate::GitTypeError::NoSupportedFiles);
        }

        Ok(StepResult::Chunks(chunks))
    }
}
