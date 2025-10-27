use crate::domain::events::EventBusInterface;
use crate::domain::models::storage::StoredRepositoryWithLanguages;
use crate::domain::services::repository_service::RepositoryService;
use crate::infrastructure::database::database::Database;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::views::repo_list::{
    CacheInfoView, ControlsView, HeaderView, LegendView, RepositoryListView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use std::sync::Arc;
use std::sync::RwLock;

pub struct RepoListScreenData {
    pub repositories: Vec<(StoredRepositoryWithLanguages, bool)>,
    pub cache_dir: String,
}

pub trait RepoListScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = RepoListScreenInterface)]
pub struct RepoListScreen {
    repositories: RwLock<Vec<(StoredRepositoryWithLanguages, bool)>>,
    cache_dir: RwLock<String>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
}

impl RepoListScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        Self {
            repositories: RwLock::new(Vec::new()),
            cache_dir: RwLock::new(String::new()),
            event_bus,
        }
    }
}

pub struct RepoListScreenDataProvider;

impl ScreenDataProvider for RepoListScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let db = Database::new()?;
        let service = RepositoryService::new(db);

        let repositories_with_cache = service.get_all_repositories_with_cache_status()?;
        let cache_dir = RepositoryService::get_cache_directory();

        Ok(Box::new(RepoListScreenData {
            repositories: repositories_with_cache,
            cache_dir: cache_dir.to_string_lossy().to_string(),
        }))
    }
}

impl Screen for RepoListScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::RepoList
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(RepoListScreenDataProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        if let Ok(screen_data) = data.downcast::<RepoListScreenData>() {
            *self.repositories.write().unwrap() = screen_data.repositories;
            *self.cache_dir.write().unwrap() = screen_data.cache_dir;
        }
        Ok(())
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Esc => {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
            }
            KeyCode::Char('c')
                if key_event
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
            }
            _ => {}
        }

        Ok(())
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(1), // Spacer
                Constraint::Length(3), // Cache info
                Constraint::Length(1), // Spacer
                Constraint::Min(1),    // Repository list
                Constraint::Length(3), // Legend
                Constraint::Length(1), // Controls
            ])
            .split(frame.area());

        HeaderView::render(frame, chunks[0]);
        let cache_dir = self.cache_dir.read().unwrap();
        CacheInfoView::render(frame, chunks[2], &cache_dir);
        let repositories = self.repositories.read().unwrap();
        RepositoryListView::render(frame, chunks[4], &repositories);
        LegendView::render(frame, chunks[5]);
        ControlsView::render(frame, chunks[6]);

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&self) -> Result<bool> {
        Ok(false)
    }

    fn cleanup(&self) -> Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_exitable(&self) -> bool {
        true
    }
}

impl RepoListScreenInterface for RepoListScreen {}
