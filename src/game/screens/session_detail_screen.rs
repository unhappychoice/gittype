use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::views::{PerformanceMetricsView, SessionInfoView, StageDetailsView};
use crate::storage::{
    daos::{
        session_dao::{SessionResultData, SessionStageResult},
        StoredRepository, StoredSession,
    },
    repositories::SessionRepository,
};
use crate::presentation::ui::Colors;
use crate::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use std::io::Stdout;

#[derive(Debug, Clone)]
pub struct SessionDisplayData {
    pub session: StoredSession,
    pub repository: Option<StoredRepository>,
    pub session_result: Option<SessionResultData>,
}

pub enum SessionDetailAction {
    Return,
}

pub struct SessionDetailScreen {
    session_data: SessionDisplayData,
    stage_results: Vec<SessionStageResult>,
    stage_scroll_offset: usize,
}

impl SessionDetailScreen {
    pub fn new_for_screen_manager() -> Result<Self> {
        let screen = Self {
            session_data: SessionDisplayData {
                session: StoredSession {
                    id: 0,
                    repository_id: None,
                    started_at: chrono::Utc::now(),
                    completed_at: Some(chrono::Utc::now()),
                    branch: None,
                    commit_hash: None,
                    is_dirty: false,
                    game_mode: "default".to_string(),
                    difficulty_level: None,
                    max_stages: None,
                    time_limit_seconds: None,
                },
                repository: None,
                session_result: None,
            },
            stage_results: Vec::new(),
            stage_scroll_offset: 0,
        };

        Ok(screen)
    }

    pub fn set_session_data(&mut self, session_data: SessionDisplayData) -> Result<()> {
        self.session_data = session_data;

        let session_repo = SessionRepository::new()?;
        self.stage_results =
            session_repo.get_session_stage_results(self.session_data.session.id)?;
        self.stage_scroll_offset = 0;

        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(f.area());

        let title = Paragraph::new("Session Details")
            .style(
                Style::default()
                    .fg(Colors::info())
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);
        f.render_widget(title, main_chunks[0]);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(12), Constraint::Min(1)])
            .split(main_chunks[1]);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_chunks[0]);

        SessionInfoView::render(
            f,
            top_chunks[0],
            &self.session_data.session,
            self.session_data.repository.as_ref(),
        );
        PerformanceMetricsView::render(f, top_chunks[1], self.session_data.session_result.as_ref());
        StageDetailsView::render(
            f,
            content_chunks[1],
            &self.stage_results,
            self.stage_scroll_offset,
        );

        let controls_line = Line::from(vec![
            Span::styled("[↑↓/JK]", Style::default().fg(Colors::key_navigation())),
            Span::styled(" Scroll Stages  ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Back", Style::default().fg(Colors::text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, main_chunks[2]);
    }
}

impl Screen for SessionDetailScreen {
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
            KeyCode::Up => {
                if self.stage_scroll_offset > 0 {
                    self.stage_scroll_offset -= 1;
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Down => {
                if self.stage_scroll_offset + 1 < self.stage_results.len() {
                    self.stage_scroll_offset += 1;
                }
                Ok(ScreenTransition::None)
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

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
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
