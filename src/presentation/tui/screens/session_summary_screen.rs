use crate::domain::events::EventBusInterface;
use crate::domain::models::{Rank, SessionResult};
use crate::domain::events::presentation_events::NavigateTo;
use crate::presentation::game::{GameData, SessionManager};
use crate::presentation::tui::views::{
    OptionsView, RankView, ScoreView, SessionSummaryHeaderView, SummaryView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{domain::models::GitRepository, GitTypeError, Result};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::RwLock;
use std::sync::{Arc, Mutex};

pub struct SessionSummaryScreenData {
    pub session_result: Option<SessionResult>,
    pub git_repository: Option<GitRepository>,
    pub session_manager: Arc<Mutex<SessionManager>>,
}

pub struct SessionSummaryScreenDataProvider {
    session_manager: Arc<Mutex<SessionManager>>,
    game_data: Arc<Mutex<GameData>>,
}

impl ScreenDataProvider for SessionSummaryScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_result = self
            .session_manager
            .lock()
            .map_err(|_| GitTypeError::TerminalError("Failed to lock SessionManager".to_string()))?
            .get_session_result();

        let git_repository = self
            .game_data
            .lock()
            .map_err(|_| GitTypeError::TerminalError("Failed to lock GameData".to_string()))?
            .git_repository
            .clone();

        Ok(Box::new(SessionSummaryScreenData {
            session_result,
            git_repository,
            session_manager: Arc::clone(&self.session_manager),
        }))
    }
}

#[derive(Debug, Clone)]
pub enum ResultAction {
    Restart,
    BackToTitle,
    Quit,
    Retry,
    Share,
}

pub trait SessionSummaryScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = SessionSummaryScreenInterface)]
pub struct SessionSummaryScreen {
    #[shaku(default)]
    action_result: RwLock<Option<ResultAction>>,
    #[shaku(default)]
    session_result: RwLock<Option<SessionResult>>,
    #[shaku(default)]
    git_repository: RwLock<Option<GitRepository>>,
    #[shaku(default)]
    session_manager: RwLock<Option<Arc<Mutex<SessionManager>>>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
}

impl SessionSummaryScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    ) -> Self {
        Self {
            action_result: RwLock::new(None),
            session_result: RwLock::new(None),
            git_repository: RwLock::new(None),
            session_manager: RwLock::new(None),
            event_bus,
            theme_service,
        }
    }

    pub fn get_action_result(&self) -> Option<ResultAction> {
        self.action_result.read().unwrap().clone()
    }
}

pub struct SessionSummaryScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for SessionSummaryScreenProvider {
    type Interface = SessionSummaryScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: std::sync::Arc<dyn crate::domain::events::EventBusInterface> =
            module.resolve();
        let theme_service: std::sync::Arc<
            dyn crate::domain::services::theme_service::ThemeServiceInterface,
        > = module.resolve();
        Ok(Box::new(SessionSummaryScreen::new(
            event_bus,
            theme_service,
        )))
    }
}

impl Screen for SessionSummaryScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::SessionSummary
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(SessionSummaryScreenDataProvider {
            session_manager: SessionManager::instance(),
            game_data: GameData::instance(),
        })
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        *self.action_result.write().unwrap() = None;

        let screen_data = data.downcast::<SessionSummaryScreenData>()?;

        *self.session_result.write().unwrap() = screen_data.session_result.clone();
        *self.git_repository.write().unwrap() = screen_data.git_repository.clone();
        *self.session_manager.write().unwrap() = Some(Arc::clone(&screen_data.session_manager));

        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Push(ScreenType::DetailsDialog));
                Ok(())
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                *self.action_result.write().unwrap() = Some(ResultAction::Retry);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::Typing));
                Ok(())
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                *self.action_result.write().unwrap() = Some(ResultAction::Share);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Push(ScreenType::SessionSharing));
                Ok(())
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                *self.action_result.write().unwrap() = Some(ResultAction::BackToTitle);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::PopTo(ScreenType::Title));
                Ok(())
            }
            KeyCode::Esc => {
                *self.action_result.write().unwrap() = Some(ResultAction::Quit);
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                *self.action_result.write().unwrap() = Some(ResultAction::Quit);
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        let session_result = self.session_result.read().unwrap();
        if let Some(ref session_result) = *session_result {
            let area = frame.area();

            let best_rank = Rank::for_score(session_result.session_score);

            // Get best status using session start records from SessionManager instance
            let session_manager = self.session_manager.read().unwrap();
            let best_status = session_manager
                .as_ref()
                .and_then(|manager| manager.lock().ok())
                .and_then(|manager| {
                    manager
                        .get_best_status_for_score(session_result.session_score)
                        .ok()
                        .flatten()
                });

            // Get actual rank ASCII height
            let rank_patterns =
                crate::domain::models::ui::ascii_rank_titles::get_all_rank_patterns();
            let rank_lines = rank_patterns.get(best_rank.name());
            let rank_ascii_height = rank_lines.map(|l| l.len()).unwrap_or(0);

            // Check if last line is empty to determine spacing needed
            let last_line_is_empty = rank_lines
                .and_then(|lines| lines.last())
                .map(|line| line.trim().is_empty())
                .unwrap_or(false);

            let rank_total_height = if last_line_is_empty {
                rank_ascii_height + 1 // ASCII + tier info
            } else {
                rank_ascii_height + 2 // ASCII + spacing + tier info
            };

            // Calculate content height
            let header_height = 4; // Header (title + spacing + YOU'RE)
            let score_height = 8; // Score label + best label + ASCII + diff
            let summary_height = 2; // Two lines of metrics
            let options_height = 2; // Two lines of options
            let total_content_height = header_height
                + rank_total_height
                + 2 // spacing before score
                + score_height
                + 1 // spacing after score
                + summary_height
                + 2 // spacing
                + options_height;

            let top_spacing = (area.height.saturating_sub(total_content_height as u16)) / 2;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(top_spacing),
                    Constraint::Length(4),                        // Header
                    Constraint::Length(rank_total_height as u16), // Rank + tier info (with spacing if needed)
                    Constraint::Length(2),                        // Spacing before score
                    Constraint::Length(score_height as u16),      // Score
                    Constraint::Length(1),                        // Spacing after score
                    Constraint::Length(2),                        // Summary
                    Constraint::Length(2),                        // Spacing
                    Constraint::Length(2),                        // Options
                    Constraint::Min(0),
                ])
                .split(area);

            SessionSummaryHeaderView::render(frame, chunks[1], &colors);
            RankView::render(
                frame,
                chunks[2],
                &best_rank,
                session_result.session_score,
            );
            ScoreView::render(
                frame,
                chunks[4],
                session_result,
                &best_rank,
                best_status.as_ref(),
                &colors,
            );
            SummaryView::render(frame, chunks[6], session_result, &colors);
            OptionsView::render(frame, chunks[8], &colors);
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

impl SessionSummaryScreenInterface for SessionSummaryScreen {}
