use crate::game::models::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::views::analytics::{LanguagesView, OverviewView, RepositoriesView, TrendsView};
use crate::storage::repositories::{GitRepositoryRepository, SessionRepository};
use crate::ui::Colors;
use crate::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListState, Paragraph, ScrollbarState},
    Frame,
};
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct AnalyticsData {
    pub total_sessions: usize,
    pub avg_cpm: f64,
    pub avg_accuracy: f64,
    pub total_time_hours: f64,
    pub cpm_trend: Vec<(String, f64)>,
    pub accuracy_trend: Vec<(String, f64)>,
    pub top_repositories: Vec<(String, f64)>,
    pub top_languages: Vec<(String, f64, usize)>,
    pub daily_sessions: HashMap<String, usize>,
    pub best_cpm: f64,
    pub total_mistakes: usize,
    pub avg_session_duration: f64,
    pub current_streak: usize,
    pub repository_stats: HashMap<String, RepoStats>,
    pub language_stats: HashMap<String, LangStats>,
}

#[derive(Debug, Clone)]
pub struct RepoStats {
    pub avg_cpm: f64,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub total_sessions: usize,
    pub total_keystrokes: usize,
    pub total_mistakes: usize,
    pub total_duration_ms: u64,
    pub avg_score: f64,
    pub best_cpm: f64,
    pub best_accuracy: f64,
    pub stages_completed: usize,
    pub stages_attempted: usize,
    pub stages_skipped: usize,
}

#[derive(Debug, Clone)]
pub struct LangStats {
    pub avg_cpm: f64,
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub total_sessions: usize,
    pub total_keystrokes: usize,
    pub total_mistakes: usize,
    pub total_duration_ms: u64,
    pub avg_score: f64,
    pub best_cpm: f64,
    pub best_accuracy: f64,
    pub stages_completed: usize,
    pub stages_attempted: usize,
    pub stages_skipped: usize,
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
}

impl AnalyticsScreen {
    pub fn new_for_screen_manager() -> Result<Self> {
        Self::new()
    }

    fn new() -> Result<Self> {
        let mut repository_list_state = ListState::default();
        repository_list_state.select(Some(0));
        let mut language_list_state = ListState::default();
        language_list_state.select(Some(0));

        Ok(Self {
            view_mode: ViewMode::Overview,
            data: None,
            repository_list_state,
            language_list_state,
            repository_scroll_state: ScrollbarState::default(),
            language_scroll_state: ScrollbarState::default(),
            action_result: None,
        })
    }

    fn load_data(&mut self) -> Result<()> {
        let session_repo = SessionRepository::new()?;
        let git_repo_repo = GitRepositoryRepository::new()?;
        let sessions = session_repo.get_sessions_filtered(None, Some(90), "date", true)?;

        if sessions.is_empty() {
            self.data = Some(AnalyticsData {
                total_sessions: 0,
                avg_cpm: 0.0,
                avg_accuracy: 0.0,
                total_time_hours: 0.0,
                cpm_trend: Vec::new(),
                accuracy_trend: Vec::new(),
                top_repositories: Vec::new(),
                top_languages: Vec::new(),
                daily_sessions: HashMap::new(),
                best_cpm: 0.0,
                total_mistakes: 0,
                avg_session_duration: 0.0,
                current_streak: 0,
                repository_stats: HashMap::new(),
                language_stats: HashMap::new(),
            });
            return Ok(());
        }

        let mut total_cpm = 0.0;
        let mut total_accuracy = 0.0;
        let mut total_duration_ms = 0u64;
        let mut repo_stats: HashMap<String, (f64, usize)> = HashMap::new();
        let mut daily_counts: HashMap<String, usize> = HashMap::new();
        let mut cpm_by_day: HashMap<String, Vec<f64>> = HashMap::new();
        let mut accuracy_by_day: HashMap<String, Vec<f64>> = HashMap::new();
        let mut best_cpm = 0.0;
        let mut total_mistakes = 0;

        let mut session_results = Vec::new();
        let mut repositories_map = HashMap::new();
        {
            for session in &sessions {
                if let Ok(Some(result)) = session_repo.get_session_result_for_analytics(session.id)
                {
                    session_results.push((session.clone(), result));
                }
            }
            for session in &sessions {
                if let Some(repo_id) = session.repository_id {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        repositories_map.entry(repo_id)
                    {
                        if let Ok(Some(repo)) = git_repo_repo.get_repository_by_id(repo_id) {
                            e.insert(repo);
                        }
                    }
                }
            }
        }

        for (session, result) in session_results {
            total_cpm += result.cpm;
            total_accuracy += result.accuracy;
            total_duration_ms += result.duration_ms;

            if result.cpm > best_cpm {
                best_cpm = result.cpm;
            }

            let estimated_mistakes =
                ((100.0 - result.accuracy) / 100.0 * result.stages_attempted as f64) as usize;
            total_mistakes += estimated_mistakes;

            let date_key = session.started_at.format("%m-%d").to_string();
            *daily_counts.entry(date_key.clone()).or_insert(0) += 1;
            cpm_by_day
                .entry(date_key.clone())
                .or_default()
                .push(result.cpm);
            accuracy_by_day
                .entry(date_key)
                .or_default()
                .push(result.accuracy);

            if let Some(repo_id) = session.repository_id {
                if let Some(repo) = repositories_map.get(&repo_id) {
                    let repo_name = format!("{}/{}", repo.user_name, repo.repository_name);
                    let entry = repo_stats.entry(repo_name).or_insert((0.0, 0));
                    entry.0 += result.cpm;
                    entry.1 += 1;
                }
            }
        }

        let session_count = sessions.len();
        let avg_cpm = if session_count > 0 {
            total_cpm / session_count as f64
        } else {
            0.0
        };
        let avg_accuracy = if session_count > 0 {
            total_accuracy / session_count as f64
        } else {
            0.0
        };
        let total_time_hours = total_duration_ms as f64 / (1000.0 * 60.0 * 60.0);

        let mut cpm_trend: Vec<(String, f64)> = cpm_by_day
            .into_iter()
            .map(|(date, cpm_values)| {
                let avg = cpm_values.iter().sum::<f64>() / cpm_values.len() as f64;
                (date, avg)
            })
            .collect();
        cpm_trend.sort_by(|a, b| a.0.cmp(&b.0));

        let mut accuracy_trend: Vec<(String, f64)> = accuracy_by_day
            .into_iter()
            .map(|(date, accuracy_values)| {
                let avg = accuracy_values.iter().sum::<f64>() / accuracy_values.len() as f64;
                (date, avg)
            })
            .collect();
        accuracy_trend.sort_by(|a, b| a.0.cmp(&b.0));

        let mut top_repositories: Vec<(String, f64)> = repo_stats
            .into_iter()
            .map(|(name, (total_cpm, count))| (name, total_cpm / count as f64))
            .collect();
        top_repositories.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let top_languages = session_repo
            .get_language_stats(Some(90))
            .unwrap_or_else(|_| Vec::new());

        let avg_session_duration = if session_count > 0 {
            total_duration_ms as f64 / session_count as f64 / (1000.0 * 60.0)
        } else {
            0.0
        };

        let mut repository_stats = HashMap::new();
        let mut language_stats = HashMap::new();

        let all_repositories = git_repo_repo.get_all_repositories()?;
        let repo_map: HashMap<i64, String> = all_repositories
            .iter()
            .map(|repo| {
                (
                    repo.id,
                    format!("{}/{}", repo.user_name, repo.repository_name),
                )
            })
            .collect();

        for session in &sessions {
            let session_result = session_repo
                .get_session_result_for_analytics(session.id)
                .unwrap_or(None);

            if let Some(result) = session_result {
                if let Some(repo_id) = session.repository_id {
                    if let Some(repo_name) = repo_map.get(&repo_id) {
                        let repo_stats =
                            repository_stats
                                .entry(repo_name.clone())
                                .or_insert_with(|| RepoStats {
                                    avg_cpm: 0.0,
                                    avg_wpm: 0.0,
                                    avg_accuracy: 0.0,
                                    total_sessions: 0,
                                    total_keystrokes: 0,
                                    total_mistakes: 0,
                                    total_duration_ms: 0,
                                    avg_score: 0.0,
                                    best_cpm: 0.0,
                                    best_accuracy: 0.0,
                                    stages_completed: 0,
                                    stages_attempted: 0,
                                    stages_skipped: 0,
                                });

                        repo_stats.total_sessions += 1;
                        repo_stats.total_keystrokes += result.keystrokes;
                        repo_stats.total_mistakes += result.mistakes;
                        repo_stats.total_duration_ms += result.duration_ms;
                        repo_stats.stages_completed += result.stages_completed;
                        repo_stats.stages_attempted += result.stages_attempted;
                        repo_stats.stages_skipped += result.stages_skipped;
                        repo_stats.best_cpm = repo_stats.best_cpm.max(result.cpm);
                        repo_stats.best_accuracy = repo_stats.best_accuracy.max(result.accuracy);
                    }
                }

                let stage_results = session_repo
                    .get_session_stage_results(session.id)
                    .unwrap_or_default();
                for stage in stage_results {
                    if let Some(language) = stage.language {
                        let lang_stats =
                            language_stats
                                .entry(language.clone())
                                .or_insert_with(|| LangStats {
                                    avg_cpm: 0.0,
                                    avg_wpm: 0.0,
                                    avg_accuracy: 0.0,
                                    total_sessions: 0,
                                    total_keystrokes: 0,
                                    total_mistakes: 0,
                                    total_duration_ms: 0,
                                    avg_score: 0.0,
                                    best_cpm: 0.0,
                                    best_accuracy: 0.0,
                                    stages_completed: 0,
                                    stages_attempted: 0,
                                    stages_skipped: 0,
                                });

                        lang_stats.total_sessions += 1;
                        lang_stats.total_keystrokes += stage.keystrokes;
                        lang_stats.total_mistakes += stage.mistakes;
                        lang_stats.total_duration_ms += stage.duration_ms;
                        lang_stats.stages_completed += 1;
                        lang_stats.best_cpm = lang_stats.best_cpm.max(stage.cpm);
                        lang_stats.best_accuracy = lang_stats.best_accuracy.max(stage.accuracy);
                    }
                }
            }
        }

        for stats in repository_stats.values_mut() {
            if stats.total_sessions > 0 {
                stats.avg_cpm =
                    stats.total_keystrokes as f64 / (stats.total_duration_ms as f64 / 60000.0);
                stats.avg_wpm = stats.avg_cpm / 5.0;
                stats.avg_accuracy = ((stats.total_keystrokes - stats.total_mistakes) as f64
                    / stats.total_keystrokes as f64)
                    * 100.0;
                stats.avg_score = stats.avg_cpm * stats.avg_accuracy / 100.0;
            }
        }

        for stats in language_stats.values_mut() {
            if stats.total_sessions > 0 {
                stats.avg_cpm =
                    stats.total_keystrokes as f64 / (stats.total_duration_ms as f64 / 60000.0);
                stats.avg_wpm = stats.avg_cpm / 5.0;
                stats.avg_accuracy = ((stats.total_keystrokes - stats.total_mistakes) as f64
                    / stats.total_keystrokes as f64)
                    * 100.0;
                stats.avg_score = stats.avg_cpm * stats.avg_accuracy / 100.0;
            }
        }

        self.data = Some(AnalyticsData {
            total_sessions: session_count,
            avg_cpm,
            avg_accuracy,
            total_time_hours,
            cpm_trend,
            accuracy_trend,
            top_repositories,
            top_languages,
            daily_sessions: daily_counts,
            best_cpm,
            total_mistakes,
            avg_session_duration,
            current_streak: 0,
            repository_stats,
            language_stats,
        });

        Ok(())
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
                    .title("Navigation"),
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
    fn init(&mut self) -> Result<()> {
        self.action_result = None;

        if let Err(e) = self.load_data() {
            eprintln!("Warning: Failed to load analytics data: {}", e);
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key_event.code {
            KeyCode::Esc => {
                self.action_result = Some(AnalyticsAction::Return);
                Ok(ScreenTransition::Replace(
                    crate::game::models::ScreenType::Title,
                ))
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(AnalyticsAction::Return);
                Ok(ScreenTransition::Exit)
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.view_mode = self.view_mode.previous();
                Ok(ScreenTransition::None)
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.view_mode = self.view_mode.next();
                Ok(ScreenTransition::None)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                match self.view_mode {
                    ViewMode::Repositories => self.previous_repository(),
                    ViewMode::Languages => self.previous_language(),
                    _ => {}
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                match self.view_mode {
                    ViewMode::Repositories => self.next_repository(),
                    ViewMode::Languages => self.next_language(),
                    _ => {}
                }
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('r') => {
                if let Err(e) = self.load_data() {
                    eprintln!("Error loading data: {}", e);
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
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        let outer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(1),
                Constraint::Length(4),
            ])
            .split(frame.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(outer_chunks[1]);

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
