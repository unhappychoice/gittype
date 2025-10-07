use crate::domain::events::EventBus;
use crate::domain::models::GitRepository;
use crate::domain::services::scoring::StageTracker;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::game::views::session_failure::{content_view, footer_view, header_view};
use crate::presentation::game::{
    GameData, RenderBackend, Screen, ScreenDataProvider, ScreenType, SessionManager, UpdateStrategy,
};
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::ResetColor,
    terminal::{self},
};
use std::io::Stdout;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

pub struct SessionFailureScreenData {
    pub total_stages: usize,
    pub completed_stages: usize,
    pub stage_trackers: Vec<(String, StageTracker)>,
    pub repo_info: Option<GitRepository>,
}

pub struct SessionFailureScreenDataProvider {
    session_manager: Arc<Mutex<SessionManager>>,
    game_data: Arc<Mutex<GameData>>,
}

impl ScreenDataProvider for SessionFailureScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let total_stages = if let Ok(session_manager) = self.session_manager.lock() {
            session_manager.get_stage_info()?.1
        } else {
            1
        };

        let (completed_stages, stage_trackers) =
            if let Ok(session_manager) = self.session_manager.lock() {
                let stage_results = session_manager.get_stage_results();
                let completed = stage_results
                    .iter()
                    .filter(|sr| !sr.was_skipped && !sr.was_failed)
                    .count();

                let trackers =
                    if let Some(current_tracker) = session_manager.get_current_stage_tracker() {
                        vec![("current_stage".to_string(), current_tracker.clone())]
                    } else {
                        vec![]
                    };

                (completed, trackers)
            } else {
                (0, vec![])
            };

        let repo_info = if let Ok(game_data) = self.game_data.lock() {
            game_data.git_repository.clone()
        } else {
            None
        };

        Ok(Box::new(SessionFailureScreenData {
            total_stages,
            completed_stages,
            stage_trackers,
            repo_info,
        }))
    }
}

pub struct SessionFailureScreen {
    total_stages: usize,
    completed_stages: usize,
    stage_trackers: Vec<(String, StageTracker)>,
    repo_info: Option<GitRepository>,
    event_bus: EventBus,
}

impl SessionFailureScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            total_stages: 1,
            completed_stages: 0,
            stage_trackers: vec![],
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

    fn get_render_backend(&self) -> RenderBackend {
        RenderBackend::Crossterm
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        let screen_data = data.downcast::<SessionFailureScreenData>()?;

        self.total_stages = screen_data.total_stages;
        self.completed_stages = screen_data.completed_stages;
        self.stage_trackers = screen_data.stage_trackers;
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

    fn render_crossterm_with_data(&mut self, _stdout: &mut Stdout) -> Result<()> {
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
