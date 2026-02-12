use crate::domain::events::presentation_events::NavigateTo;
use crate::domain::events::EventBusInterface;
use crate::domain::models::storage::StoredRepository;
use crate::domain::services::session_service::{SessionDisplayData, SessionServiceInterface};
use crate::domain::services::theme_service::ThemeServiceInterface;
use crate::presentation::tui::{Screen, ScreenDataProvider, ScreenType, UpdateStrategy};
use crate::presentation::ui::Colors;
use crate::Result;
use chrono::{DateTime, Local};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
    Frame,
};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Date,
    Performance,
    Repository,
    Duration,
}

impl SortBy {
    pub fn display_name(&self) -> &str {
        match self {
            SortBy::Date => "Date",
            SortBy::Performance => "Score",
            SortBy::Repository => "Repository",
            SortBy::Duration => "Duration",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            SortBy::Date => "date",
            SortBy::Performance => "score",
            SortBy::Repository => "repository",
            SortBy::Duration => "duration",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateFilter {
    All,
    Last7Days,
    Last30Days,
    Last90Days,
}

impl DateFilter {
    pub fn display_name(&self) -> &str {
        match self {
            DateFilter::All => "All Time",
            DateFilter::Last7Days => "Last 7 days",
            DateFilter::Last30Days => "Last 30 days",
            DateFilter::Last90Days => "Last 90 days",
        }
    }

    pub fn to_days(&self) -> Option<i64> {
        match self {
            DateFilter::All => None,
            DateFilter::Last7Days => Some(7),
            DateFilter::Last30Days => Some(30),
            DateFilter::Last90Days => Some(90),
        }
    }
}

#[derive(Debug)]
pub struct FilterState {
    pub repository_filter: Option<i64>,
    pub date_filter: DateFilter,
    pub sort_by: SortBy,
    pub sort_descending: bool,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            repository_filter: None,
            date_filter: DateFilter::Last30Days,
            sort_by: SortBy::Date,
            sort_descending: true,
        }
    }
}

pub struct RecordsScreenData {
    pub sessions: Vec<SessionDisplayData>,
    pub repositories: Vec<StoredRepository>,
}

#[derive(Clone)]
pub enum RecordsAction {
    Return,
    ViewDetails(i64),
}

pub trait RecordsScreenInterface: Screen {}

#[derive(shaku::Component)]
#[shaku(interface = RecordsScreenInterface)]
pub struct RecordsScreen {
    #[shaku(default)]
    sessions: RwLock<Vec<SessionDisplayData>>,
    #[shaku(default)]
    repositories: RwLock<Vec<StoredRepository>>,
    #[shaku(default)]
    filter_state: RwLock<FilterState>,
    #[shaku(default)]
    list_state: RwLock<ListState>,
    #[shaku(default)]
    scroll_state: RwLock<ScrollbarState>,
    #[shaku(default)]
    action_result: RwLock<Option<RecordsAction>>,
    #[shaku(default)]
    selected_session_for_detail: RwLock<Option<SessionDisplayData>>,
    #[shaku(inject)]
    event_bus: Arc<dyn EventBusInterface>,
    #[shaku(inject)]
    theme_service: Arc<dyn ThemeServiceInterface>,
    #[shaku(inject)]
    session_service: Arc<dyn SessionServiceInterface>,
}

impl RecordsScreen {
    pub fn new(
        event_bus: Arc<dyn EventBusInterface>,
        theme_service: Arc<dyn ThemeServiceInterface>,
        session_service: Arc<dyn SessionServiceInterface>,
    ) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            sessions: RwLock::new(Vec::new()),
            repositories: RwLock::new(Vec::new()),
            filter_state: RwLock::new(FilterState::default()),
            list_state: RwLock::new(list_state),
            scroll_state: RwLock::new(ScrollbarState::default()),
            action_result: RwLock::new(None),
            selected_session_for_detail: RwLock::new(None),
            event_bus,
            theme_service,
            session_service,
        }
    }

    pub fn get_selected_session_for_detail(&self) -> Option<SessionDisplayData> {
        self.selected_session_for_detail.read().unwrap().clone()
    }

    pub fn set_selected_session_from_index(&self, index: usize) {
        let sessions = self.sessions.read().unwrap();
        if let Some(session) = sessions.get(index) {
            *self.selected_session_for_detail.write().unwrap() = Some(session.clone());
        }
    }

    fn render_session_list(&self, f: &mut Frame, colors: &Colors) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Header (title + filter info)
                Constraint::Min(1),    // Session list
                Constraint::Length(1), // Controls at bottom
            ])
            .split(f.area());

        // Header block containing title and filter info
        let header_lines = vec![
            Line::from(vec![
                Span::raw("  "), // Left padding
                Span::styled(
                    "Records - Typing Session Records",
                    Style::default()
                        .fg(colors.info())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("  "), // Left padding
                Span::styled(
                    {
                        let filter_state = self.filter_state.read().unwrap();
                        let sessions = self.sessions.read().unwrap();
                        format!(
                            "Filter: {} | Sort: {} {} | Sessions: {}",
                            filter_state.date_filter.display_name(),
                            filter_state.sort_by.display_name(),
                            if filter_state.sort_descending {
                                "↓"
                            } else {
                                "↑"
                            },
                            sessions.len()
                        )
                    },
                    Style::default().fg(colors.accuracy()),
                ),
            ]),
        ];

        let header = Paragraph::new(header_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border()))
                .title("Session Records"),
        );
        f.render_widget(header, chunks[0]);

        // Session list
        let sessions = self.sessions.read().unwrap();
        if sessions.is_empty() {
            let empty_msg = vec![
                Line::from("No typing sessions found for the selected time period."),
                Line::from("Start typing to build your records!"),
            ];
            let empty_paragraph = Paragraph::new(empty_msg)
                .style(Style::default().fg(colors.text_secondary()))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(colors.border()))
                        .title("Sessions"),
                );
            f.render_widget(empty_paragraph, chunks[1]);
        } else {
            // Update scroll state first - read and write separately to avoid deadlock
            let sessions_len = sessions.len();
            let new_scroll_state = self
                .scroll_state
                .read()
                .unwrap()
                .content_length(sessions_len);
            *self.scroll_state.write().unwrap() = new_scroll_state;

            // Create list items
            let items: Vec<ListItem> = sessions
                .iter()
                .map(|session_data| {
                    let line = format_session_line_ratatui_static(session_data, colors);
                    ListItem::new(line)
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(colors.border()))
                        .title("Sessions")
                        .title_style(
                            Style::default()
                                .fg(colors.text())
                                .add_modifier(Modifier::BOLD),
                        ),
                )
                .style(Style::default().fg(colors.text()))
                .highlight_style(
                    Style::default()
                        .bg(colors.background_secondary())
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");

            f.render_stateful_widget(list, chunks[1], &mut *self.list_state.write().unwrap());

            // Render scrollbar
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));
            f.render_stateful_widget(
                scrollbar,
                chunks[1].inner(Margin {
                    vertical: 1,
                    horizontal: 2,
                }),
                &mut *self.scroll_state.write().unwrap(),
            );
        }
        drop(sessions);

        // Controls at the bottom row - matching title screen colors
        let controls_line = Line::from(vec![
            Span::styled(
                "[↑↓/JK] Navigate  ",
                Style::default().fg(colors.key_navigation()),
            ),
            Span::styled("[SPACE]", Style::default().fg(colors.key_action())),
            Span::styled(" Details  ", Style::default().fg(colors.text())),
            Span::styled("[F]", Style::default().fg(colors.border())),
            Span::styled(" Filter  ", Style::default().fg(colors.text())),
            Span::styled("[S]", Style::default().fg(colors.info())),
            Span::styled(" Sort  ", Style::default().fg(colors.text())),
            Span::styled("[R]", Style::default().fg(colors.warning())),
            Span::styled(" Refresh  ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Back", Style::default().fg(colors.text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, chunks[2]);
    }

    fn refresh_sessions(&self) -> Result<()> {
        // Refresh repository list to include any newly created repositories
        *self.repositories.write().unwrap() = self.session_service.get_all_repositories()?;

        // Use the improved database filtering method
        let filter_state = self.filter_state.read().unwrap();
        let session_display_data = self.session_service.get_sessions_with_display_data(
            filter_state.repository_filter,
            filter_state.date_filter.to_days(),
            filter_state.sort_by.to_string(),
            filter_state.sort_descending,
        )?;
        drop(filter_state);

        *self.sessions.write().unwrap() = session_display_data;

        // Reset selection if needed
        let sessions_len = {
            let sessions = self.sessions.read().unwrap();
            let len = sessions.len();

            if sessions.is_empty() {
                self.list_state.write().unwrap().select(None);
            } else {
                self.list_state.write().unwrap().select(Some(0));
            }

            len
        };

        // Update scroll state - read first, then write
        let new_scroll_state = self
            .scroll_state
            .read()
            .unwrap()
            .content_length(sessions_len);
        *self.scroll_state.write().unwrap() = new_scroll_state;

        Ok(())
    }

    fn cycle_sort(&self) {
        use SortBy::*;
        let mut filter_state = self.filter_state.write().unwrap();
        filter_state.sort_by = match filter_state.sort_by {
            Date => Performance,
            Performance => Repository,
            Repository => Duration,
            Duration => Date,
        };
        // Toggle sort direction when cycling back to Date
        if filter_state.sort_by == Date {
            filter_state.sort_descending = !filter_state.sort_descending;
        }
    }

    fn cycle_date_filter(&self) {
        use DateFilter::*;
        let mut filter_state = self.filter_state.write().unwrap();
        filter_state.date_filter = match filter_state.date_filter {
            All => Last7Days,
            Last7Days => Last30Days,
            Last30Days => Last90Days,
            Last90Days => All,
        };
        drop(filter_state);

        // Reset selection
        let sessions = self.sessions.read().unwrap();
        if !sessions.is_empty() {
            self.list_state.write().unwrap().select(Some(0));
        }
    }
}

fn format_session_line_ratatui_static<'a>(
    session_data: &'a SessionDisplayData,
    colors: &Colors,
) -> Line<'a> {
    let local_time: DateTime<Local> = session_data.session.started_at.into();
    let date_str = local_time.format("%Y-%m-%d %H:%M").to_string();

    let repo_str = if let Some(ref repo) = session_data.repository {
        format!("{}/{}", repo.user_name, repo.repository_name)
    } else {
        "Unknown".to_string()
    };

    let (cpm_str, acc_str, score_str, stages_str, duration_str) =
        if let Some(ref result) = session_data.session_result {
            (
                format!("{:.1}", result.cpm),
                format!("{:.1}%", result.accuracy),
                format!("{:.0}", result.score),
                format!("{}/{}", result.stages_completed, result.stages_attempted),
                format!(
                    "{}m{}s",
                    result.duration_ms / 60000,
                    (result.duration_ms % 60000) / 1000
                ),
            )
        } else {
            (
                "--".to_string(),
                "--".to_string(),
                "--".to_string(),
                "--".to_string(),
                "--".to_string(),
            )
        };

    // Truncate repository name if too long
    let repo_display = if repo_str.len() > 24 {
        format!("{}...", &repo_str[..21])
    } else {
        format!("{:<24}", repo_str)
    };

    Line::from(vec![
        Span::styled(
            format!("{:<17}", date_str),
            Style::default().fg(colors.text()),
        ),
        Span::styled(
            format!("{:<26}", repo_display),
            Style::default().fg(colors.info()),
        ),
        Span::styled(
            format!("{:>6}", score_str),
            Style::default().fg(colors.score()),
        ),
        Span::styled(
            format!("{:>6}", cpm_str),
            Style::default().fg(colors.success()),
        ),
        Span::styled(
            format!("{:>6}", acc_str),
            Style::default().fg(colors.accuracy()),
        ),
        Span::styled(
            format!("{:>5}", stages_str),
            Style::default().fg(colors.border()),
        ),
        Span::styled(
            format!("{:>10}", duration_str),
            Style::default().fg(colors.text_secondary()),
        ),
    ])
}

pub struct RecordsScreenProvider;

impl shaku::Provider<crate::presentation::di::AppModule> for RecordsScreenProvider {
    type Interface = RecordsScreen;

    fn provide(
        module: &crate::presentation::di::AppModule,
    ) -> std::result::Result<Box<Self::Interface>, Box<dyn std::error::Error>> {
        use shaku::HasComponent;
        let event_bus: Arc<dyn EventBusInterface> = module.resolve();
        let theme_service: Arc<dyn ThemeServiceInterface> = module.resolve();
        let session_service: Arc<dyn SessionServiceInterface> = module.resolve();
        Ok(Box::new(RecordsScreen::new(
            event_bus,
            theme_service,
            session_service,
        )))
    }
}

impl Screen for RecordsScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Records
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        // Empty provider - RecordsScreen loads data in init_with_data
        struct EmptyProvider;
        impl ScreenDataProvider for EmptyProvider {
            fn provide(&self) -> Result<Box<dyn std::any::Any>> {
                Ok(Box::new(()))
            }
        }
        Box::new(EmptyProvider)
    }

    fn init_with_data(&self, data: Box<dyn std::any::Any>) -> Result<()> {
        *self.action_result.write().unwrap() = None;

        // Try to downcast to RecordsScreenData, or load from service
        if let Ok(screen_data) = data.downcast::<RecordsScreenData>() {
            *self.sessions.write().unwrap() = screen_data.sessions;
            *self.repositories.write().unwrap() = screen_data.repositories;
        } else {
            // Load data using injected service
            let session_display_data = self.session_service.get_sessions_with_display_data(
                None,     // repository_filter
                Some(30), // date_filter: Last 30 days
                "date",   // sort_by
                true,     // sort_descending
            )?;
            let repositories = self.session_service.get_all_repositories()?;

            *self.sessions.write().unwrap() = session_display_data;
            *self.repositories.write().unwrap() = repositories;
        }

        Ok(())
    }

    fn handle_key_event(&self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Esc => {
                *self.action_result.write().unwrap() = Some(RecordsAction::Return);
                self.event_bus
                    .as_event_bus()
                    .publish(NavigateTo::Replace(ScreenType::Title));
                Ok(())
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                *self.action_result.write().unwrap() = Some(RecordsAction::Return);
                self.event_bus.as_event_bus().publish(NavigateTo::Exit);
                Ok(())
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous_item();
                Ok(())
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next_item();
                Ok(())
            }
            KeyCode::Char('r') => {
                if let Err(e) = self.refresh_sessions() {
                    eprintln!("Error refreshing sessions: {}", e);
                }
                Ok(())
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let list_state = self.list_state.read().unwrap();
                if let Some(selected_index) = list_state.selected() {
                    let sessions = self.sessions.read().unwrap();
                    if let Some(session) = sessions.get(selected_index) {
                        *self.selected_session_for_detail.write().unwrap() = Some(session.clone());
                        self.event_bus
                            .as_event_bus()
                            .publish(NavigateTo::Push(ScreenType::SessionDetail));
                        return Ok(());
                    }
                }
                Ok(())
            }
            KeyCode::Char('s') => {
                self.cycle_sort();
                if let Err(e) = self.refresh_sessions() {
                    eprintln!("Error refreshing sessions after sort change: {}", e);
                }
                Ok(())
            }
            KeyCode::Char('f') => {
                self.cycle_date_filter();
                if let Err(e) = self.refresh_sessions() {
                    eprintln!("Error refreshing sessions after filter change: {}", e);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn render_ratatui(&self, frame: &mut ratatui::Frame) -> Result<()> {
        let colors = self.theme_service.get_colors();
        // Full implementation matching original render_session_list design
        self.render_session_list(frame, &colors);
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
}

impl RecordsScreenInterface for RecordsScreen {}

impl RecordsScreen {
    pub fn get_action_result(&self) -> Option<RecordsAction> {
        self.action_result.read().unwrap().clone()
    }

    fn next_item(&self) {
        let sessions = self.sessions.read().unwrap();
        let mut list_state = self.list_state.write().unwrap();
        let i = match list_state.selected() {
            Some(i) => {
                if i >= sessions.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        list_state.select(Some(i));
        let new_scroll_state = self.scroll_state.read().unwrap().position(i);
        *self.scroll_state.write().unwrap() = new_scroll_state;
    }

    fn previous_item(&self) {
        let sessions = self.sessions.read().unwrap();
        let mut list_state = self.list_state.write().unwrap();
        let i = match list_state.selected() {
            Some(i) => {
                if i == 0 {
                    sessions.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        list_state.select(Some(i));
        let new_scroll_state = self.scroll_state.read().unwrap().position(i);
        *self.scroll_state.write().unwrap() = new_scroll_state;
    }
}
