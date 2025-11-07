use super::{ExecutionContext, Step, StepResult, StepType};
use crate::domain::models::{DifficultyLevel, SessionConfig, SessionState};
use crate::domain::services::stage_builder_service::StageRepository;
use crate::domain::services::SessionManager;
use crate::infrastructure::git::LocalGitRepositoryClient;
use crate::presentation::ui::Colors;
use crate::{GitTypeError, Result};
use ratatui::style::Color;
use std::time::Instant;

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
        let git_repository = context.git_repository.as_ref().cloned().or_else(|| {
            context
                .current_repo_path
                .as_ref()
                .or(context.repo_path)
                .and_then(|path| {
                    LocalGitRepositoryClient::new()
                        .create_from_local_path(path)
                        .ok()
                })
        });

        // Get stores from context
        let challenge_store = context.challenge_store.clone().ok_or_else(|| {
            GitTypeError::TerminalError("ChallengeStore not available".to_string())
        })?;

        // Verify challenges are available
        let challenge_count = challenge_store
            .get_challenges()
            .map(|c| c.len())
            .unwrap_or(0);

        if challenge_count == 0 {
            return Err(GitTypeError::ExtractionFailed(
                "No challenges available for finalization".to_string(),
            ));
        }

        // Initialize StageRepository: build difficulty indices for optimal performance
        if let Some(stage_repository) = &context.stage_repository {
            // Downcast to concrete type to call build_difficulty_indices
            if let Some(concrete_stage_repo) =
                stage_repository.as_any().downcast_ref::<StageRepository>()
            {
                concrete_stage_repo.build_difficulty_indices();
            }
        } else {
            log::warn!("StageRepository not available in context, skipping difficulty index build");
        }

        // Initialize SessionManager
        if let Some(session_manager) = &context.session_manager {
            // Downcast to concrete type to access methods
            if let Some(concrete_session_manager) =
                session_manager.as_any().downcast_ref::<SessionManager>()
            {
                // Reset session to clean state
                concrete_session_manager.reset();

                // Set session configuration
                let session_config = SessionConfig {
                    max_stages: 3,
                    session_timeout: None,
                    difficulty: DifficultyLevel::Normal,
                    max_skips: 3,
                };
                concrete_session_manager.set_config(session_config);

                // Set git repository context
                concrete_session_manager.set_git_repository(git_repository);

                // Start session by setting state to InProgress
                concrete_session_manager.set_state(SessionState::InProgress {
                    current_stage: 0,
                    started_at: Instant::now(),
                });
            }
        } else {
            log::warn!("SessionManager not available in context, skipping session initialization");
        }

        // Tracker instances are now managed via DI - no need to initialize global instances

        Ok(StepResult::Skipped)
    }
}
