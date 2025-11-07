use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::repositories::trending_repository::{
    TrendingRepositoryInfo, TrendingRepositoryInterface,
};
use crate::domain::services::theme_service::ThemeServiceInterface;
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
use std::sync::{Arc, RwLock};

pub trait TrendingRepositorySelectionScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = TrendingRepositorySelectionScreenInterface)]
pub struct TrendingRepositorySelectionScreen {
    #[shaku(default)]
    repositories: RwLock<Vec<TrendingRepositoryInfo>>,
    #[shaku(default)]
    list_state: RwLock<ListState>,
    #[shaku(default)]
    selected_index: RwLock<Option<usize>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn ThemeServiceInterface>,
    #[shaku(inject)]
    trending_repository: Arc<dyn TrendingRepositoryInterface>,
}

impl TrendingRepositorySelectionScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn ThemeServiceInterface>,
        trending_repository: Arc<dyn TrendingRepositoryInterface>,
    ) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            repositories: RwLock::new(Vec::new()),
            list_state: RwLock::new(list_state),
            selected_index: RwLock::new(None),
            event_bus,
            theme_service,
            trending_repository,
        }
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        *self.selected_index.read().unwrap()
    }

    pub fn get_repositories(&self) -> Vec<TrendingRepositoryInfo> {
        self.repositories.read().unwrap().clone()
    }

    fn render_ui(&self, frame: &mut Frame, colors: &crate::presentation::ui::Colors) {
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

        HeaderView::render(frame, chunks[0], colors);
        let repositories = self.repositories.read().unwrap();
        let mut list_state = self.list_state.write().unwrap();
        RepositoryListView::render(frame, chunks[1], &repositories, &mut list_state, colors);
        ControlsView::render(frame, chunks[2], colors);
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

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        if let Ok(params) = data.downcast::<(Option<String>, String)>() {
            let (language, period) = *params;

            // Build cache key
            let cache_key = format!("{}:{}", language.as_deref().unwrap_or("all"), period);

            // Fetch repositories
            let repositories = self.trending_repository.get_trending_repositories_sync(
                &cache_key,
                language.as_deref(),
                &period,
            )?;

            *self.repositories.write().unwrap() = repositories;
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
        let colors = self.theme_service.get_colors();
        self.render_ui(frame, &colors);
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

impl TrendingRepositorySelectionScreenInterface for TrendingRepositorySelectionScreen {}
