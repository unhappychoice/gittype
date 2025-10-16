use crate::domain::events::EventBus;
use crate::domain::models::storage::{SessionStageResult, StoredSession};
use crate::domain::repositories::session_repository::{SessionRepository, SessionRepositoryTrait};
use crate::domain::services::session_service::SessionDisplayData;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::screens::RecordsScreen;
use crate::presentation::tui::views::{PerformanceMetricsView, SessionInfoView, StageDetailsView};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::{GitTypeError, Result};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub enum SessionDetailAction {
    Return,
}

pub struct SessionDetailScreen {
    session_data: SessionDisplayData,
    stage_results: Vec<SessionStageResult>,
    stage_scroll_offset: usize,
    event_bus: EventBus,
    session_repository: Option<Box<dyn SessionRepositoryTrait>>,
}

impl SessionDetailScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
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
            event_bus,
            session_repository: None,
        }
    }

    pub fn with_session_repository<T: SessionRepositoryTrait + 'static>(
        mut self,
        session_repository: T,
    ) -> Self {
        self.session_repository = Some(Box::new(session_repository));
        self
    }
}

pub struct SessionDetailScreenDataProvider;

impl ScreenDataProvider for SessionDetailScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

impl Screen for SessionDetailScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::SessionDetail
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(SessionDetailScreenDataProvider)
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        let _ = data;
        Ok(())
    }

    fn on_pushed_from(&mut self, source_screen: &dyn Screen) -> Result<()> {
        let records = source_screen
            .as_any()
            .downcast_ref::<RecordsScreen>()
            .ok_or_else(|| {
                GitTypeError::ScreenInitializationError(
                    "SessionDetail must be pushed from Records screen".to_string(),
                )
            })?;

        let session_data = records
            .get_selected_session_for_detail()
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                GitTypeError::ScreenInitializationError(
                    "SessionDetail requires selected session data from Records screen".to_string(),
                )
            })?;

        let stage_results = if let Some(ref repo) = self.session_repository {
            repo.get_session_stage_results(session_data.session.id)?
        } else {
            let repo = SessionRepository::new()?;
            repo.get_session_stage_results(session_data.session.id)?
        };

        self.session_data = session_data;
        self.stage_results = stage_results;
        self.stage_scroll_offset = 0;

        if self.session_data.session.id == 0 {
            return Err(GitTypeError::ScreenInitializationError(
                "SessionDetail: session id cannot be 0".to_string(),
            ));
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.event_bus.publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Up => {
                if self.stage_scroll_offset > 0 {
                    self.stage_scroll_offset -= 1;
                }
                Ok(())
            }
            KeyCode::Down => {
                if self.stage_scroll_offset + 1 < self.stage_results.len() {
                    self.stage_scroll_offset += 1;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let title = Paragraph::new("Session Details")
            .style(
                Style::default()
                    .fg(Colors::info())
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);
        frame.render_widget(title, main_chunks[0]);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(12), Constraint::Min(1)])
            .split(main_chunks[1]);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_chunks[0]);

        SessionInfoView::render(
            frame,
            top_chunks[0],
            &self.session_data.session,
            self.session_data.repository.as_ref(),
        );
        PerformanceMetricsView::render(
            frame,
            top_chunks[1],
            self.session_data.session_result.as_ref(),
        );
        StageDetailsView::render(
            frame,
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
        frame.render_widget(controls, main_chunks[2]);

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
