use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::services::scoring::StageResult;
use crate::domain::services::session_manager_service::SessionManagerInterface;
use crate::domain::services::SessionManager;
use crate::presentation::tui::screens::ResultAction;
use crate::presentation::tui::views::StageCompletionView;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use std::sync::{Arc, RwLock};

pub struct StageSummaryData {
    pub stage_result: StageResult,
    pub current_stage: usize,
    pub total_stages: usize,
    pub is_completed: bool,
}

pub struct StageSummaryDataProvider;

impl ScreenDataProvider for StageSummaryDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

pub trait StageSummaryScreenInterface: Screen {}

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
    #[shaku(inject)]
    session_manager: Arc<dyn SessionManagerInterface>,
}

impl StageSummaryScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
        session_manager: Arc<dyn SessionManagerInterface>,
    ) -> Self {
        Self {
            stage_result: RwLock::new(None),
            action_result: RwLock::new(None),
            session_current_stage: RwLock::new(1),
            total_stages: RwLock::new(3),
            is_completed: RwLock::new(false),
            event_bus,
            theme_service,
            session_manager,
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
        let event_bus: Arc<dyn EventBusInterface> = module.resolve();
        let theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface> =
            module.resolve();
        let session_manager: Arc<dyn SessionManagerInterface> = module.resolve();
        Ok(Box::new(StageSummaryScreen::new(
            event_bus,
            theme_service,
            session_manager,
        )))
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
        Box::new(StageSummaryDataProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        *self.action_result.write().unwrap() = None;

        let (stage_result, current_stage, total_stages, is_completed) =
            if let Ok(data) = data.downcast::<StageSummaryData>() {
                (
                    Some(data.stage_result),
                    data.current_stage,
                    data.total_stages,
                    data.is_completed,
                )
            } else {
                // If no data provided, get from injected dependencies
                let sm = self
                    .session_manager
                    .as_any()
                    .downcast_ref::<SessionManager>()
                    .ok_or_else(|| {
                        GitTypeError::TerminalError("Failed to get SessionManager".to_string())
                    })?;

                let stage_result = sm.get_stage_results().last().cloned();
                let (current_stage, total_stages) = sm.get_stage_info().unwrap_or((1, 3));
                let is_completed = sm.is_session_completed().unwrap_or(false);

                (stage_result, current_stage, total_stages, is_completed)
            };

        *self.stage_result.write().unwrap() = stage_result;
        *self.session_current_stage.write().unwrap() = current_stage;
        *self.total_stages.write().unwrap() = total_stages;
        *self.is_completed.write().unwrap() = is_completed;

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
                    .as_any()
                    .downcast_ref::<SessionManager>()
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
