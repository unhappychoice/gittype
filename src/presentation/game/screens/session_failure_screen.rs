use crate::domain::events::EventBus;
use crate::domain::models::{GitRepository, SessionResult};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::views::session_failure::{ContentView, FooterView, HeaderView};
use crate::presentation::game::{
    GameData, Screen, ScreenDataProvider, ScreenType, SessionManager, UpdateStrategy,
};
use crate::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::{Arc, Mutex};

pub struct SessionFailureScreenData {
    pub session_result: SessionResult,
    pub total_stages: usize,
    pub repo_info: Option<GitRepository>,
}

pub struct SessionFailureScreenDataProvider {
    session_manager: Arc<Mutex<SessionManager>>,
    game_data: Arc<Mutex<GameData>>,
}

impl ScreenDataProvider for SessionFailureScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_manager = self.session_manager.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("SessionManager lock failed: {}", e))
        })?;

        let session_result = session_manager
            .get_session_result()
            .unwrap_or_else(SessionResult::default);

        let total_stages = session_manager
            .get_stage_info()
            .map(|(_, total)| total)
            .unwrap_or(1);

        let game_data = self.game_data.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!("GameData lock failed: {}", e))
        })?;

        let repo_info = game_data.git_repository.clone();

        Ok(Box::new(SessionFailureScreenData {
            session_result,
            total_stages,
            repo_info,
        }))
    }
}

pub struct SessionFailureScreen {
    session_result: SessionResult,
    total_stages: usize,
    repo_info: Option<GitRepository>,
    event_bus: EventBus,
}

impl SessionFailureScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            session_result: SessionResult::default(),
            total_stages: 1,
            repo_info: None,
            event_bus,
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
        Box::new(SessionFailureScreenDataProvider {
            session_manager: SessionManager::instance(),
            game_data: GameData::instance(),
        })
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        let screen_data = data.downcast::<SessionFailureScreenData>()?;

        self.session_result = screen_data.session_result;
        self.total_stages = screen_data.total_stages;
        self.repo_info = screen_data.repo_info;

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::Typing));
                Ok(())
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::Title));
                Ok(())
            }
            KeyCode::Esc => {
                self.event_bus.publish(NavigateTo::Exit);
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

        HeaderView::render(frame, chunks[1]);
        ContentView::render(frame, chunks[3], &self.session_result, self.total_stages);
        FooterView::render(frame, chunks[5]);

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
