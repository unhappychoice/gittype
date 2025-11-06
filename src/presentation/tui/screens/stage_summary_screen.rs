use crate::domain::events::EventBusInterface;
use crate::domain::services::scoring::{SessionTracker, StageResult, GLOBAL_SESSION_TRACKER};
use crate::domain::events::presentation_events::NavigateTo;
use crate::presentation::game::SessionManager;
use crate::presentation::tui::screens::ResultAction;
use crate::presentation::tui::views::StageCompletionView;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use std::sync::{Arc, Mutex, RwLock};

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

pub trait StageSummaryScreenInterface: Screen {}

#[derive(Clone)]
pub struct SessionManagerRef(Arc<Mutex<SessionManager>>);

impl Default for SessionManagerRef {
    fn default() -> Self {
        Self(SessionManager::instance())
    }
}

#[derive(shaku::Component)]
#[shaku(interface = StageSummaryScreenInterface)]
pub struct StageSummaryScreen {
    #[shaku(default)]
    pub stage_result: RwLock<Option<StageResult>>,
    #[shaku(default)]
    action_result: RwLock<Option<ResultAction>>,
    #[shaku(default)]
    session_current_stage: RwLock<usize>,
    #[shaku(default)]
    total_stages: RwLock<usize>,
    #[shaku(default)]
    is_completed: RwLock<bool>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    #[shaku(default)]
    session_manager: SessionManagerRef,
}

impl StageSummaryScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    ) -> Self {
        Self {
            stage_result: RwLock::new(None),
            action_result: RwLock::new(None),
            session_current_stage: RwLock::new(1),
            total_stages: RwLock::new(3),
            is_completed: RwLock::new(false),
            event_bus,
            theme_service,
            session_manager: SessionManagerRef::default(),
        }
    }

    pub fn with_result(self, result: StageResult) -> Self {
        *self.stage_result.write().unwrap() = Some(result);
        self
    }

    pub fn set_stage_result(&self, result: StageResult) {
        *self.stage_result.write().unwrap() = Some(result);
    }

    pub fn get_action_result(&self) -> Option<ResultAction> {
        self.action_result.read().unwrap().clone()
    }
}

pub struct StageSummaryScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for StageSummaryScreenProvider {
    type Interface = StageSummaryScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: std::sync::Arc<dyn crate::domain::events::EventBusInterface> =
            module.resolve();
        let theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface> =
            module.resolve();
        Ok(Box::new(StageSummaryScreen::new(event_bus, theme_service)))
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

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        *self.action_result.write().unwrap() = None;

        let data = data.downcast::<StageSummaryData>()?;

        *self.stage_result.write().unwrap() = Some(data.stage_result);
        *self.session_current_stage.write().unwrap() = data.current_stage;
        *self.total_stages.write().unwrap() = data.total_stages;
        *self.is_completed.write().unwrap() = data.is_completed;

        Ok(())
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                *self.action_result.write().unwrap() = Some(ResultAction::BackToTitle);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::SessionFailure));
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                *self.action_result.write().unwrap() = Some(ResultAction::Quit);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::SessionFailure));
                Ok(())
            }
            KeyCode::Char(' ') => {
                let is_session_completed = self
                    .session_manager
                    .0
                    .lock()
                    .ok()
                    .and_then(|sm| sm.is_session_completed().ok())
                    .unwrap_or(true);

                if !is_session_completed {
                    self.event_bus
                        .as_event_bus()
                        .publish(NavigateTo::Replace(ScreenType::Typing));
                } else {
                    self.event_bus
                        .as_event_bus()
                        .publish(NavigateTo::Replace(ScreenType::Animation));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        let stage_result = self.stage_result.read().unwrap();
        if let Some(ref stage_result) = *stage_result {
            let is_completed = *self.is_completed.read().unwrap();
            let session_current_stage = *self.session_current_stage.read().unwrap();
            let total_stages = *self.total_stages.read().unwrap();

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

            StageCompletionView::render(
                frame,
                stage_result,
                completed_stage,
                total_stages,
                has_next,
                stage_result.keystrokes,
                &colors,
            );
        }

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&self) -> Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl StageSummaryScreenInterface for StageSummaryScreen {}
