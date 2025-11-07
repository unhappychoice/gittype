use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use std::sync::{Arc, RwLock};

use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::models::storage::SessionStageResult;
use crate::domain::repositories::session_repository::SessionRepositoryTrait;
use crate::domain::services::session_service::SessionDisplayData;
use crate::domain::services::theme_service::ThemeServiceInterface;
use crate::presentation::tui::screens::RecordsScreen;
use crate::presentation::tui::views::{PerformanceMetricsView, SessionInfoView, StageDetailsView};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{GitTypeError, Result};

pub enum SessionDetailAction {
    Return,
}

pub trait SessionDetailScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = SessionDetailScreenInterface)]
pub struct SessionDetailScreen {
    #[shaku(default)]
    session_data: RwLock<SessionDisplayData>,
    #[shaku(default)]
    stage_results: RwLock<Vec<SessionStageResult>>,
    #[shaku(default)]
    stage_scroll_offset: RwLock<usize>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn ThemeServiceInterface>,
    #[shaku(inject)]
    session_repository: Arc<dyn SessionRepositoryTrait>,
}

impl SessionDetailScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn ThemeServiceInterface>,
        session_repository: Arc<dyn SessionRepositoryTrait>,
    ) -> Self {
        Self {
            session_data: RwLock::new(SessionDisplayData::default()),
            stage_results: RwLock::new(Vec::new()),
            stage_scroll_offset: RwLock::new(0),
            event_bus,
            theme_service,
            session_repository,
        }
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

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        let _ = data;
        Ok(())
    }

    fn on_pushed_from(&self, source_screen: &dyn Screen) -> Result<()> {
        log::debug!("SessionDetailScreen::on_pushed_from called");

        let records = source_screen
            .as_any()
            .downcast_ref::<RecordsScreen>()
            .ok_or_else(|| {
                log::error!("Failed to downcast source_screen to RecordsScreen");
                GitTypeError::ScreenInitializationError(
                    "SessionDetail must be pushed from Records screen".to_string(),
                )
            })?;

        let session_data = records.get_selected_session_for_detail().ok_or_else(|| {
            log::error!("No session data selected in RecordsScreen");
            GitTypeError::ScreenInitializationError(
                "SessionDetail requires selected session data from Records screen".to_string(),
            )
        })?;

        log::debug!(
            "Session data retrieved: id={}, repository={:?}",
            session_data.session.id,
            session_data.repository.as_ref().map(|r| &r.repository_name)
        );

        if session_data.session.id == 0 {
            log::error!("Session id is 0");
            return Err(GitTypeError::ScreenInitializationError(
                "SessionDetail: session id cannot be 0".to_string(),
            ));
        }

        let stage_results = self
            .session_repository
            .get_session_stage_results(session_data.session.id)?;

        log::debug!(
            "Retrieved {} stage results for session {}",
            stage_results.len(),
            session_data.session.id
        );

        *self.session_data.write().unwrap() = session_data.clone();
        *self.stage_results.write().unwrap() = stage_results;
        *self.stage_scroll_offset.write().unwrap() = 0;

        log::debug!("SessionDetailScreen initialized successfully");
        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Up => {
                let mut offset = self.stage_scroll_offset.write().unwrap();
                if *offset > 0 {
                    *offset -= 1;
                }
                Ok(())
            }
            KeyCode::Down => {
                let mut offset = self.stage_scroll_offset.write().unwrap();
                let stage_results = self.stage_results.read().unwrap();
                if *offset + 1 < stage_results.len() {
                    *offset += 1;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut ratatui::Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        let session_data = self.session_data.read().unwrap();
        let stage_results = self.stage_results.read().unwrap();
        let stage_scroll_offset = *self.stage_scroll_offset.read().unwrap();

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
                    .fg(colors.info())
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
            &session_data.session,
            session_data.repository.as_ref(),
            &colors,
        );
        PerformanceMetricsView::render(
            frame,
            top_chunks[1],
            session_data.session_result.as_ref(),
            &colors,
        );
        StageDetailsView::render(
            frame,
            content_chunks[1],
            &stage_results,
            stage_scroll_offset,
            &colors,
        );

        let controls_line = Line::from(vec![
            Span::styled("[↑↓/JK]", Style::default().fg(colors.key_navigation())),
            Span::styled(" Scroll Stages  ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Back", Style::default().fg(colors.text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        frame.render_widget(controls, main_chunks[2]);

        Ok(())
    }

    fn cleanup(&self) -> Result<()> {
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

impl SessionDetailScreenInterface for SessionDetailScreen {}
