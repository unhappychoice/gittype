use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::models::{GitRepository, SessionResult};
use crate::domain::services::session_manager_service::SessionManagerInterface;
use crate::domain::services::SessionManager;
use crate::domain::stores::RepositoryStoreInterface;
use crate::presentation::tui::views::session_failure::{ContentView, FooterView, HeaderView};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::Arc;
use std::sync::RwLock;

pub struct SessionFailureScreenData {
    pub session_result: SessionResult,
    pub total_stages: usize,
    pub repo_info: Option<GitRepository>,
}

pub struct SessionFailureScreenDataProvider;

impl ScreenDataProvider for SessionFailureScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

pub trait SessionFailureScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = SessionFailureScreenInterface)]
pub struct SessionFailureScreen {
    #[shaku(default)]
    session_result: RwLock<SessionResult>,
    #[shaku(default)]
    total_stages: RwLock<usize>,
    #[shaku(default)]
    repo_info: RwLock<Option<GitRepository>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    #[shaku(inject)]
    session_manager: Arc<dyn SessionManagerInterface>,
    #[shaku(inject)]
    repository_store: Arc<dyn RepositoryStoreInterface>,
}

impl SessionFailureScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
        session_manager: Arc<dyn SessionManagerInterface>,
        repository_store: Arc<dyn RepositoryStoreInterface>,
    ) -> Self {
        Self {
            session_result: RwLock::new(SessionResult::default()),
            total_stages: RwLock::new(1),
            repo_info: RwLock::new(None),
            event_bus,
            theme_service,
            session_manager,
            repository_store,
        }
    }
}

impl Screen for SessionFailureScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::SessionFailure
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(SessionFailureScreenDataProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        let (session_result, total_stages, repo_info) = if let Ok(screen_data) =
            data.downcast::<SessionFailureScreenData>()
        {
            (
                screen_data.session_result,
                screen_data.total_stages,
                screen_data.repo_info,
            )
        } else {
            // If no data provided, get from injected dependencies
            let sm = self
                .session_manager
                .as_any()
                .downcast_ref::<SessionManager>()
                .ok_or_else(|| {
                    crate::GitTypeError::TerminalError("Failed to get SessionManager".to_string())
                })?;

            let session_result = sm
                .get_session_result()
                .unwrap_or_else(SessionResult::default);
            let total_stages = sm.get_stage_info().map(|(_, total)| total).unwrap_or(1);
            let repo_info = self.repository_store.get_repository();

            (session_result, total_stages, repo_info)
        };

        *self.session_result.write().unwrap() = session_result;
        *self.total_stages.write().unwrap() = total_stages;
        *self.repo_info.write().unwrap() = repo_info;

        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::Typing));
                Ok(())
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::Title));
                Ok(())
            }
            KeyCode::Esc => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        let area = frame.area();

        // Calculate vertical centering
        let content_height = 10; // header + 3 spacing + content (6 lines) + 1 spacing + nav
        let top_spacing = (area.height.saturating_sub(content_height)) / 2;

        // Create vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(top_spacing), // Top spacing
                Constraint::Length(1),           // Header
                Constraint::Length(3),           // Spacing after header
                Constraint::Length(6), // Content (stage + spacing + metrics*2 + spacing + message)
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Navigation
                Constraint::Min(0),    // Bottom spacing
            ])
            .split(area);

        let session_result = self.session_result.read().unwrap();
        let total_stages = *self.total_stages.read().unwrap();

        HeaderView::render(frame, chunks[1], &colors);
        ContentView::render(frame, chunks[3], &session_result, total_stages, &colors);
        FooterView::render(frame, chunks[5], &colors);

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&self) -> crate::Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl SessionFailureScreenInterface for SessionFailureScreen {}
