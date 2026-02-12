use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::services::theme_service::ThemeServiceInterface;
use crate::presentation::tui::views::trending_language_selection::{
    ControlsView, HeaderView, LanguageListView,
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

pub trait TrendingLanguageSelectionScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = TrendingLanguageSelectionScreenInterface)]
pub struct TrendingLanguageSelectionScreen {
    #[shaku(default)]
    list_state: RwLock<ListState>,
    #[shaku(default)]
    selected_language: RwLock<Option<String>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn ThemeServiceInterface>,
}

impl TrendingLanguageSelectionScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn ThemeServiceInterface>,
    ) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state: RwLock::new(list_state),
            selected_language: RwLock::new(None),
            event_bus,
            theme_service,
        }
    }

    pub fn get_selected_language(&self) -> Option<String> {
        self.selected_language.read().unwrap().clone()
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
                Constraint::Length(3), // Header
                Constraint::Min(1),    // Language list
                Constraint::Length(1), // Controls at bottom
            ])
            .split(outer_chunks[1]);

        HeaderView::render(frame, chunks[0], colors);
        let mut list_state = self.list_state.write().unwrap();
        LanguageListView::render(frame, chunks[1], &mut list_state, colors);
        ControlsView::render(frame, chunks[2], colors);
    }
}

pub struct TrendingLanguageSelectionScreenDataProvider;

impl ScreenDataProvider for TrendingLanguageSelectionScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        Ok(Box::new(()))
    }
}

impl Screen for TrendingLanguageSelectionScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::TrendingLanguageSelection
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(TrendingLanguageSelectionScreenDataProvider)
    }

    fn init_with_data(&self, _data: Box<dyn std::any::Any>) -> Result<()> {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        *self.list_state.write().unwrap() = list_state;
        *self.selected_language.write().unwrap() = None;
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
                    if selected < LanguageListView::languages_count() - 1 {
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
                    if let Some(lang_code) = LanguageListView::get_language_code(selected) {
                        *self.selected_language.write().unwrap() = Some(lang_code.to_string());
                        self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                    }
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

impl TrendingLanguageSelectionScreenInterface for TrendingLanguageSelectionScreen {}
