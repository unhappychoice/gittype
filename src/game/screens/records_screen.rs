use super::session_detail_screen::SessionDisplayData;
use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::storage::{
    daos::{session_dao::SessionResultData, StoredRepository},
    repositories::SessionRepository,
    HasDatabase,
};
use crate::ui::Colors;
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

#[derive(Clone)]
pub enum RecordsAction {
    Return,
    ViewDetails(i64),
}

pub struct RecordsScreen {
    sessions: Vec<SessionDisplayData>,
    repositories: Vec<StoredRepository>,
    filter_state: FilterState,
    list_state: ListState,
    scroll_state: ScrollbarState,
    action_result: Option<RecordsAction>,
    selected_session_for_detail: Option<SessionDisplayData>,
}

impl RecordsScreen {
    pub fn new_for_screen_manager() -> Result<Self> {
        Self::new()
    }

    fn new() -> Result<Self> {
        let session_repo = SessionRepository::new()?;
        let repositories = session_repo.get_all_repositories()?;
        let sessions = Vec::new();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut screen = Self {
            sessions,
            repositories,
            filter_state: FilterState::default(),
            list_state,
            scroll_state: ScrollbarState::default(),
            action_result: None,
            selected_session_for_detail: None,
        };

        screen.refresh_sessions()?;
        Ok(screen)
    }

    pub fn get_selected_session_for_detail(&self) -> &Option<SessionDisplayData> {
        &self.selected_session_for_detail
    }

    fn render_session_list(&mut self, f: &mut Frame) {
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
                        .fg(Colors::info())
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("  "), // Left padding
                Span::styled(
                    format!(
                        "Filter: {} | Sort: {} {} | Sessions: {}",
                        self.filter_state.date_filter.display_name(),
                        self.filter_state.sort_by.display_name(),
                        if self.filter_state.sort_descending {
                            "↓"
                        } else {
                            "↑"
                        },
                        self.sessions.len()
                    ),
                    Style::default().fg(Colors::accuracy()),
                ),
            ]),
        ];

        let header = Paragraph::new(header_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::border()))
                .title("Session Records"),
        );
        f.render_widget(header, chunks[0]);

        // Session list
        if self.sessions.is_empty() {
            let empty_msg = vec![
                Line::from("No typing sessions found for the selected time period."),
                Line::from("Start typing to build your records!"),
            ];
            let empty_paragraph = Paragraph::new(empty_msg)
                .style(Style::default().fg(Colors::text_secondary()))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::border()))
                        .title("Sessions"),
                );
            f.render_widget(empty_paragraph, chunks[1]);
        } else {
            // Update scroll state first
            self.scroll_state = self.scroll_state.content_length(self.sessions.len());

            // Create list items
            let items: Vec<ListItem> = self
                .sessions
                .iter()
                .map(|session_data| {
                    let line = format_session_line_ratatui_static(session_data);
                    ListItem::new(line)
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::border()))
                        .title("Sessions")
                        .title_style(
                            Style::default()
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        ),
                )
                .style(Style::default().fg(Colors::text()))
                .highlight_style(
                    Style::default()
                        .bg(Colors::background_secondary())
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");

            f.render_stateful_widget(list, chunks[1], &mut self.list_state);

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
                &mut self.scroll_state,
            );
        }

        // Controls at the bottom row - matching title screen colors
        let controls_line = Line::from(vec![
            Span::styled(
                "[↑↓/JK] Navigate  ",
                Style::default().fg(Colors::key_navigation()),
            ),
            Span::styled("[SPACE]", Style::default().fg(Colors::key_action())),
            Span::styled(" Details  ", Style::default().fg(Colors::text())),
            Span::styled("[F]", Style::default().fg(Colors::border())),
            Span::styled(" Filter  ", Style::default().fg(Colors::text())),
            Span::styled("[S]", Style::default().fg(Colors::info())),
            Span::styled(" Sort  ", Style::default().fg(Colors::text())),
            Span::styled("[R]", Style::default().fg(Colors::warning())),
            Span::styled(" Refresh  ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Back", Style::default().fg(Colors::text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, chunks[2]);
    }

    fn refresh_sessions(&mut self) -> Result<()> {
        let session_repo = SessionRepository::new()?;

        // Use the improved database filtering method
        let sessions = session_repo.get_sessions_filtered(
            self.filter_state.repository_filter,
            self.filter_state.date_filter.to_days(),
            self.filter_state.sort_by.to_string(),
            self.filter_state.sort_descending,
        )?;

        // Convert to SessionDisplayData with session results and repository info
        let mut session_display_data = Vec::new();
        let repository_map: std::collections::HashMap<i64, StoredRepository> = self
            .repositories
            .iter()
            .map(|repo| (repo.id, repo.clone()))
            .collect();

        for session in sessions {
            let session_result = {
                let db = session_repo.db_with_lock()?;
                db.get_session_result(session.id).unwrap_or(None)
            };

            let repository = session
                .repository_id
                .and_then(|id| repository_map.get(&id).cloned());

            session_display_data.push(SessionDisplayData {
                session,
                repository,
                session_result,
            });
        }

        self.sessions = session_display_data;

        // Reset selection if needed
        if self.sessions.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(0));
        }

        self.scroll_state = self.scroll_state.content_length(self.sessions.len());

        Ok(())
    }

    fn cycle_sort(&mut self) {
        use SortBy::*;
        self.filter_state.sort_by = match self.filter_state.sort_by {
            Date => Performance,
            Performance => Repository,
            Repository => Duration,
            Duration => Date,
        };
        // Toggle sort direction when cycling back to Date
        if self.filter_state.sort_by == Date {
            self.filter_state.sort_descending = !self.filter_state.sort_descending;
        }
    }

    fn cycle_date_filter(&mut self) {
        use DateFilter::*;
        self.filter_state.date_filter = match self.filter_state.date_filter {
            All => Last7Days,
            Last7Days => Last30Days,
            Last30Days => Last90Days,
            Last90Days => All,
        };

        // Reset selection
        if !self.sessions.is_empty() {
            self.list_state.select(Some(0));
        }
    }
}

fn format_session_line_ratatui_static<'a>(session_data: &'a SessionDisplayData) -> Line<'a> {
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
            Style::default().fg(Colors::text()),
        ),
        Span::styled(
            format!("{:<26}", repo_display),
            Style::default().fg(Colors::info()),
        ),
        Span::styled(
            format!("{:>6}", score_str),
            Style::default().fg(Colors::score()),
        ),
        Span::styled(
            format!("{:>6}", cpm_str),
            Style::default().fg(Colors::success()),
        ),
        Span::styled(
            format!("{:>6}", acc_str),
            Style::default().fg(Colors::accuracy()),
        ),
        Span::styled(
            format!("{:>5}", stages_str),
            Style::default().fg(Colors::border()),
        ),
        Span::styled(
            format!("{:>10}", duration_str),
            Style::default().fg(Colors::text_secondary()),
        ),
    ])
}

// Extension trait for getting session result data
trait SessionResultExt {
    fn get_session_result(&self, session_id: i64) -> Result<Option<SessionResultData>>;
}

impl SessionResultExt for crate::storage::Database {
    fn get_session_result(&self, session_id: i64) -> Result<Option<SessionResultData>> {
        use crate::storage::daos::SessionDao;
        let dao = SessionDao::new(self);
        dao.get_session_result(session_id)
    }
}

impl Screen for RecordsScreen {
    fn init(&mut self) -> Result<()> {
        self.action_result = None;
        self.refresh_sessions()?;
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Esc => {
                self.action_result = Some(RecordsAction::Return);
                // Return to Title screen
                Ok(ScreenTransition::Replace(
                    crate::game::models::ScreenType::Title,
                ))
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(RecordsAction::Return);
                Ok(ScreenTransition::Exit)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous_item();
                Ok(ScreenTransition::None)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next_item();
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('r') => {
                if let Err(e) = self.refresh_sessions() {
                    eprintln!("Error refreshing sessions: {}", e);
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if let Some(selected_index) = self.list_state.selected() {
                    if let Some(session) = self.sessions.get(selected_index) {
                        // Store session data for the transition
                        self.selected_session_for_detail = Some(session.clone());

                        return Ok(ScreenTransition::Push(
                            crate::game::models::ScreenType::SessionDetail,
                        ));
                    }
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('s') => {
                self.cycle_sort();
                if let Err(e) = self.refresh_sessions() {
                    eprintln!("Error refreshing sessions after sort change: {}", e);
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('f') => {
                self.cycle_date_filter();
                if let Err(e) = self.refresh_sessions() {
                    eprintln!("Error refreshing sessions after filter change: {}", e);
                }
                Ok(ScreenTransition::None)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut std::io::Stdout,
        _session_result: Option<&crate::models::SessionResult>,
        _total_result: Option<&crate::scoring::TotalResult>,
    ) -> Result<()> {
        // NOTE: History screen should use render_ratatui() instead
        // This render_crossterm_with_data() should not be used
        eprintln!("Warning: Records screen render_crossterm_with_data() called - this should use ratatui backend");
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        // Full implementation matching original render_session_list design
        self.render_session_list(frame);
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl RecordsScreen {
    pub fn get_action_result(&self) -> Option<RecordsAction> {
        self.action_result.clone()
    }

    fn next_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.sessions.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }

    fn previous_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.sessions.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }
}
