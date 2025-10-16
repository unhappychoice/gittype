use crate::domain::events::EventBus;
use crate::domain::models::storage::StoredRepositoryWithLanguages;
use crate::domain::services::repository_service::RepositoryService;
use crate::infrastructure::database::database::Database;
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

pub struct RepoPlayScreenData {
    pub repositories: Vec<(StoredRepositoryWithLanguages, bool)>,
}

pub struct RepoPlayScreen {
    repositories: Vec<(StoredRepositoryWithLanguages, bool)>,
    list_state: ListState,
    selected_index: Option<usize>,
    event_bus: EventBus,
}

impl RepoPlayScreen {
    pub fn new(event_bus: EventBus) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            repositories: Vec::new(),
            list_state,
            selected_index: None,
            event_bus,
        }
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn get_selected_repository(&self) -> Option<&(StoredRepositoryWithLanguages, bool)> {
        self.selected_index
            .and_then(|index| self.repositories.get(index))
    }
}

pub struct RepoPlayScreenDataProvider;

impl ScreenDataProvider for RepoPlayScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let db = Database::new()?;
        let service = RepositoryService::new(db);

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

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        if let Ok(screen_data) = data.downcast::<RepoPlayScreenData>() {
            self.repositories = screen_data.repositories;
            self.list_state = ListState::default();
            self.list_state.select(Some(0));
            self.selected_index = None;
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
            KeyCode::Char('j') | KeyCode::Down => {
                if let Some(selected) = self.list_state.selected() {
                    if !self.repositories.is_empty() && selected < self.repositories.len() - 1 {
                        self.list_state.select(Some(selected + 1));
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Some(selected) = self.list_state.selected() {
                    if selected > 0 {
                        self.list_state.select(Some(selected - 1));
                    }
                }
            }
            KeyCode::Char(' ') => {
                if let Some(selected) = self.list_state.selected() {
                    self.selected_index = Some(selected);
                    self.event_bus.publish(NavigateTo::Exit);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut Frame) -> Result<()> {
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
        RepositoryListView::render(frame, chunks[1], &self.repositories, &mut self.list_state);
        ControlsView::render(frame, chunks[2]);

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
