use crate::application::service::analytics_service::{AnalyticsData, AnalyticsService};
use crate::domain::events::EventBus;
use crate::domain::repositories::SessionRepository;
use crate::infrastructure::database::database::Database;
use crate::presentation::game::events::NavigateTo;
use crate::presentation::tui::views::analytics::{
    LanguagesView, OverviewView, RepositoriesView, TrendsView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListState, Paragraph, ScrollbarState},
    Frame,
};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ViewMode {
    Overview,
    Trends,
    Repositories,
    Languages,
}

impl ViewMode {
    pub fn display_name(&self) -> &str {
        match self {
            ViewMode::Overview => "Overview",
            ViewMode::Trends => "Trends",
            ViewMode::Repositories => "Repositories",
            ViewMode::Languages => "Languages",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ViewMode::Overview => ViewMode::Trends,
            ViewMode::Trends => ViewMode::Repositories,
            ViewMode::Repositories => ViewMode::Languages,
            ViewMode::Languages => ViewMode::Overview,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            ViewMode::Overview => ViewMode::Languages,
            ViewMode::Trends => ViewMode::Overview,
            ViewMode::Repositories => ViewMode::Trends,
            ViewMode::Languages => ViewMode::Repositories,
        }
    }
}

#[derive(Clone)]
pub enum AnalyticsAction {
    Return,
}

pub struct AnalyticsScreen {
    view_mode: ViewMode,
    data: Option<AnalyticsData>,
    repository_list_state: ListState,
    language_list_state: ListState,
    repository_scroll_state: ScrollbarState,
    language_scroll_state: ScrollbarState,
    action_result: Option<AnalyticsAction>,
    event_bus: EventBus,
}

pub struct AnalyticsScreenDataProvider {}

impl ScreenDataProvider for AnalyticsScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_repository = Arc::new(SessionRepository::new()?);
        let db = Arc::new(Database::new()?);
        let service = AnalyticsService::new(session_repository, db);

        service
            .load_analytics_data()
            .map(|data| Box::new(data) as Box<dyn std::any::Any>)
    }
}


impl AnalyticsScreen {
    pub fn new(event_bus: EventBus) -> Self {
        let mut repository_list_state = ListState::default();
        repository_list_state.select(Some(0));
        let mut language_list_state = ListState::default();
        language_list_state.select(Some(0));

        Self {
            view_mode: ViewMode::Overview,
            data: None,
            repository_list_state,
            language_list_state,
            repository_scroll_state: ScrollbarState::default(),
            language_scroll_state: ScrollbarState::default(),
            action_result: None,
            event_bus,
        }
    }

    fn next_repository(&mut self) {
        if let Some(data) = &self.data {
            let i = match self.repository_list_state.selected() {
                Some(i) => {
                    if i >= data.top_repositories.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.repository_list_state.select(Some(i));
            self.repository_scroll_state = self.repository_scroll_state.position(i);
        }
    }

    fn previous_repository(&mut self) {
        if let Some(data) = &self.data {
            let i = match self.repository_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        data.top_repositories.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.repository_list_state.select(Some(i));
            self.repository_scroll_state = self.repository_scroll_state.position(i);
        }
    }

    fn next_language(&mut self) {
        if let Some(data) = &self.data {
            let i = match self.language_list_state.selected() {
                Some(i) => {
                    if i >= data.top_languages.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.language_list_state.select(Some(i));
            self.language_scroll_state = self.language_scroll_state.position(i);
        }
    }

    fn previous_language(&mut self) {
        if let Some(data) = &self.data {
            let i = match self.language_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        data.top_languages.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.language_list_state.select(Some(i));
            self.language_scroll_state = self.language_scroll_state.position(i);
        }
    }

    pub fn get_action_result(&self) -> Option<AnalyticsAction> {
        self.action_result.clone()
    }

    fn render_header(&mut self, f: &mut Frame, area: Rect) {
        let header = Paragraph::new(vec![Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Performance Analytics",
                Style::default()
                    .fg(Colors::info())
                    .add_modifier(Modifier::BOLD),
            ),
        ])])
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::border()))
                .title("GitType Analytics"),
        );
        f.render_widget(header, area);
    }

    fn render_view_tabs(&mut self, f: &mut Frame, area: Rect) {
        let all_views = [
            ViewMode::Overview,
            ViewMode::Trends,
            ViewMode::Repositories,
            ViewMode::Languages,
        ];

        let mut tab_spans = Vec::new();
        tab_spans.push(Span::raw("  "));

        for (i, view) in all_views.iter().enumerate() {
            if i > 0 {
                tab_spans.push(Span::styled(" | ", Style::default().fg(Colors::text())));
            }

            let style = if *view == self.view_mode {
                Style::default()
                    .fg(Colors::text())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Colors::text_secondary())
            };

            tab_spans.push(Span::styled(view.display_name(), style));
        }

        let tabs = Paragraph::new(vec![Line::from(tab_spans)])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Views"),
            );

        f.render_widget(tabs, area);
    }

    fn render_content_with_state(&mut self, f: &mut Frame, area: Rect) {
        if let Some(data) = &self.data.clone() {
            match self.view_mode {
                ViewMode::Overview => OverviewView::render(f, area, data),
                ViewMode::Trends => TrendsView::render(f, area, data),
                ViewMode::Repositories => RepositoriesView::render_with_state(
                    f,
                    area,
                    data,
                    &mut self.repository_list_state,
                    &mut self.repository_scroll_state,
                ),
                ViewMode::Languages => LanguagesView::render_with_state(
                    f,
                    area,
                    data,
                    &mut self.language_list_state,
                    &mut self.language_scroll_state,
                ),
            }
        } else {
            let loading = Paragraph::new("Loading analytics data...")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading, area);
        }
    }

    fn render_controls(&mut self, f: &mut Frame, area: Rect) {
        let controls_line = Line::from(vec![
            Span::styled("[←→/HL]", Style::default().fg(Colors::key_navigation())),
            Span::styled(" Switch View  ", Style::default().fg(Colors::text())),
            Span::styled("[↑↓/JK]", Style::default().fg(Colors::key_navigation())),
            Span::styled(" Navigate  ", Style::default().fg(Colors::text())),
            Span::styled("[R]", Style::default().fg(Colors::score())),
            Span::styled(" Refresh  ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Back", Style::default().fg(Colors::text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, area);
    }
}

impl Screen for AnalyticsScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Analytics
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(AnalyticsScreenDataProvider {})
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        self.action_result = None;

        let analytics_data = data.downcast::<AnalyticsData>()?;

        self.data = Some(*analytics_data);

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.action_result = Some(AnalyticsAction::Return);
                self.event_bus
                    .publish(NavigateTo::Replace(ScreenType::Title));
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(AnalyticsAction::Return);
                self.event_bus.publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.view_mode = self.view_mode.previous();
                Ok(())
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.view_mode = self.view_mode.next();
                Ok(())
            }
            KeyCode::Up | KeyCode::Char('k') => {
                match self.view_mode {
                    ViewMode::Repositories => self.previous_repository(),
                    ViewMode::Languages => self.previous_language(),
                    _ => {}
                }
                Ok(())
            }
            KeyCode::Down | KeyCode::Char('j') => {
                match self.view_mode {
                    ViewMode::Repositories => self.next_repository(),
                    ViewMode::Languages => self.next_language(),
                    _ => {}
                }
                Ok(())
            }
            KeyCode::Char('r') => {
                let provider = Self::default_provider();
                if let Ok(data) = provider.provide() {
                    if let Ok(analytics_data) = data.downcast::<AnalyticsData>() {
                        self.data = Some(*analytics_data);
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(frame.area());

        self.render_header(frame, chunks[0]);
        self.render_view_tabs(frame, chunks[1]);
        self.render_content_with_state(frame, chunks[2]);
        self.render_controls(frame, chunks[3]);

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
