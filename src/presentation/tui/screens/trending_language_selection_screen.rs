use crate::domain::events::EventBus;
use crate::presentation::game::events::NavigateTo;
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

pub struct TrendingLanguageSelectionScreen {
    list_state: ListState,
    selected_language: Option<String>,
    event_bus: EventBus,
}

impl TrendingLanguageSelectionScreen {
    pub fn new(event_bus: EventBus) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state,
            selected_language: None,
            event_bus,
        }
    }

    pub fn get_selected_language(&self) -> Option<&str> {
        self.selected_language.as_deref()
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
                Constraint::Length(3), // Header
                Constraint::Min(1),    // Language list
                Constraint::Length(1), // Controls at bottom
            ])
            .split(outer_chunks[1]);

        HeaderView::render(frame, chunks[0]);
        LanguageListView::render(frame, chunks[1], &mut self.list_state);
        ControlsView::render(frame, chunks[2]);
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

    fn init_with_data(&mut self, _data: Box<dyn std::any::Any>) -> Result<()> {
        self.list_state = ListState::default();
        self.list_state.select(Some(0));
        self.selected_language = None;
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
                    if selected < LanguageListView::languages_count() - 1 {
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
                    if let Some(lang_code) = LanguageListView::get_language_code(selected) {
                        self.selected_language = Some(lang_code.to_string());
                        self.event_bus.publish(NavigateTo::Exit);
                    }
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
