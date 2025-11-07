use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::models::{GitRepository, SessionResult};
use crate::domain::services::session_manager_service::SessionManagerInterface;
use crate::domain::services::theme_service::ThemeServiceInterface;
use crate::domain::services::SessionManager;
use crate::domain::stores::RepositoryStoreInterface;
use crate::presentation::sharing::{SharingPlatform, SharingService};
use crate::presentation::tui::views::{
    ShareBackOptionView, SharePlatformOptionsView, SharePreviewView, ShareTitleView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::{Arc, RwLock};

pub struct SessionSummaryShareData {
    pub session_result: SessionResult,
    pub git_repository: Option<GitRepository>,
}

pub struct SessionSummaryShareDataProvider;

impl ScreenDataProvider for SessionSummaryShareDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

pub trait SessionSummaryShareScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = SessionSummaryShareScreenInterface)]
pub struct SessionSummaryShareScreen {
    #[shaku(default)]
    session_result: RwLock<Option<SessionResult>>,
    #[shaku(default)]
    git_repository: RwLock<Option<GitRepository>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn ThemeServiceInterface>,
    #[shaku(inject)]
    session_manager: Arc<dyn SessionManagerInterface>,
    #[shaku(inject)]
    repository_store: Arc<dyn RepositoryStoreInterface>,
}

impl SessionSummaryShareScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn ThemeServiceInterface>,
        session_manager: Arc<dyn SessionManagerInterface>,
        repository_store: Arc<dyn RepositoryStoreInterface>,
    ) -> Self {
        Self {
            session_result: RwLock::new(None),
            git_repository: RwLock::new(None),
            event_bus,
            theme_service,
            session_manager,
            repository_store,
        }
    }
}

pub struct SessionSummaryShareScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for SessionSummaryShareScreenProvider {
    type Interface = SessionSummaryShareScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: Arc<dyn EventBusInterface> = module.resolve();
        let theme_service: Arc<dyn ThemeServiceInterface> = module.resolve();
        let session_manager: Arc<dyn SessionManagerInterface> = module.resolve();
        let repository_store: Arc<dyn RepositoryStoreInterface> = module.resolve();
        Ok(Box::new(SessionSummaryShareScreen::new(
            event_bus,
            theme_service,
            session_manager,
            repository_store,
        )))
    }
}

impl Screen for SessionSummaryShareScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::SessionSharing
    }

    fn default_provider() -> Box<dyn crate::presentation::tui::ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(SessionSummaryShareDataProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        let (session_result, git_repository) =
            if let Ok(data) = data.downcast::<SessionSummaryShareData>() {
                (Some(data.session_result), data.git_repository)
            } else {
                // If no data provided, get from injected dependencies
                let sm = self
                    .session_manager
                    .as_any()
                    .downcast_ref::<SessionManager>()
                    .ok_or_else(|| {
                        GitTypeError::TerminalError("Failed to get SessionManager".to_string())
                    })?;

                let session_result = sm.get_session_result();
                let git_repository = self.repository_store.get_repository();

                (session_result, git_repository)
            };

        *self.session_result.write().unwrap() = session_result;
        *self.git_repository.write().unwrap() = git_repository;

        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('1') => {
                let session_result = self.session_result.read().unwrap();
                let git_repository = self.git_repository.read().unwrap();
                if let Some(ref session_result) = *session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::X,
                        &git_repository,
                    );
                }
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('2') => {
                let session_result = self.session_result.read().unwrap();
                let git_repository = self.git_repository.read().unwrap();
                if let Some(ref session_result) = *session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::Reddit,
                        &git_repository,
                    );
                }
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('3') => {
                let session_result = self.session_result.read().unwrap();
                let git_repository = self.git_repository.read().unwrap();
                if let Some(ref session_result) = *session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::LinkedIn,
                        &git_repository,
                    );
                }
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('4') => {
                let session_result = self.session_result.read().unwrap();
                let git_repository = self.git_repository.read().unwrap();
                if let Some(ref session_result) = *session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::Facebook,
                        &git_repository,
                    );
                }
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Esc => {
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
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
        let session_result = self.session_result.read().unwrap();
        let git_repository = self.git_repository.read().unwrap();
        if let Some(ref session_result) = *session_result {
            let area = frame.area();

            let content_height = 12;
            let top_spacing = (area.height.saturating_sub(content_height)) / 2;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(top_spacing),
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(4),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ])
                .split(area);

            ShareTitleView::render(frame, chunks[1], &colors);
            SharePreviewView::render(frame, chunks[3], session_result, &git_repository, &colors);
            SharePlatformOptionsView::render(frame, chunks[5], &colors);
            ShareBackOptionView::render(frame, chunks[7], &colors);
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

impl SessionSummaryShareScreenInterface for SessionSummaryShareScreen {}
