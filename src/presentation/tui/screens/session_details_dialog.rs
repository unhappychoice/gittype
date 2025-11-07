use crate::domain::events::EventBusInterface;
use crate::domain::models::SessionResult;
use crate::domain::repositories::session_repository::{BestRecords, BestStatus, SessionRepository};
use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::services::session_manager_service::SessionManagerInterface;
use crate::domain::stores::{RepositoryStore, RepositoryStoreInterface};
use crate::domain::services::SessionManager;
use crate::presentation::tui::views::{
    BestRecordsView, ControlsView, HeaderView, StageResultsView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::{domain::models::GitRepository, GitTypeError, Result};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::RwLock;
use std::sync::Arc;

pub struct SessionDetailsDialogData {
    pub session_result: Option<SessionResult>,
    pub repo_info: Option<GitRepository>,
    pub best_status: Option<BestStatus>,
    pub best_records: Option<BestRecords>,
}

pub struct SessionDetailsDialogDataProvider;

impl ScreenDataProvider for SessionDetailsDialogDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

pub trait SessionDetailsDialogInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = SessionDetailsDialogInterface)]
pub struct SessionDetailsDialog {
    #[shaku(default)]
    session_result: RwLock<Option<SessionResult>>,
    #[shaku(default)]
    repo_info: RwLock<Option<GitRepository>>,
    #[shaku(default)]
    best_status: RwLock<Option<BestStatus>>,
    #[shaku(default)]
    best_records: RwLock<Option<BestRecords>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
    #[shaku(inject)]
    session_manager: Arc<dyn SessionManagerInterface>,
    #[shaku(inject)]
    repository_store: Arc<dyn RepositoryStoreInterface>,
}

impl SessionDetailsDialog {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface>,
        session_manager: Arc<dyn SessionManagerInterface>,
        repository_store: Arc<dyn RepositoryStoreInterface>,
    ) -> Self {
        Self {
            session_result: RwLock::new(None),
            repo_info: RwLock::new(None),
            best_status: RwLock::new(None),
            best_records: RwLock::new(None),
            event_bus,
            theme_service,
            session_manager,
            repository_store,
        }
    }

    fn ui(&self, f: &mut Frame, colors: &crate::presentation::ui::Colors) {
        let session_result_ref = self.session_result.read().unwrap();
        let session_result = session_result_ref
            .as_ref()
            .expect("SessionDetailsDialog requires session data");

        // Calculate required content height dynamically
        let stage_count = session_result.stage_results.len();
        let best_records_lines = if self.best_records.read().unwrap().is_some() {
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

        HeaderView::render(f, main_chunks[0], colors);

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(best_records_lines as u16),
                Constraint::Min(1),
            ])
            .split(main_chunks[2]);

        BestRecordsView::render(
            f,
            content_chunks[0],
            session_result,
            self.best_status.read().unwrap().as_ref(),
            self.best_records.read().unwrap().as_ref(),
            colors,
        );
        StageResultsView::render(
            f,
            content_chunks[1],
            session_result,
            &self.repo_info.read().unwrap(),
            colors,
        );
        ControlsView::render(f, main_chunks[4], colors);
    }
}

pub struct SessionDetailsDialogProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for SessionDetailsDialogProvider {
    type Interface = SessionDetailsDialog;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: Arc<dyn EventBusInterface> = module.resolve();
        let theme_service: Arc<dyn crate::domain::services::theme_service::ThemeServiceInterface> =
            module.resolve();
        let session_manager: Arc<dyn SessionManagerInterface> = module.resolve();
        let repository_store: Arc<dyn RepositoryStoreInterface> = module.resolve();
        Ok(Box::new(SessionDetailsDialog::new(
            event_bus,
            theme_service,
            session_manager,
            repository_store,
        )))
    }
}

impl Screen for SessionDetailsDialog {
    fn get_type(&self) -> ScreenType {
        ScreenType::DetailsDialog
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(SessionDetailsDialogDataProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        // Try to downcast to SessionDetailsDialogData first
        let (session_result, repo_info, best_status, best_records) =
            if let Ok(dialog_data) = data.downcast::<SessionDetailsDialogData>() {
                (
                    dialog_data.session_result,
                    dialog_data.repo_info,
                    dialog_data.best_status,
                    dialog_data.best_records,
                )
            } else {
                // If no data provided, get from injected dependencies
                let sm = self
                    .session_manager
                    .as_any()
                    .downcast_ref::<SessionManager>()
                    .ok_or_else(|| {
                        GitTypeError::TerminalError("Failed to get SessionManager".to_string())
                    })?;

                let session_result = sm.get_session_result();
                let repo_info = self.repository_store.get_repository();

                let best_status = if let Some(ref result) = session_result {
                    sm.get_best_status_for_score(result.session_score)
                        .ok()
                        .flatten()
                } else {
                    None
                };

                let best_records = SessionRepository::get_best_records_global().ok().flatten();

                (session_result, repo_info, best_status, best_records)
            };

        *self.session_result.write().unwrap() = session_result;
        *self.repo_info.write().unwrap() = repo_info;
        *self.best_status.write().unwrap() = best_status;
        *self.best_records.write().unwrap() = best_records;

        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.event_bus.as_event_bus().publish(NavigateTo::Pop);
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        self.ui(frame, &colors);
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

impl SessionDetailsDialogInterface for SessionDetailsDialog {}
