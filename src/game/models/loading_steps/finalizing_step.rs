use super::{ExecutionContext, Step, StepResult, StepType};
use crate::game::session_manager::{SessionConfig, SessionManager};
use crate::game::stage_repository::StageRepository;
use crate::game::DifficultyLevel;
use crate::game::GameData;
use crate::scoring::tracker::{SessionTracker, TotalTracker};
use crate::ui::Colors;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct FinalizingStep;

impl Step for FinalizingStep {
    fn step_type(&self) -> StepType {
        StepType::Finalizing
    }
    fn step_number(&self) -> usize {
        8
    }
    fn description(&self) -> &str {
        "Preparing content for optimal typing practice"
    }
    fn step_name(&self) -> &str {
        "Finalizing"
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

    fn execute(&self, context: &mut ExecutionContext) -> Result<StepResult> {
        let git_repository = context
            .git_repository
            .as_ref()
            .cloned()
            .or_else(GameData::get_git_repository);

        // Verify challenges are available in GameData
        let challenge_count = GameData::with_challenges(|c| c.len()).unwrap_or(0);

        if challenge_count == 0 {
            return Err(crate::GitTypeError::ExtractionFailed(
                "No challenges available for finalization".to_string(),
            ));
        }

        // Initialize StageRepository (no longer requires challenges ownership)
        StageRepository::initialize_global(git_repository.clone())?;

        // Build difficulty indices for optimal performance
        StageRepository::build_global_difficulty_indices()?;

        // Update title screen with challenge data (done once during initialization)
        StageRepository::update_global_title_screen_data()?;

        // Initialize SessionManager
        SessionManager::reset_global_session()?;
        let session_config = SessionConfig {
            max_stages: 3,
            session_timeout: None,
            difficulty: DifficultyLevel::Normal,
            max_skips: 3,
        };
        SessionManager::initialize_session(Some(session_config))?;
        SessionManager::set_git_repository(git_repository)?;
        SessionManager::start_global_session()?;

        // Initialize global trackers for Ctrl+C handler
        SessionTracker::initialize_global_instance(SessionTracker::new());
        TotalTracker::initialize_global_instance(TotalTracker::new());


        Ok(StepResult::Skipped)
    }
}
