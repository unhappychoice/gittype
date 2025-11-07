use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListState, Paragraph, ScrollbarState},
    Frame,
};

use std::sync::{Arc, RwLock};

use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::repositories::SessionRepository;
use crate::domain::services::analytics_service::{
    AnalyticsData, AnalyticsService, AnalyticsServiceInterface,
};
use crate::domain::services::theme_service::ThemeServiceInterface;
use crate::infrastructure::database::daos::{RepositoryDao, RepositoryDaoInterface};
use crate::infrastructure::database::database::{Database, DatabaseInterface};
use crate::presentation::tui::views::analytics::{
    LanguagesView, OverviewView, RepositoriesView, TrendsView,
};
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum ViewMode {
    #[default]
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

pub trait AnalyticsScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = AnalyticsScreenInterface)]
pub struct AnalyticsScreen {
    #[shaku(default)]
    view_mode: RwLock<ViewMode>,
    #[shaku(default)]
    data: RwLock<Option<AnalyticsData>>,
    #[shaku(default)]
    repository_list_state: RwLock<ListState>,
    #[shaku(default)]
    language_list_state: RwLock<ListState>,
    #[shaku(default)]
    repository_scroll_state: RwLock<ScrollbarState>,
    #[shaku(default)]
    language_scroll_state: RwLock<ScrollbarState>,
    #[shaku(default)]
    action_result: RwLock<Option<AnalyticsAction>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn ThemeServiceInterface>,
}

pub struct AnalyticsScreenDataProvider {}

impl ScreenDataProvider for AnalyticsScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let session_repository = Arc::new(SessionRepository::new()?);
        let db = Arc::new(Database::new()?) as Arc<dyn DatabaseInterface>;
        let repository_dao =
            Arc::new(RepositoryDao::new(Arc::clone(&db))) as Arc<dyn RepositoryDaoInterface>;
        let service = AnalyticsService::new(session_repository, repository_dao);

        service
            .load_analytics_data()
            .map(|data| Box::new(data) as Box<dyn std::any::Any>)
    }
}

impl AnalyticsScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn ThemeServiceInterface>,
    ) -> Self {
        let mut repository_list_state = ListState::default();
        repository_list_state.select(Some(0));
        let mut language_list_state = ListState::default();
        language_list_state.select(Some(0));

        Self {
            view_mode: RwLock::new(ViewMode::Overview),
            data: RwLock::new(None),
            repository_list_state: RwLock::new(repository_list_state),
            language_list_state: RwLock::new(language_list_state),
            repository_scroll_state: RwLock::new(ScrollbarState::default()),
            language_scroll_state: RwLock::new(ScrollbarState::default()),
            action_result: RwLock::new(None),
            event_bus,
            theme_service,
        }
    }

    fn next_repository(&self) {
        let data = self.data.read().unwrap();
        if let Some(data) = &*data {
            let mut list_state = self.repository_list_state.write().unwrap();
            let i = match list_state.selected() {
                Some(i) => {
                    if i >= data.top_repositories.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            list_state.select(Some(i));
            let new_scroll_state = self.repository_scroll_state.read().unwrap().position(i);
            *self.repository_scroll_state.write().unwrap() = new_scroll_state;
        }
    }

    fn previous_repository(&self) {
        let data = self.data.read().unwrap();
        if let Some(data) = &*data {
            let mut list_state = self.repository_list_state.write().unwrap();
            let i = match list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        data.top_repositories.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            list_state.select(Some(i));
            let new_scroll_state = self.repository_scroll_state.read().unwrap().position(i);
            *self.repository_scroll_state.write().unwrap() = new_scroll_state;
        }
    }

    fn next_language(&self) {
        let data = self.data.read().unwrap();
        if let Some(data) = &*data {
            let mut list_state = self.language_list_state.write().unwrap();
            let i = match list_state.selected() {
                Some(i) => {
                    if i >= data.top_languages.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            list_state.select(Some(i));
            let new_scroll_state = self.language_scroll_state.read().unwrap().position(i);
            *self.language_scroll_state.write().unwrap() = new_scroll_state;
        }
    }

    fn previous_language(&self) {
        let data = self.data.read().unwrap();
        if let Some(data) = &*data {
            let mut list_state = self.language_list_state.write().unwrap();
            let i = match list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        data.top_languages.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            list_state.select(Some(i));
            let new_scroll_state = self.language_scroll_state.read().unwrap().position(i);
            *self.language_scroll_state.write().unwrap() = new_scroll_state;
        }
    }

    pub fn get_action_result(&self) -> Option<AnalyticsAction> {
        self.action_result.read().unwrap().clone()
    }

    fn render_header(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let header = Paragraph::new(vec![Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Performance Analytics",
                Style::default()
                    .fg(colors.info())
                    .add_modifier(Modifier::BOLD),
            ),
        ])])
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border()))
                .title("GitType Analytics"),
        );
        f.render_widget(header, area);
    }

    fn render_view_tabs(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let all_views = [
            ViewMode::Overview,
            ViewMode::Trends,
            ViewMode::Repositories,
            ViewMode::Languages,
        ];

        let mut tab_spans = Vec::new();
        tab_spans.push(Span::raw("  "));

        let view_mode = *self.view_mode.read().unwrap();
        for (i, view) in all_views.iter().enumerate() {
            if i > 0 {
                tab_spans.push(Span::styled(" | ", Style::default().fg(colors.text())));
            }

            let style = if *view == view_mode {
                Style::default()
                    .fg(colors.text())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(colors.text_secondary())
            };

            tab_spans.push(Span::styled(view.display_name(), style));
        }

        let tabs = Paragraph::new(vec![Line::from(tab_spans)])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .title("Views"),
            );

        f.render_widget(tabs, area);
    }

    fn render_content_with_state(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let data = self.data.read().unwrap();
        if let Some(data) = &*data {
            let view_mode = *self.view_mode.read().unwrap();
            match view_mode {
                ViewMode::Overview => OverviewView::render(f, area, data, colors),
                ViewMode::Trends => TrendsView::render(f, area, data, colors),
                ViewMode::Repositories => {
                    let mut repo_list = self.repository_list_state.write().unwrap();
                    let mut repo_scroll = self.repository_scroll_state.write().unwrap();
                    RepositoriesView::render_with_state(
                        f,
                        area,
                        data,
                        &mut repo_list,
                        &mut repo_scroll,
                        colors,
                    )
                }
                ViewMode::Languages => {
                    let mut lang_list = self.language_list_state.write().unwrap();
                    let mut lang_scroll = self.language_scroll_state.write().unwrap();
                    LanguagesView::render_with_state(
                        f,
                        area,
                        data,
                        &mut lang_list,
                        &mut lang_scroll,
                        colors,
                    )
                }
            }
        } else {
            let loading = Paragraph::new("Loading analytics data...")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading, area);
        }
    }

    fn render_controls(&self, f: &mut Frame, area: Rect, colors: &Colors) {
        let controls_line = Line::from(vec![
            Span::styled("[←→/HL]", Style::default().fg(colors.key_navigation())),
            Span::styled(" Switch View  ", Style::default().fg(colors.text())),
            Span::styled("[↑↓/JK]", Style::default().fg(colors.key_navigation())),
            Span::styled(" Navigate  ", Style::default().fg(colors.text())),
            Span::styled("[R]", Style::default().fg(colors.score())),
            Span::styled(" Refresh  ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Back", Style::default().fg(colors.text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, area);
    }
}

pub struct AnalyticsScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for AnalyticsScreenProvider {
    type Interface = AnalyticsScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: std::sync::Arc<dyn EventBusInterface> = module.resolve();
        let theme_service: std::sync::Arc<dyn ThemeServiceInterface> = module.resolve();
        Ok(Box::new(AnalyticsScreen::new(event_bus, theme_service)))
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

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        *self.action_result.write().unwrap() = None;

        let analytics_data = data.downcast::<AnalyticsData>()?;

        *self.data.write().unwrap() = Some(*analytics_data);

        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                *self.action_result.write().unwrap() = Some(AnalyticsAction::Return);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::Title));
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                *self.action_result.write().unwrap() = Some(AnalyticsAction::Return);
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Left | KeyCode::Char('h') => {
                let mut view_mode = self.view_mode.write().unwrap();
                *view_mode = view_mode.previous();
                Ok(())
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let mut view_mode = self.view_mode.write().unwrap();
                *view_mode = view_mode.next();
                Ok(())
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let view_mode = *self.view_mode.read().unwrap();
                match view_mode {
                    ViewMode::Repositories => self.previous_repository(),
                    ViewMode::Languages => self.previous_language(),
                    _ => {}
                }
                Ok(())
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let view_mode = *self.view_mode.read().unwrap();
                match view_mode {
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
                        *self.data.write().unwrap() = Some(*analytics_data);
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut ratatui::Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(frame.area());

        self.render_header(frame, chunks[0], &colors);
        self.render_view_tabs(frame, chunks[1], &colors);
        self.render_content_with_state(frame, chunks[2], &colors);
        self.render_controls(frame, chunks[3], &colors);

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

impl AnalyticsScreenInterface for AnalyticsScreen {}
