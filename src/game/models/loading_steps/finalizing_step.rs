use super::{ExecutionContext, Step, StepResult, StepType};
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct FinalizingStep;

impl Step for FinalizingStep {
    fn step_type(&self) -> StepType {
        StepType::Finalizing
    }
    fn step_number(&self) -> usize {
        6
    }
    fn description(&self) -> &str {
        "Preparing content for optimal typing practice"
    }
    fn step_name(&self) -> &str {
        "Finalizing"
    }

    fn icon(&self, is_current: bool, is_completed: bool) -> (&str, Color) {
        if is_completed {
            ("✓", Color::Green)
        } else if is_current {
            ("⚡", Color::Yellow)
        } else {
            ("◦", Color::DarkGray)
        }
    }

    fn supports_progress(&self) -> bool {
        false
    }
    fn progress_unit(&self) -> &str {
        ""
    }

    fn format_progress(
        &self,
        _processed: usize,
        _total: usize,
        _progress: f64,
        _spinner: char,
    ) -> String {
        "Finalizing...".to_string()
    }

    fn execute(&self, _context: &mut ExecutionContext) -> Result<StepResult> {
        // Finalization steps - no-op for now
        Ok(StepResult::Skipped)
    }
}
