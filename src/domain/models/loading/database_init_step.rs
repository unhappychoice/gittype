use super::{ExecutionContext, Step, StepResult, StepType};
use crate::domain::repositories::SessionRepository;
use crate::infrastructure::database::database::Database;
use crate::presentation::ui::Colors;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct DatabaseInitStep;

impl Step for DatabaseInitStep {
    fn step_type(&self) -> StepType {
        StepType::DatabaseInit
    }
    fn step_number(&self) -> usize {
        1
    }
    fn description(&self) -> &str {
        "Initializing database and session recording"
    }
    fn step_name(&self) -> &str {
        "Database Setup"
    }

    fn icon(&self, is_current: bool, is_completed: bool, colors: &Colors) -> (&str, Color) {
        if is_completed {
            ("✓", colors.success())
        } else if is_current {
            ("💾", colors.warning())
        } else {
            ("◦", colors.text_secondary())
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
        "Initializing database...".to_string()
    }

    fn execute(&self, _context: &mut ExecutionContext) -> Result<StepResult> {
        log::info!("DatabaseInitStep: Starting database initialization");

        // Initialize database with migrations
        let database = Database::new()?;
        database.init()?;
        log::info!("DatabaseInitStep: Database initialized successfully");

        // Initialize global session repository
        if let Err(e) = SessionRepository::initialize_global() {
            log::error!(
                "DatabaseInitStep: Failed to initialize global session repository: {}",
                e
            );
            return Err(e);
        } else {
            log::info!("DatabaseInitStep: Global session repository initialized successfully");
        }

        Ok(StepResult::Skipped)
    }
}
