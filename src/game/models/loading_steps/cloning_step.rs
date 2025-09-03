use super::{ExecutionContext, Step, StepResult, StepType};
use crate::repository_manager::RepositoryManager;
use crate::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct CloningStep;

impl Step for CloningStep {
    fn step_type(&self) -> StepType {
        StepType::Cloning
    }
    fn step_number(&self) -> usize {
        1
    }
    fn description(&self) -> &str {
        "Cloning repository from remote source"
    }
    fn step_name(&self) -> &str {
        "Cloning repository"
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
        if let Some(repo_spec) = context.repo_spec {
            let repo_info = RepositoryManager::parse_repo_url(repo_spec)?;

            // Clone repository
            let repo_path =
                RepositoryManager::clone_or_update_repo(&repo_info, context.loading_screen)?;

            // Extract actual git info from cloned repository and set it in loading screen
            if let Some(screen) = context.loading_screen {
                if let Ok(Some(git_repository)) =
                    crate::extractor::GitRepositoryExtractor::extract_git_repository(&repo_path)
                {
                    let _ = screen.set_git_repository(&git_repository);
                } else {
                    // Fallback to basic info from RepoInfo if git extraction fails
                    let git_repository = crate::models::GitRepository {
                        user_name: repo_info.owner.clone(),
                        repository_name: repo_info.name.clone(),
                        remote_url: format!(
                            "https://{}/{}/{}",
                            repo_info.origin, repo_info.owner, repo_info.name
                        ),
                        branch: None,
                        commit_hash: None,
                        is_dirty: false,
                    };
                    let _ = screen.set_git_repository(&git_repository);
                }
            }

            Ok(StepResult::RepoPath(repo_path))
        } else {
            // Skip this step if no remote repo specified
            Ok(StepResult::Skipped)
        }
    }
}
