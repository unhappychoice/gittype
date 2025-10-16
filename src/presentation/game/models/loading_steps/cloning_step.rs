use super::{ExecutionContext, Step, StepResult, StepType};
use crate::infrastructure::git::{LocalGitRepositoryClient, RemoteGitRepositoryClient};
use crate::presentation::game::GameData;
use crate::presentation::tui::screens::loading_screen::ProgressReporter;
use crate::presentation::ui::Colors;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct CloningStep;

impl Step for CloningStep {
    fn step_type(&self) -> StepType {
        StepType::Cloning
    }
    fn step_number(&self) -> usize {
        2
    }
    fn description(&self) -> &str {
        "Cloning repository from remote source"
    }
    fn step_name(&self) -> &str {
        "Cloning repository"
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
        ""
    }

    fn format_progress(
        &self,
        _processed: usize,
        _total: usize,
        progress: f64,
        spinner: char,
    ) -> String {
        format!("{} {:.1}%", spinner, progress * 100.0)
    }

    fn execute(&self, context: &mut ExecutionContext) -> Result<StepResult> {
        let Some(repo_spec) = context.repo_spec else {
            return Ok(StepResult::Skipped);
        };

        let progress_callback = |current: usize, total: usize| {
            if let Some(screen) = context.loading_screen {
                screen.set_file_counts(StepType::Cloning, current, total, None);
            }
        };

        let repo_path = RemoteGitRepositoryClient::clone_repository(repo_spec, progress_callback)?;
        context.current_repo_path = Some(repo_path.clone());

        // Extract git repository information after cloning
        let repository = LocalGitRepositoryClient::extract_git_repository(&repo_path)?;
        context.git_repository = Some(repository.clone());
        let _ = GameData::set_git_repository(Some(repository));

        Ok(StepResult::RepoPath(repo_path))
    }
}
