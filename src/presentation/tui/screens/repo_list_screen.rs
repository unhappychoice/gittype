use crate::application::service::repository_service::RepositoryService;
use crate::domain::events::EventBus;
use crate::domain::models::storage::StoredRepositoryWithLanguages;
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

pub struct RepoListScreenData {
    pub repositories: Vec<(StoredRepositoryWithLanguages, bool)>,
    pub cache_dir: String,
}

pub struct RepoListScreen {
    repositories: Vec<(StoredRepositoryWithLanguages, bool)>,
    cache_dir: String,
    event_bus: EventBus,
}

impl RepoListScreen {
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            repositories: Vec::new(),
            cache_dir: String::new(),
            event_bus,
        }
    }
}

pub struct RepoListScreenDataProvider;

impl ScreenDataProvider for RepoListScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let db = std::sync::Arc::new(Database::new()?);
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

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        if let Ok(screen_data) = data.downcast::<RepoListScreenData>() {
            self.repositories = screen_data.repositories;
            self.cache_dir = screen_data.cache_dir;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Esc => {
                self.event_bus.publish(NavigateTo::Exit);
            }
            KeyCode::Char('c')
                if key_event
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.event_bus.publish(NavigateTo::Exit);
            }
            _ => {}
        }

        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
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
        CacheInfoView::render(frame, chunks[2], &self.cache_dir);
        RepositoryListView::render(frame, chunks[4], &self.repositories);
        LegendView::render(frame, chunks[5]);
        ControlsView::render(frame, chunks[6]);

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_exitable(&self) -> bool {
        true
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
