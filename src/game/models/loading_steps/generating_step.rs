use super::{ExecutionContext, Step, StepResult, StepType};
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
            ("✓", Color::Green)
        } else if is_current {
            ("⚡", Color::Yellow)
        } else {
            ("◦", Color::DarkGray)
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

    fn execute(&self, _context: &mut ExecutionContext) -> Result<StepResult> {
        // Challenge generation is handled within the ExtractingStep
        Ok(StepResult::Skipped)
    }
}
