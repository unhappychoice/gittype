use crate::domain::events::EventBus;
use crate::domain::services::scoring::{SessionTracker, StageResult, GLOBAL_SESSION_TRACKER};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::SessionManager;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::tui::screens::ResultAction;
use crate::presentation::tui::views::StageCompletionView;
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use std::sync::{Arc, Mutex};

pub struct StageSummaryData {
    pub stage_result: StageResult,
    pub current_stage: usize,
    pub total_stages: usize,
    pub is_completed: bool,
}

pub struct StageSummaryDataProvider {
    session_tracker: Arc<Mutex<Option<SessionTracker>>>,
    session_manager: Arc<Mutex<SessionManager>>,
}

impl ScreenDataProvider for StageSummaryDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let stage_result = self
            .session_tracker
            .lock()
            .map_err(|e| {
                GitTypeError::TerminalError(format!("Failed to lock session tracker: {}", e))
            })?
            .as_ref()
            .and_then(|t| {
                let data = t.get_data();
                data.stage_results.last().cloned()
            })
            .ok_or_else(|| GitTypeError::TerminalError("No stage result available".to_string()))?;

        let session_manager = self.session_manager.lock().map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to lock SessionManager: {}", e))
        })?;

        let (current_stage, total_stages) = session_manager
            .get_stage_info()
            .map_err(|e| GitTypeError::TerminalError(format!("Failed to get stage info: {}", e)))?;
        let is_completed = session_manager.is_session_completed().map_err(|e| {
            GitTypeError::TerminalError(format!("Failed to check if session completed: {}", e))
        })?;

        Ok(Box::new(StageSummaryData {
            stage_result,
            current_stage,
            total_stages,
            is_completed,
        }))
    }
}

pub struct StageSummaryScreen {
    pub stage_result: Option<StageResult>,
    action_result: Option<ResultAction>,
    session_current_stage: usize,
    total_stages: usize,
    is_completed: bool,
    event_bus: EventBus,
    session_manager: Arc<Mutex<SessionManager>>,
}

impl StageSummaryScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            stage_result: None,
            action_result: None,
            session_current_stage: 1,
            total_stages: 3,
            is_completed: false,
            event_bus,
            session_manager: SessionManager::instance(),
        }
    }

    pub fn with_result(mut self, result: StageResult) -> Self {
        self.stage_result = Some(result);
        self
    }

    pub fn set_stage_result(&mut self, result: StageResult) {
        self.stage_result = Some(result);
    }

    pub fn get_action_result(&self) -> Option<ResultAction> {
        self.action_result.clone()
    }
}

impl Screen for StageSummaryScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::StageSummary
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(StageSummaryDataProvider {
            session_tracker: GLOBAL_SESSION_TRACKER.clone(),
            session_manager: SessionManager::instance(),
        })
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        self.action_result = None;

        let data = data.downcast::<StageSummaryData>()?;

        self.stage_result = Some(data.stage_result);
        self.session_current_stage = data.current_stage;
        self.total_stages = data.total_stages;
        self.is_completed = data.is_completed;

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.action_result = Some(ResultAction::BackToTitle);
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::SessionFailure));
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(ResultAction::Quit);
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::SessionFailure));
                Ok(())
            }
            KeyCode::Char(' ') => {
                let is_session_completed = self
                    .session_manager
                    .lock()
                    .ok()
                    .and_then(|sm| sm.is_session_completed().ok())
                    .unwrap_or(true);

                if !is_session_completed {
                    self.event_bus
                        .publish(NavigateTo::Replace(ScreenType::Typing));
                } else {
                    self.event_bus
                        .publish(NavigateTo::Replace(ScreenType::Animation));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        if let Some(ref stage_result) = self.stage_result {
            // Calculate the stage number that was just completed
            let completed_stage = if self.is_completed {
                // If session is completed, show the total number of stages completed
                self.session_current_stage
            } else {
                // If session is in progress, the current stage has been incremented
                // so we need to show the previous stage number
                self.session_current_stage.saturating_sub(1).max(1)
            };

            let has_next = !self.is_completed;

            StageCompletionView::render(
                frame,
                stage_result,
                completed_stage,
                self.total_stages,
                has_next,
                stage_result.keystrokes,
            );
        }

        Ok(())
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
