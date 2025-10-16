use crate::domain::events::EventBus;
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

pub struct SessionSummaryShareScreen {
    session_result: Option<SessionResult>,
    git_repository: Option<GitRepository>,
    event_bus: EventBus,
}

impl SessionSummaryShareScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            session_result: None,
            git_repository: None,
            event_bus,
        }
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

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        let data = data.downcast::<SessionSummaryShareData>()?;

        self.session_result = Some(data.session_result);
        self.git_repository = data.git_repository;

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('1') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::X,
                        &self.git_repository,
                    );
                }
                self.event_bus.publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('2') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::Reddit,
                        &self.git_repository,
                    );
                }
                self.event_bus.publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('3') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::LinkedIn,
                        &self.git_repository,
                    );
                }
                self.event_bus.publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('4') => {
                if let Some(ref session_result) = self.session_result {
                    let _ = SharingService::share_result(
                        session_result,
                        SharingPlatform::Facebook,
                        &self.git_repository,
                    );
                }
                self.event_bus.publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Esc => {
                self.event_bus.publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        if let Some(ref session_result) = self.session_result {
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
            SharePreviewView::render(frame, chunks[3], session_result, &self.git_repository);
            SharePlatformOptionsView::render(frame, chunks[5]);
            ShareBackOptionView::render(frame, chunks[7]);
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
