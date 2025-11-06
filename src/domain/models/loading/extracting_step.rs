use super::{ExecutionContext, Step, StepResult, StepType};
use crate::domain::models::{Language, Languages};
use crate::domain::services::source_code_parser::SourceCodeParser;
use crate::presentation::ui::Colors;
use crate::{GitTypeError, Result};
use ratatui::style::Color;
use std::path::PathBuf;

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

    fn icon(&self, is_current: bool, is_completed: bool, colors: &Colors) -> (&str, Color) {
        if is_completed {
            ("✓", colors.success())
        } else if is_current {
            ("⚡", colors.warning())
        } else {
            ("◦", colors.text_secondary())
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
            GitTypeError::ExtractionFailed("No extraction options available".to_string())
        })?;

        let screen = context.loading_screen.ok_or_else(|| {
            GitTypeError::ExtractionFailed("No loading screen available".to_string())
        })?;

        let scanned_files = context.scanned_files.as_ref().ok_or_else(|| {
            GitTypeError::ExtractionFailed(
                "No scanned files available from ScanningStep".to_string(),
            )
        })?;

        let mut extractor = SourceCodeParser::new()?;
        let files_to_process: Vec<(PathBuf, Box<dyn Language>)> = scanned_files
            .iter()
            .filter_map(|path| {
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    Languages::from_extension(extension).map(|language| (path.to_owned(), language))
                } else {
                    None
                }
            })
            .collect();

        let chunks = extractor.extract_chunks_with_progress(files_to_process, options, screen)?;

        if chunks.is_empty() {
            return Err(GitTypeError::NoSupportedFiles);
        }

        Ok(StepResult::Chunks(chunks))
    }
}
