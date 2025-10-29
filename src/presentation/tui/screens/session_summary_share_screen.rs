use crate::domain::events::EventBusInterface;
use crate::domain::models::SessionResult;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::{GameData, SessionManager};
use crate::presentation::sharing::{SharingPlatform, SharingService};
use crate::presentation::tui::views::{
    ShareBackOptionView, SharePlatformOptionsView, SharePreviewView, ShareTitleView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{domain::models::GitRepository, GitTypeError, Result};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::RwLock;
use std::sync::{Arc, Mutex};

pub struct SessionSummaryShareData {
    pub session_result: SessionResult,
    pub git_repository: Option<GitRepository>,
}

pub struct SessionSummaryShareDataProvider {
    session_manager: Arc<Mutex<SessionManager>>,
    game_data: Arc<Mutex<GameData>>,
}

impl ScreenDataProvider for SessionSummaryShareDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = self
            .session_manager
            .lock()
            .map_err(|_| GitTypeError::TerminalError("Failed to lock SessionManager".to_string()))?
            .get_session_result()
            .ok_or_else(|| {
                GitTypeError::TerminalError("No session result available".to_string())
            })?;

        let git_repository = self
            .game_data
            .lock()
            .map_err(|_| GitTypeError::TerminalError("Failed to lock GameData".to_string()))?
            .git_repository
            .clone();

        Ok(Box::new(SessionSummaryShareData {
            session_result,
            git_repository,
        }))
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
}

impl SessionSummaryShareScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        Self {
            session_result: RwLock::new(None),
            git_repository: RwLock::new(None),
            event_bus,
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
        let event_bus: std::sync::Arc<dyn crate::domain::events::EventBusInterface> =
            module.resolve();
        Ok(Box::new(SessionSummaryShareScreen::new(event_bus)))
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
        Box::new(SessionSummaryShareDataProvider {
            session_manager: SessionManager::instance(),
            game_data: GameData::instance(),
        })
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        let data = data.downcast::<SessionSummaryShareData>()?;

        *self.session_result.write().unwrap() = Some(data.session_result);
        *self.git_repository.write().unwrap() = data.git_repository;

        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
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

            ShareTitleView::render(frame, chunks[1]);
            SharePreviewView::render(frame, chunks[3], session_result, &git_repository);
            SharePlatformOptionsView::render(frame, chunks[5]);
            ShareBackOptionView::render(frame, chunks[7]);
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
