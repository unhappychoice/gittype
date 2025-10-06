use super::{ExecutionContext, Step, StepResult, StepType};
use crate::domain::repositories::challenge_repository::CHALLENGE_REPOSITORY;
use crate::domain::services::challenge_generator::ChallengeGenerator;
use crate::presentation::game::GameData;
use crate::presentation::ui::Colors;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct GeneratingStep;

impl Step for GeneratingStep {
    fn step_type(&self) -> StepType {
        StepType::Generating
    }
    fn step_number(&self) -> usize {
        7
    }
    fn description(&self) -> &str {
        "Generating challenges across difficulty levels"
    }
    fn step_name(&self) -> &str {
        "Generating challenges"
    }

    fn icon(&self, is_current: bool, is_completed: bool) -> (&str, Color) {
        if is_completed {
            ("✓", Colors::success())
        } else if is_current {
            ("⚡", Colors::warning())
        } else {
            ("◦", Colors::text_secondary())
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

        let screen = context.loading_screen.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No loading screen available".to_string())
        })?;

        let converter = ChallengeGenerator::new();
        let generated_challenges = converter.convert_with_progress(chunks, screen);

        // Cache the generated challenges if we have git repository info
        if let Some(ref git_repo) = context.git_repository {
            match CHALLENGE_REPOSITORY.save_challenges(git_repo, &generated_challenges) {
                Ok(_) => {
                    log::info!(
                        "Successfully cached {} challenges for {}",
                        generated_challenges.len(),
                        git_repo.remote_url
                    );
                }
                Err(e) => {
                    log::warn!("Failed to cache challenges: {}", e);
                }
            }
        }

        // Store challenges in GameData
        GameData::set_results(generated_challenges, context.git_repository.take())?;

        Ok(StepResult::Skipped)
    }
}
