use crate::domain::events::EventBusInterface;
use crate::domain::models::storage::StoredRepositoryWithLanguages;
use crate::domain::services::repository_service::RepositoryService;
use crate::infrastructure::database::database::Database;
use crate::infrastructure::git::RemoteGitRepositoryClient;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::views::repo_play::{ControlsView, HeaderView, RepositoryListView};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};
use std::sync::Arc;
use std::sync::RwLock;

pub struct RepoPlayScreenData {
    pub repositories: Vec<(StoredRepositoryWithLanguages, bool)>,
}

pub trait RepoPlayScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = RepoPlayScreenInterface)]
pub struct RepoPlayScreen {
    #[shaku(default)]
    repositories: RwLock<Vec<(StoredRepositoryWithLanguages, bool)>>,
    #[shaku(default)]
    list_state: RwLock<ListState>,
    #[shaku(default)]
    selected_index: RwLock<Option<usize>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
}

impl RepoPlayScreen {
    pub fn new(event_bus: Arc<dyn EventBusInterface>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            repositories: RwLock::new(Vec::new()),
            list_state: RwLock::new(list_state),
            selected_index: RwLock::new(None),
            event_bus,
        }
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        *self.selected_index.read().unwrap()
    }

    pub fn get_selected_repository(&self) -> Option<(StoredRepositoryWithLanguages, bool)> {
        let selected_index = *self.selected_index.read().unwrap();
        let repositories = self.repositories.read().unwrap();
        selected_index.and_then(|index| repositories.get(index).cloned())
    }
}

pub struct RepoPlayScreenDataProvider;

impl ScreenDataProvider for RepoPlayScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let db = Database::new()?;
        let service = RepositoryService::new(db, RemoteGitRepositoryClient::new());

        let repositories_with_cache = service.get_all_repositories_with_cache_status()?;

        Ok(Box::new(RepoPlayScreenData {
            repositories: repositories_with_cache,
        }))
    }
}

impl Screen for RepoPlayScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::RepoPlay
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(RepoPlayScreenDataProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        if let Ok(screen_data) = data.downcast::<RepoPlayScreenData>() {
            *self.repositories.write().unwrap() = screen_data.repositories;
            let mut list_state = ListState::default();
            list_state.select(Some(0));
            *self.list_state.write().unwrap() = list_state;
            *self.selected_index.write().unwrap() = None;
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
            KeyCode::Char('j') | KeyCode::Down => {
                let mut list_state = self.list_state.write().unwrap();
                if let Some(selected) = list_state.selected() {
                    let repositories = self.repositories.read().unwrap();
                    if !repositories.is_empty() && selected < repositories.len() - 1 {
                        list_state.select(Some(selected + 1));
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let mut list_state = self.list_state.write().unwrap();
                if let Some(selected) = list_state.selected() {
                    if selected > 0 {
                        list_state.select(Some(selected - 1));
                    }
                }
            }
            KeyCode::Char(' ') => {
                let list_state = self.list_state.read().unwrap();
                if let Some(selected) = list_state.selected() {
                    *self.selected_index.write().unwrap() = Some(selected);
                    self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn render_ratatui(&self, frame: &mut Frame) -> Result<()> {
        // Add horizontal padding
        let outer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(2), // Left padding
                Constraint::Min(1),    // Main content
                Constraint::Length(2), // Right padding
            ])
            .split(frame.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(1),    // Repository list
                Constraint::Length(1), // Controls at bottom
            ])
            .split(outer_chunks[1]);

        HeaderView::render(frame, chunks[0]);
        let repositories = self.repositories.read().unwrap();
        let mut list_state = self.list_state.write().unwrap();
        RepositoryListView::render(frame, chunks[1], &repositories, &mut list_state);
        ControlsView::render(frame, chunks[2]);

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

impl RepoPlayScreenInterface for RepoPlayScreen {}
