use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::screens::ResultAction;
use crate::game::views::stage_summary::StageCompletionView;
use crate::game::{ScreenType, SessionManager};
use crate::scoring::StageResult;
use crate::Result;

pub struct StageSummaryScreen {
    pub stage_result: Option<StageResult>,
    action_result: Option<ResultAction>,
}

impl Default for StageSummaryScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl StageSummaryScreen {
    pub fn new() -> Self {
        Self {
            stage_result: None,
            action_result: None,
        }
    }

    pub fn with_result(mut self, result: StageResult) -> Self {
        self.stage_result = Some(result);
        self
    }

    pub fn get_action_result(&self) -> Option<ResultAction> {
        self.action_result.clone()
    }
}

impl Screen for StageSummaryScreen {
    fn init(&mut self) -> Result<()> {
        self.action_result = None;
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Esc => {
                self.action_result = Some(ResultAction::BackToTitle);
                Ok(ScreenTransition::Replace(ScreenType::SessionFailure))
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(ResultAction::Quit);
                Ok(ScreenTransition::Replace(ScreenType::SessionFailure))
            }
            KeyCode::Char(' ') => {
                let is_session_completed =
                    SessionManager::is_global_session_completed().unwrap_or(true);

                if !is_session_completed {
                    Ok(ScreenTransition::Replace(ScreenType::Typing))
                } else {
                    Ok(ScreenTransition::Replace(ScreenType::Animation))
                }
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut std::io::Stdout,
        _session_result: Option<&crate::domain::models::SessionResult>,
        _total_result: Option<&crate::scoring::TotalResult>,
    ) -> Result<()> {
        if let Some(ref stage_result) = self.stage_result {
            let (session_current_stage, total_stages) =
                SessionManager::get_global_stage_info().unwrap_or((1, 3));
            let is_completed = SessionManager::is_global_session_completed().unwrap_or(true);

            // Calculate the stage number that was just completed
            let completed_stage = if is_completed {
                // If session is completed, show the total number of stages completed
                session_current_stage
            } else {
                // If session is in progress, the current stage has been incremented
                // so we need to show the previous stage number
                session_current_stage.saturating_sub(1).max(1)
            };

            let has_next = !is_completed;

            StageCompletionView::render_complete(
                stage_result,
                completed_stage,
                total_stages,
                has_next,
                stage_result.keystrokes,
            )?;

            Ok(())
        } else {
            Ok(())
        }
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
