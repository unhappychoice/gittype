use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::views::{BestRecordsView, ControlsView, HeaderView, StageResultsView};
use crate::game::{GameData, SessionManager};
use crate::storage::repositories::session_repository::SessionRepository;
use crate::{domain::models::GitRepository, Result};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::io::Stdout;

pub struct SessionDetailsDialog {
    session_result: Option<crate::domain::models::SessionResult>,
    repo_info: Option<GitRepository>,
    best_status: Option<crate::storage::repositories::session_repository::BestStatus>,
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
            best_status: None,
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let session_result = self
            .session_result
            .as_ref()
            .expect("SessionDetailsDialog requires session data");

        // Calculate required content height dynamically
        let stage_count = session_result.stage_results.len();
        let best_records_lines = if let Ok(Some(_)) = SessionRepository::get_best_records_global() {
            5 // Header + 3 records + padding
        } else {
            3 // Just header and no records message
        };

        let stage_results_lines = if stage_count > 0 {
            2 + (stage_count * 2) // Header + (stage_name + metrics) * count
        } else {
            2 // Just header
        };

        let total_content_height = 1 + 1 + best_records_lines + stage_results_lines + 1 + 1; // header + spacing + content + spacing + controls
        let dialog_height =
            total_content_height.min(f.area().height.saturating_sub(4) as usize) as u16;

        let outer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(dialog_height),
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
            .constraints([
                Constraint::Length(best_records_lines as u16),
                Constraint::Min(1),
            ])
            .split(main_chunks[2]);

        BestRecordsView::render_with_best_status(
            f,
            content_chunks[0],
            session_result,
            self.best_status.as_ref(),
        );
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

            // Calculate best_status once during initialization
            self.best_status =
                SessionManager::get_best_status_for_score(session_result.session_score)
                    .ok()
                    .flatten();

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
        _session_result: Option<&crate::domain::models::SessionResult>,
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
