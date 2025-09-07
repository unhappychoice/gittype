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
        3
    }
    fn description(&self) -> &str {
        "Scanning repository files"
    }
    fn step_name(&self) -> &str {
        "Scanning repository"
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

    fn execute(&self, _context: &mut ExecutionContext) -> Result<StepResult> {
        // Scanning is now handled within ExtractingStep
        Ok(StepResult::Skipped)
    }
}
