use crate::domain::events::EventBus;
use crate::domain::repositories::trending_repository::{
    TrendingRepositoryInfo, TRENDING_REPOSITORY,
};
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::views::trending_repository_selection::{
    ControlsView, HeaderView, RepositoryListView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};

pub struct TrendingRepositorySelectionScreen {
    repositories: Vec<TrendingRepositoryInfo>,
    list_state: ListState,
    selected_index: Option<usize>,
    event_bus: EventBus,
}

impl TrendingRepositorySelectionScreen {
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

    pub fn get_repositories(&self) -> &[TrendingRepositoryInfo] {
        &self.repositories
    }

    fn render_ui(&mut self, frame: &mut Frame) {
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
                Constraint::Length(4), // Header
                Constraint::Min(1),    // Repository list
                Constraint::Length(1), // Controls at bottom
            ])
            .split(outer_chunks[1]);

        HeaderView::render(frame, chunks[0]);
        RepositoryListView::render(frame, chunks[1], &self.repositories, &mut self.list_state);
        ControlsView::render(frame, chunks[2]);
    }
}

pub struct TrendingRepositorySelectionScreenDataProvider;

impl ScreenDataProvider for TrendingRepositorySelectionScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

impl Screen for TrendingRepositorySelectionScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::TrendingRepositorySelection
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TrendingRepositorySelectionScreenDataProvider)
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        if let Ok(params) = data.downcast::<(Option<String>, String)>() {
            let (language, period) = *params;

            // Build cache key
            let cache_key = format!("{}:{}", language.as_deref().unwrap_or("all"), period);

            // Fetch repositories
            let repositories = TRENDING_REPOSITORY.get_trending_repositories_sync(
                &cache_key,
                language.as_deref(),
                &period,
            )?;

            self.repositories = repositories;
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
            KeyCode::Char('c') if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
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
        self.render_ui(frame);
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
