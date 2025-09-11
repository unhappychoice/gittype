use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::views::{BestRecordsView, ControlsView, HeaderView, StageResultsView};
use crate::game::{GameData, SessionManager};
use crate::{models::GitRepository, Result};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::io::Stdout;

pub struct SessionDetailsDialog {
    session_result: Option<crate::models::SessionResult>,
    repo_info: Option<GitRepository>,
}

impl Default for SessionDetailsDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionDetailsDialog {
    pub fn new() -> Self {
        Self {
            session_result: None,
            repo_info: None,
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let session_result = self
            .session_result
            .as_ref()
            .expect("SessionDetailsDialog requires session data");

        let outer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(17),
                Constraint::Min(1),
            ])
            .split(f.area());

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(80),
                Constraint::Min(1),
            ])
            .split(outer_chunks[1]);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(horizontal_chunks[1]);

        HeaderView::render(f, main_chunks[0]);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Min(1)])
            .split(main_chunks[2]);

        BestRecordsView::render(f, content_chunks[0], session_result);
        StageResultsView::render(f, content_chunks[1], session_result, &self.repo_info);
        ControlsView::render(f, main_chunks[4]);
    }
}

impl Screen for SessionDetailsDialog {
    fn init(&mut self) -> Result<()> {
        if let Ok(Some(session_result)) = SessionManager::get_global_session_result() {
            let game_data = GameData::instance();
            let data = game_data.lock().unwrap();
            let repo_info = data.git_repository.clone();
            drop(data);

            self.session_result = Some(session_result);
            self.repo_info = repo_info;
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => Ok(ScreenTransition::Pop),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut Stdout,
        _session_result: Option<&crate::models::SessionResult>,
        _total_result: Option<&crate::scoring::TotalResult>,
    ) -> Result<()> {
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
        self.ui(frame);
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
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
