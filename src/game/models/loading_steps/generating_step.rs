use super::{ExecutionContext, Step, StepResult, StepType};
use crate::cache::CHALLENGE_CACHE;
use crate::ui::Colors;
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

        let loader = context.repository_loader.as_ref().ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No repository loader available".to_string())
        })?;

        // Generate challenges
        let generated_challenges = loader.convert_chunks_and_files_to_challenges_with_progress(
            chunks,
            vec![],
            None,
            screen,
        );

        // Cache the generated challenges if we have git repository info
        if let Some(ref git_repo) = context.git_repository {
            match CHALLENGE_CACHE.save(git_repo, &generated_challenges) {
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
        use crate::game::GameData;
        GameData::set_results(generated_challenges, context.git_repository.take())?;

        Ok(StepResult::Skipped)
    }
}
