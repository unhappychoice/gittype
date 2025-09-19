use super::{ExecutionContext, Step, StepResult, StepType};
use crate::cache::CHALLENGE_CACHE;
use crate::ui::Colors;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct CacheCheckStep;

impl Step for CacheCheckStep {
    fn step_type(&self) -> StepType {
        StepType::CacheCheck
    }
    fn step_number(&self) -> usize {
        3
    }
    fn description(&self) -> &str {
        "Checking cache for existing challenges"
    }
    fn step_name(&self) -> &str {
        "Cache check"
    }

    fn icon(&self, is_current: bool, is_completed: bool) -> (&str, Color) {
        if is_completed {
            ("✓", Colors::SUCCESS)
        } else if is_current {
            ("🔍", Colors::WARNING)
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
            "{} {:.1}% {}/{} challenges from cache",
            spinner,
            progress * 100.0,
            processed,
            total
        )
    }

    fn execute(&self, context: &mut ExecutionContext) -> Result<StepResult> {
        // Early return if no git repository info
        let Some(ref git_repo) = context.git_repository else {
            log::info!("No git repository info - skipping cache check");
            return Ok(StepResult::Skipped);
        };

        log::info!(
            "CacheCheckStep - Git repository info: url={}, commit={:?}, is_dirty={}",
            git_repo.remote_url,
            git_repo.commit_hash,
            git_repo.is_dirty
        );

        // Early return if repository is dirty
        if git_repo.is_dirty {
            log::info!(
                "Repository is dirty - skipping cache check for {}",
                git_repo.remote_url
            );
            return Ok(StepResult::Skipped);
        }

        // Try to load from cache
        let Some(cached_challenges) = CHALLENGE_CACHE.load_with_progress(
            git_repo,
            context
                .loading_screen
                .map(|s| s as &dyn crate::game::screens::loading_screen::ProgressReporter),
        ) else {
            log::info!(
                "Cache miss for {} - proceeding with full extraction",
                git_repo.remote_url
            );
            return Ok(StepResult::Skipped);
        };

        // Cache hit! Store challenges and skip remaining steps
        log::info!(
            "Cache hit! Reconstructed {} challenges for {} (clean repository)",
            cached_challenges.len(),
            git_repo.remote_url
        );

        let challenge_count = cached_challenges.len();

        use crate::game::GameData;
        GameData::set_results(cached_challenges, context.git_repository.clone())?;

        // Mark that cache was used so other steps can skip
        context.cache_used = true;
        log::info!(
            "Cache hit: {} challenges loaded from cache",
            challenge_count
        );

        Ok(StepResult::Skipped)
    }
}
