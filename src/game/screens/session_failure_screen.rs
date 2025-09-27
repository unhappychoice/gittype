use crate::game::game_data::GameData;
use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::session_manager::SessionManager;
use crate::game::views::session_failure::{content_view, footer_view, header_view};
use crate::game::ScreenType;
use crate::domain::models::GitRepository;
use crate::domain::services::scoring::StageTracker;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::ResetColor,
    terminal::{self},
};
use std::io::Stdout;
use std::io::{stdout, Write};

pub struct SessionFailureScreen {
    total_stages: usize,
    completed_stages: usize,
    stage_trackers: Vec<(String, StageTracker)>,
    repo_info: Option<GitRepository>,
}

impl Default for SessionFailureScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionFailureScreen {
    pub fn new() -> Self {
        Self {
            total_stages: 1,
            completed_stages: 0,
            stage_trackers: vec![],
            repo_info: None,
        }
    }

    fn load_global_data(&mut self) -> Result<()> {
        // Get total stages from the global API
        if let Ok((_, total)) = SessionManager::get_global_stage_info() {
            self.total_stages = total;
        }

        if let Ok(session_manager) = SessionManager::instance().lock() {
            let stage_results = session_manager.get_stage_results();

            // Count only successfully completed stages (not skipped or failed)
            self.completed_stages = stage_results
                .iter()
                .filter(|sr| !sr.was_skipped && !sr.was_failed)
                .count();

            // Get current stage tracker for the failed stage
            if let Some(current_tracker) = session_manager.get_current_stage_tracker() {
                self.stage_trackers = vec![("current_stage".to_string(), current_tracker.clone())];
            } else {
                self.stage_trackers = vec![];
            }
        }

        if let Ok(game_data) = GameData::instance().lock() {
            self.repo_info = game_data.git_repository.clone();
        }

        Ok(())
    }
}

impl Screen for SessionFailureScreen {
    fn init(&mut self) -> Result<()> {
        self.load_global_data()?;
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                Ok(ScreenTransition::Replace(ScreenType::Typing))
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                Ok(ScreenTransition::Replace(ScreenType::Title))
            }
            KeyCode::Esc => Ok(ScreenTransition::Exit),
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
        _total_result: Option<&crate::domain::services::scoring::TotalResult>,
    ) -> Result<()> {
        let mut stdout = stdout();

        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_y = terminal_height / 2;

        header_view::render_header(&mut stdout, terminal_width, center_y)?;

        content_view::render_stage_progress(
            &mut stdout,
            terminal_width,
            center_y,
            self.total_stages,
            self.completed_stages,
        )?;

        content_view::render_metrics(&mut stdout, terminal_width, center_y, &self.stage_trackers)?;
        content_view::render_failure_message(&mut stdout, terminal_width, center_y)?;
        footer_view::render_navigation(&mut stdout, terminal_width, center_y)?;

        execute!(stdout, ResetColor)?;

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
