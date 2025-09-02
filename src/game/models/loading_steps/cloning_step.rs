use super::{ExecutionContext, Step, StepResult, StepType};
use crate::repo_manager::RepoManager;
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
            match RepoManager::parse_repo_url(repo_spec)? {
                repo_info => {
                    let repo_display = format!(
                        "Repository: {}/{} ({})",
                        repo_info.owner, repo_info.name, repo_info.origin
                    );

                    // Set repo info in LoadingScreen
                    if let Some(screen) = context.loading_screen {
                        let _ = screen.set_repo_info(repo_display);
                    }

                    // Clone repository
                    let repo_path =
                        RepoManager::clone_or_update_repo(&repo_info, context.loading_screen)?;
                    Ok(StepResult::RepoPath(repo_path))
                }
            }
        } else {
            // Skip this step if no remote repo specified
            Ok(StepResult::Skipped)
        }
    }
}
