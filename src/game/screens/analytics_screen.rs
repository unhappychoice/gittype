use crate::storage::{repositories::SessionRepository, HasDatabase};
use crate::ui::Colors;
use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState,
        Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame, Terminal,
};
use std::collections::HashMap;
use std::io::stdout;

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
    // Extended repository and language statistics
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
}

impl AnalyticsScreen {
    pub fn show() -> Result<AnalyticsAction> {
        let mut screen = Self::new()?;
        screen.run()
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
        })
    }

    fn run(&mut self) -> Result<AnalyticsAction> {
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        if let Err(e) = self.load_data() {
            eprintln!("Warning: Failed to load analytics data: {}", e);
            // Use empty data as fallback
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
        }
        let result = self.run_app(&mut terminal);

        terminal.backend_mut().execute(LeaveAlternateScreen)?;
        result
    }

    fn run_app(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<AnalyticsAction> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(AnalyticsAction::Return),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(AnalyticsAction::Return);
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.view_mode = self.view_mode.previous();
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.view_mode = self.view_mode.next();
                    }
                    KeyCode::Up | KeyCode::Char('k') => match self.view_mode {
                        ViewMode::Repositories => self.previous_repository(),
                        ViewMode::Languages => self.previous_language(),
                        _ => {}
                    },
                    KeyCode::Down | KeyCode::Char('j') => match self.view_mode {
                        ViewMode::Repositories => self.next_repository(),
                        ViewMode::Languages => self.next_language(),
                        _ => {}
                    },
                    KeyCode::Char('r') => {
                        if let Err(e) = self.load_data() {
                            eprintln!("Error loading data: {}", e);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn load_data(&mut self) -> Result<()> {
        let session_repo = SessionRepository::new()?;
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

        // Avoid deadlock by collecting all data in a single database lock
        let mut session_results = Vec::new();
        let mut repositories_map = HashMap::new();
        {
            let db = session_repo.db_with_lock()?;
            // First, collect all session results
            for session in &sessions {
                if let Ok(Some(result)) = db.get_session_result(session.id) {
                    session_results.push((session.clone(), result));
                }
            }
            // Then, collect all repository information
            for session in &sessions {
                if let Some(repo_id) = session.repository_id {
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        repositories_map.entry(repo_id)
                    {
                        if let Ok(Some(repo)) = db.get_repository(repo_id) {
                            e.insert(repo);
                        }
                    }
                }
            }
        } // Database lock released here

        // Now process the collected results without holding the lock
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
        // Display all repositories - no limit

        let top_languages = session_repo
            .get_language_stats(Some(90))
            .unwrap_or_else(|_| Vec::new());

        let avg_session_duration = if session_count > 0 {
            total_duration_ms as f64 / session_count as f64 / (1000.0 * 60.0)
        } else {
            0.0
        };

        // Calculate detailed repository and language statistics
        let mut repository_stats = HashMap::new();
        let mut language_stats = HashMap::new();

        // Get all repositories for mapping repo_id to repo_name
        let all_repositories = session_repo.get_all_repositories()?;
        let repo_map: HashMap<i64, String> = all_repositories
            .iter()
            .map(|repo| {
                (
                    repo.id,
                    format!("{}/{}", repo.user_name, repo.repository_name),
                )
            })
            .collect();

        // Process each session for detailed stats
        for session in &sessions {
            let session_result = {
                let db = session_repo.db_with_lock()?;
                db.get_session_result(session.id).unwrap_or(None)
            };

            if let Some(result) = session_result {
                // Repository stats
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

                // Language stats (get from stage results)
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
                        lang_stats.stages_completed += 1; // Each stage record represents a completed stage
                        lang_stats.best_cpm = lang_stats.best_cpm.max(stage.cpm);
                        lang_stats.best_accuracy = lang_stats.best_accuracy.max(stage.accuracy);
                    }
                }
            }
        }

        // Calculate averages for repository stats
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

        // Calculate averages for language stats
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

    fn ui(&mut self, f: &mut Frame) {
        // Add horizontal padding
        let outer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4), // Left padding
                Constraint::Min(1),    // Main content
                Constraint::Length(4), // Right padding
            ])
            .split(f.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // View tabs
                Constraint::Min(1),    // Content
                Constraint::Length(1), // Controls
            ])
            .split(outer_chunks[1]);

        self.render_header(f, chunks[0]);
        self.render_view_tabs(f, chunks[1]);
        self.render_content(f, chunks[2]);
        self.render_controls(f, chunks[3]);
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let header = Paragraph::new(vec![Line::from(vec![
            Span::raw("  "), // Left padding
            Span::styled(
                "Performance Analytics",
                Style::default()
                    .fg(Colors::INFO)
                    .add_modifier(Modifier::BOLD),
            ),
        ])])
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::BORDER))
                .title("GitType Analytics"),
        );
        f.render_widget(header, area);
    }

    fn render_view_tabs(&self, f: &mut Frame, area: Rect) {
        let all_views = [
            ViewMode::Overview,
            ViewMode::Trends,
            ViewMode::Repositories,
            ViewMode::Languages,
        ];

        let mut tab_spans = Vec::new();
        tab_spans.push(Span::raw("  ")); // Left padding

        for (i, view) in all_views.iter().enumerate() {
            if i > 0 {
                tab_spans.push(Span::styled(" | ", Style::default().fg(Colors::TEXT)));
            }

            let style = if *view == self.view_mode {
                Style::default()
                    .fg(Colors::TEXT)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Colors::MUTED)
            };

            tab_spans.push(Span::styled(view.display_name(), style));
        }

        let tabs = Paragraph::new(vec![Line::from(tab_spans)])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Navigation"),
            );

        f.render_widget(tabs, area);
    }

    fn render_content(&mut self, f: &mut Frame, area: Rect) {
        if let Some(data) = &self.data.clone() {
            match self.view_mode {
                ViewMode::Overview => self.render_overview(f, area, data),
                ViewMode::Trends => self.render_trends(f, area, data),
                ViewMode::Repositories => self.render_repositories(f, area, data),
                ViewMode::Languages => self.render_languages(f, area, data),
            }
        } else {
            let loading = Paragraph::new("Loading analytics data...")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading, area);
        }
    }

    fn render_overview(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Stats summary
                Constraint::Min(3),    // Chart area
                Constraint::Length(8), // Top repositories and languages
            ])
            .split(area);

        // Overview stats (two lines)
        let overview_text = vec![
            Line::from(vec![
                Span::raw("  "), // Left padding
                Span::raw("Sessions: "),
                Span::styled(
                    data.total_sessions.to_string(),
                    Style::default().fg(Colors::CPM_WPM),
                ),
                Span::raw("  │  Avg CPM: "),
                Span::styled(
                    format!("{:.1}", data.avg_cpm),
                    Style::default().fg(Colors::ACCURACY),
                ),
                Span::raw("  │  Best CPM: "),
                Span::styled(
                    format!("{:.1}", data.best_cpm),
                    Style::default().fg(Colors::ACCURACY),
                ),
                Span::raw("  │  Avg Accuracy: "),
                Span::styled(
                    format!("{:.1}%", data.avg_accuracy),
                    Style::default().fg(Colors::INFO),
                ),
            ]),
            Line::from(vec![
                Span::raw("Total Time: "),
                Span::styled(
                    format!("{:.1}h", data.total_time_hours),
                    Style::default().fg(Colors::SCORE),
                ),
                Span::raw("  │  Avg Session: "),
                Span::styled(
                    format!("{:.1}m", data.avg_session_duration),
                    Style::default().fg(Colors::SCORE),
                ),
                Span::raw("  │  Total Mistakes: "),
                Span::styled(
                    data.total_mistakes.to_string(),
                    Style::default().fg(Colors::ERROR),
                ),
                Span::raw("  │  Repositories: "),
                Span::styled(
                    data.top_repositories.len().to_string(),
                    Style::default().fg(Colors::ACTION_KEY),
                ),
            ]),
        ];

        let overview = Paragraph::new(overview_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Overview (Last 7 days)"),
            );
        f.render_widget(overview, chunks[0]);

        // Simple text-based chart
        self.render_simple_chart(f, chunks[1], data);

        // Bottom section with top repositories and languages
        self.render_bottom_stats(f, chunks[2], data);
    }

    fn render_trends(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // CPM trend
                Constraint::Percentage(50), // Accuracy trend (placeholder)
            ])
            .split(area);

        self.render_cpm_trend(f, chunks[0], data);
        self.render_accuracy_trend(f, chunks[1], data);
    }

    fn render_cpm_trend(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        if data.cpm_trend.is_empty() {
            let empty_msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw(
                        "No trend data available - keep typing to build your performance history!",
                    ),
                ]),
            ])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("CPM Trend"),
            );
            f.render_widget(empty_msg, area);
            return;
        }

        // Convert trend data to chart points
        let mut chart_data: Vec<(f64, f64)> = Vec::new();
        for (i, (_date, cpm)) in data.cpm_trend.iter().enumerate() {
            chart_data.push((i as f64, *cpm));
        }

        // Calculate bounds
        let max_cpm = data
            .cpm_trend
            .iter()
            .map(|(_, cpm)| *cpm)
            .fold(0.0, f64::max);
        let min_cpm = data
            .cpm_trend
            .iter()
            .map(|(_, cpm)| *cpm)
            .fold(max_cpm, f64::min);
        let cpm_range = (max_cpm - min_cpm).max(10.0); // Minimum range of 10

        let datasets = vec![Dataset::default()
            .name("CPM")
            .marker(ratatui::symbols::Marker::Braille)
            .style(Style::default().fg(Colors::CPM_WPM))
            .graph_type(GraphType::Line)
            .data(&chart_data)];

        // Create x-axis labels from dates
        let x_labels: Vec<Span> = data
            .cpm_trend
            .iter()
            .step_by((data.cpm_trend.len().max(1) / 8).max(1)) // Show ~8 labels max
            .map(|(date, _)| {
                let day = if date.len() >= 5 { &date[3..] } else { date };
                Span::styled(day, Style::default().fg(Colors::TEXT))
            })
            .collect();

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("CPM Performance Trend"),
            )
            .x_axis(
                Axis::default()
                    .title("Date")
                    .style(Style::default().fg(Colors::SECONDARY))
                    .labels(x_labels)
                    .bounds([0.0, (data.cpm_trend.len().saturating_sub(1)) as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("CPM")
                    .style(Style::default().fg(Colors::SECONDARY))
                    .bounds([min_cpm - cpm_range * 0.1, max_cpm + cpm_range * 0.1])
                    .labels(vec![
                        Span::styled(format!("{:.0}", min_cpm), Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.0}", (min_cpm + max_cpm) / 2.0),
                            Style::default().fg(Colors::TEXT),
                        ),
                        Span::styled(format!("{:.0}", max_cpm), Style::default().fg(Colors::TEXT)),
                    ]),
            );

        f.render_widget(chart, area);
    }

    fn render_accuracy_trend(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        if data.accuracy_trend.is_empty() {
            let empty_msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw("No accuracy trend data available - keep typing to build your accuracy history!"),
                ]),
            ])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Accuracy Trend"),
            );
            f.render_widget(empty_msg, area);
            return;
        }

        // Convert trend data to chart points
        let mut chart_data: Vec<(f64, f64)> = Vec::new();
        for (i, (_date, accuracy)) in data.accuracy_trend.iter().enumerate() {
            chart_data.push((i as f64, *accuracy));
        }

        // Calculate bounds for accuracy (should be between 0-100)
        let max_accuracy = data
            .accuracy_trend
            .iter()
            .map(|(_, acc)| *acc)
            .fold(0.0, f64::max);
        let min_accuracy = data
            .accuracy_trend
            .iter()
            .map(|(_, acc)| *acc)
            .fold(max_accuracy, f64::min);
        let accuracy_range = (max_accuracy - min_accuracy).max(10.0);

        let datasets = vec![Dataset::default()
            .name("Accuracy")
            .marker(ratatui::symbols::Marker::Braille)
            .style(Style::default().fg(Colors::ACCURACY))
            .graph_type(GraphType::Line)
            .data(&chart_data)];

        // Create x-axis labels from dates
        let x_labels: Vec<Span> = data
            .accuracy_trend
            .iter()
            .step_by((data.accuracy_trend.len().max(1) / 8).max(1))
            .map(|(date, _)| {
                let day = if date.len() >= 5 { &date[3..] } else { date };
                Span::styled(day, Style::default().fg(Colors::TEXT))
            })
            .collect();

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Accuracy Performance Trend"),
            )
            .x_axis(
                Axis::default()
                    .title("Date")
                    .style(Style::default().fg(Colors::SECONDARY))
                    .labels(x_labels)
                    .bounds([0.0, (data.accuracy_trend.len().saturating_sub(1)) as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("Accuracy (%)")
                    .style(Style::default().fg(Colors::SECONDARY))
                    .bounds([
                        min_accuracy - accuracy_range * 0.1,
                        max_accuracy + accuracy_range * 0.1,
                    ])
                    .labels(vec![
                        Span::styled(
                            format!("{:.1}%", min_accuracy),
                            Style::default().fg(Colors::TEXT),
                        ),
                        Span::styled(
                            format!("{:.1}%", (min_accuracy + max_accuracy) / 2.0),
                            Style::default().fg(Colors::TEXT),
                        ),
                        Span::styled(
                            format!("{:.1}%", max_accuracy),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
            );

        f.render_widget(chart, area);
    }

    fn render_repositories(&mut self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), // Repository list
                Constraint::Percentage(60), // Repository details
            ])
            .split(area);

        // Left side: Repository list
        let mut items: Vec<ListItem> = Vec::new();

        if data.top_repositories.is_empty() {
            items.push(ListItem::new("No repositories available"));
        } else {
            // Calculate available width for repository list
            let available_width = chunks[0].width.saturating_sub(4) as usize; // Account for borders
            let cpm_text_width = 10; // "123.4 CPM" max width
            let name_width = available_width.saturating_sub(cpm_text_width);

            for (repo_name, avg_cpm) in data.top_repositories.iter() {
                // Truncate name to fit available space
                let display_name = if repo_name.len() > name_width {
                    format!("{}...", &repo_name[..name_width.saturating_sub(3)])
                } else {
                    repo_name.clone()
                };

                let cpm_text = format!("{:.1} CPM", avg_cpm);
                let spaces_needed =
                    available_width.saturating_sub(display_name.len() + cpm_text.len());

                let item_text =
                    format!("{}{}{}", display_name, " ".repeat(spaces_needed), cpm_text);
                items.push(ListItem::new(item_text));
            }
        }

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Repositories"),
            )
            .style(Style::default().fg(Colors::TEXT))
            .highlight_style(
                Style::default()
                    .bg(Colors::MUTED)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("► ");

        // Update scrollbar content length
        self.repository_scroll_state = self
            .repository_scroll_state
            .content_length(data.top_repositories.len());

        f.render_stateful_widget(list, chunks[0], &mut self.repository_list_state);

        // Render scrollbar for repository list
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        f.render_stateful_widget(
            scrollbar,
            chunks[0].inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.repository_scroll_state,
        );

        // Right side: Repository details
        self.render_repository_details(f, chunks[1], data);
    }

    fn render_repository_details(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let selected_index = self.repository_list_state.selected();

        let detail_lines = if let (Some(_), Some(repo_data)) = (
            selected_index,
            data.top_repositories.get(selected_index.unwrap_or(0)),
        ) {
            let repo_name = &repo_data.0;
            let detailed_stats = data.repository_stats.get(repo_name);

            let mut lines = vec![
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Repository: ", Style::default().fg(Colors::TEXT)),
                    Span::styled(
                        &repo_data.0,
                        Style::default()
                            .fg(Colors::INFO)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
            ];

            if let Some(stats) = detailed_stats {
                lines.extend_from_slice(&[
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "📈 Speed Metrics:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average CPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", stats.avg_cpm),
                            Style::default().fg(Colors::CPM_WPM),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average WPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", stats.avg_wpm),
                            Style::default().fg(Colors::ACTION_KEY),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best CPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", stats.best_cpm),
                            Style::default().fg(Colors::CPM_WPM),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "🎯 Accuracy & Quality:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average Accuracy: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}%", stats.avg_accuracy),
                            Style::default().fg(Colors::ACCURACY),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best Accuracy: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}%", stats.best_accuracy),
                            Style::default().fg(Colors::ACCURACY),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Mistakes: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{}", stats.total_mistakes),
                            Style::default().fg(Colors::ERROR),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "📊 Volume & Activity:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Sessions: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{}", stats.total_sessions),
                            Style::default().fg(Colors::INFO),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Keystrokes: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{}", stats.total_keystrokes),
                            Style::default().fg(Colors::SCORE),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Time: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!(
                                "{}h {}m",
                                stats.total_duration_ms / 3600000,
                                (stats.total_duration_ms % 3600000) / 60000
                            ),
                            Style::default().fg(Colors::SECONDARY),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "🏆 Progress:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average Score: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.0}", stats.avg_score),
                            Style::default().fg(Colors::SCORE),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Stages: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!(
                                "{}/{} completed",
                                stats.stages_completed, stats.stages_attempted
                            ),
                            Style::default().fg(Colors::BORDER),
                        ),
                    ]),
                ]);
            } else {
                lines.extend_from_slice(&[
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• Average CPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", repo_data.1),
                            Style::default().fg(Colors::CPM_WPM),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• WPM Equivalent: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", repo_data.1 / 5.0),
                            Style::default().fg(Colors::ACTION_KEY),
                        ),
                    ]),
                ]);
            }

            lines.extend_from_slice(&[
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        "Navigation:",
                        Style::default()
                            .fg(Colors::TEXT)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Use ", Style::default().fg(Colors::MUTED)),
                    Span::styled("↑↓ / JK", Style::default().fg(Colors::ACCURACY)),
                    Span::styled(
                        " to navigate repositories",
                        Style::default().fg(Colors::MUTED),
                    ),
                ]),
            ]);

            lines
        } else {
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        "No Repository Selected",
                        Style::default().fg(Colors::MUTED),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw("Select a repository from the list to view details"),
                ]),
            ]
        };

        let details = Paragraph::new(detail_lines)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Repository Details (Last 90 Days)"),
            );
        f.render_widget(details, area);
    }

    fn render_languages(&mut self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), // Language list
                Constraint::Percentage(60), // Language details
            ])
            .split(area);

        // Left side: Language list
        let mut items: Vec<ListItem> = Vec::new();

        if data.top_languages.is_empty() {
            items.push(ListItem::new("No languages available"));
        } else {
            // Calculate available width for language list
            let available_width = chunks[0].width.saturating_sub(4) as usize; // Account for borders
            let cpm_count_width = 12; // "123.4 CPM (99)" max width
            let name_width = available_width.saturating_sub(cpm_count_width);

            for (lang_name, avg_cpm, session_count) in data.top_languages.iter() {
                // Truncate name to fit available space
                let display_name = if lang_name.len() > name_width {
                    format!("{}...", &lang_name[..name_width.saturating_sub(3)])
                } else {
                    lang_name.clone()
                };

                let cpm_text = format!("{:.1} CPM ({:2})", avg_cpm, session_count);
                let spaces_needed =
                    available_width.saturating_sub(display_name.len() + cpm_text.len());

                let item_text =
                    format!("{}{}{}", display_name, " ".repeat(spaces_needed), cpm_text);
                items.push(ListItem::new(item_text));
            }
        }

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Languages"),
            )
            .style(Style::default().fg(Colors::TEXT))
            .highlight_style(
                Style::default()
                    .bg(Colors::MUTED)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("► ");

        // Update scrollbar content length
        self.language_scroll_state = self
            .language_scroll_state
            .content_length(data.top_languages.len());

        f.render_stateful_widget(list, chunks[0], &mut self.language_list_state);

        // Render scrollbar for language list
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        f.render_stateful_widget(
            scrollbar,
            chunks[0].inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.language_scroll_state,
        );

        // Right side: Language details
        self.render_language_details(f, chunks[1], data);
    }

    fn render_language_details(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let selected_index = self.language_list_state.selected();

        let detail_lines = if let (Some(_), Some(lang_data)) = (
            selected_index,
            data.top_languages.get(selected_index.unwrap_or(0)),
        ) {
            let lang_name = &lang_data.0;
            let detailed_stats = data.language_stats.get(lang_name);

            let mut lines = vec![
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Language: ", Style::default().fg(Colors::TEXT)),
                    Span::styled(
                        &lang_data.0,
                        Style::default()
                            .fg(Colors::INFO)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
            ];

            if let Some(stats) = detailed_stats {
                lines.extend_from_slice(&[
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "📈 Speed Metrics:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average CPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", stats.avg_cpm),
                            Style::default().fg(Colors::CPM_WPM),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average WPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", stats.avg_wpm),
                            Style::default().fg(Colors::ACTION_KEY),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best CPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", stats.best_cpm),
                            Style::default().fg(Colors::CPM_WPM),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "🎯 Accuracy & Quality:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average Accuracy: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}%", stats.avg_accuracy),
                            Style::default().fg(Colors::ACCURACY),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best Accuracy: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}%", stats.best_accuracy),
                            Style::default().fg(Colors::ACCURACY),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Mistakes: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{}", stats.total_mistakes),
                            Style::default().fg(Colors::ERROR),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "📊 Volume & Activity:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Sessions: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{}", stats.total_sessions),
                            Style::default().fg(Colors::INFO),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Keystrokes: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{}", stats.total_keystrokes),
                            Style::default().fg(Colors::SCORE),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Time: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!(
                                "{}h {}m",
                                stats.total_duration_ms / 3600000,
                                (stats.total_duration_ms % 3600000) / 60000
                            ),
                            Style::default().fg(Colors::SECONDARY),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "🏆 Progress:",
                            Style::default()
                                .fg(Colors::TEXT)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average Score: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.0}", stats.avg_score),
                            Style::default().fg(Colors::SCORE),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Stages: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!(
                                "{}/{} completed",
                                stats.stages_completed, stats.stages_attempted
                            ),
                            Style::default().fg(Colors::BORDER),
                        ),
                    ]),
                ]);
            } else {
                lines.extend_from_slice(&[
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• Average CPM: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", lang_data.1),
                            Style::default().fg(Colors::CPM_WPM),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• WPM Equivalent: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{:.1}", lang_data.1 / 5.0),
                            Style::default().fg(Colors::ACTION_KEY),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• Session Count: ", Style::default().fg(Colors::TEXT)),
                        Span::styled(
                            format!("{}", lang_data.2),
                            Style::default().fg(Colors::ACCURACY),
                        ),
                    ]),
                ]);
            }

            lines.extend_from_slice(&[
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        "Navigation:",
                        Style::default()
                            .fg(Colors::TEXT)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Use ", Style::default().fg(Colors::MUTED)),
                    Span::styled("↑↓ / JK", Style::default().fg(Colors::ACCURACY)),
                    Span::styled(
                        " to navigate languages",
                        Style::default().fg(Colors::MUTED),
                    ),
                ]),
            ]);

            lines
        } else {
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("No Language Selected", Style::default().fg(Colors::MUTED)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw("Select a language from the list to view details"),
                ]),
            ]
        };

        let details = Paragraph::new(detail_lines)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Language Details (Last 90 Days)"),
            );
        f.render_widget(details, area);
    }

    fn render_simple_chart(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        if data.daily_sessions.is_empty() {
            let empty_msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw("No recent activity - start typing to see your activity chart!"),
                ]),
            ])
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Recent Activity"),
            );
            f.render_widget(empty_msg, area);
            return;
        }

        // Calculate how many days can fit in the available width with 0-padding for missing days
        let available_width = area.width.saturating_sub(6) as usize; // Account for borders and padding
        let bar_width = 2u16;
        let bar_gap = 1u16;
        let chars_per_bar = (bar_width + bar_gap) as usize;
        let max_days = (available_width / chars_per_bar).clamp(7, 90); // Between 7-90 days

        // Generate continuous day range with 0 for missing days
        use chrono::{Datelike, Duration, Local};
        let today = Local::now().date_naive();
        let mut continuous_data = Vec::new();

        for i in (0..max_days).rev() {
            let date = today - Duration::days(i as i64);
            let date_key = format!("{:02}-{:02}", date.month(), date.day());
            let count = data.daily_sessions.get(&date_key).copied().unwrap_or(0);
            continuous_data.push((date.day(), count));
        }

        // Create bar chart data
        let bar_data: Vec<(String, u64)> = continuous_data
            .iter()
            .map(|(day, count)| (format!("{:02}", day), *count as u64))
            .collect();

        // Convert to &str references for BarChart
        let bars: Vec<(&str, u64)> = bar_data
            .iter()
            .map(|(day, count)| (day.as_str(), *count))
            .collect();

        // bar_width already defined above

        let max_value = data.daily_sessions.values().max().copied().unwrap_or(0) as u64;
        let total_sessions: usize = data.daily_sessions.values().sum();

        let chart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title(format!(
                        "Recent Activity - {} Days | {} Total Sessions | Max: {}/Day",
                        continuous_data.len(),
                        total_sessions,
                        max_value
                    )),
            )
            .data(&bars)
            .bar_width(bar_width)
            .bar_gap(1) // Small gap for better readability
            .bar_style(Style::default().fg(Colors::CPM_WPM))
            .value_style(
                Style::default()
                    .fg(Colors::ACCURACY)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(Style::default().fg(Colors::INFO))
            .max(max_value);

        f.render_widget(chart, area);
    }

    fn render_bottom_stats(&self, f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Top Repositories
                Constraint::Percentage(50), // Top Languages
            ])
            .split(area);

        // Top Repositories (Last 90 days)
        let mut repo_lines = vec![];

        if data.top_repositories.is_empty() {
            repo_lines.push(Line::from("No repository data available"));
        } else {
            // Calculate available width (subtract borders and padding)
            let available_width = chunks[0].width.saturating_sub(4) as usize; // Account for borders
            let cpm_text_width = 10; // "123.4 CPM" max width
            let index_width = 4; // "99. " max width
            let name_width = available_width.saturating_sub(cpm_text_width + index_width);

            for (i, (repo_name, avg_cpm)) in data.top_repositories.iter().enumerate() {
                // Truncate name to fit available space
                let display_name = if repo_name.len() > name_width {
                    format!("{}...", &repo_name[..name_width.saturating_sub(3)])
                } else {
                    repo_name.clone()
                };

                let index_text = format!("{}. ", i + 1);
                let cpm_text = format!("{:.1} CPM", avg_cpm);

                // Calculate spaces needed to push CPM to the right
                let used_width = 2 + index_text.len() + display_name.len(); // 2 for left padding
                let spaces_needed = available_width.saturating_sub(used_width + cpm_text.len());

                repo_lines.push(Line::from(vec![
                    Span::raw("  "), // Left padding
                    Span::styled(index_text, Style::default().fg(Colors::MUTED)),
                    Span::styled(display_name, Style::default().fg(Colors::INFO)),
                    Span::raw(" ".repeat(spaces_needed)), // Spacer to push CPM right
                    Span::styled(cpm_text, Style::default().fg(Colors::CPM_WPM)),
                ]));
            }
        }

        let repositories = Paragraph::new(repo_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::BORDER))
                .title("Top Repositories (Last 90 Days)"),
        );
        f.render_widget(repositories, chunks[0]);

        // Top Languages (Last 90 days)
        let mut lang_lines = vec![];

        if data.top_languages.is_empty() {
            lang_lines.push(Line::from("No language data available"));
        } else {
            // Calculate available width (subtract borders and padding)
            let available_width = chunks[1].width.saturating_sub(4) as usize; // Account for borders
            let cpm_count_width = 12; // "123.4 CPM (99)" max width
            let index_width = 4; // "99. " max width
            let name_width = available_width.saturating_sub(cpm_count_width + index_width);

            for (i, (lang_name, avg_cpm, _)) in data.top_languages.iter().enumerate() {
                // Truncate name to fit available space
                let display_name = if lang_name.len() > name_width {
                    format!("{}...", &lang_name[..name_width.saturating_sub(3)])
                } else {
                    lang_name.clone()
                };

                let index_text = format!("{}. ", i + 1);
                let cpm_text = format!("{:.1} CPM", avg_cpm);

                // Calculate spaces needed to push CPM to the right
                let used_width = 2 + index_text.len() + display_name.len(); // 2 for left padding
                let spaces_needed = available_width.saturating_sub(used_width + cpm_text.len());

                lang_lines.push(Line::from(vec![
                    Span::raw("  "), // Left padding
                    Span::styled(index_text, Style::default().fg(Colors::MUTED)),
                    Span::styled(display_name, Style::default().fg(Colors::INFO)),
                    Span::raw(" ".repeat(spaces_needed)), // Spacer to push CPM right
                    Span::styled(cpm_text, Style::default().fg(Colors::CPM_WPM)),
                ]));
            }
        }

        let languages = Paragraph::new(lang_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::BORDER))
                .title("Top Languages (Last 90 Days)"),
        );
        f.render_widget(languages, chunks[1]);
    }

    fn render_controls(&self, f: &mut Frame, area: Rect) {
        let controls_line = Line::from(vec![
            Span::styled("[←→/HL]", Style::default().fg(Colors::BORDER)),
            Span::styled(" Switch View  ", Style::default().fg(Colors::TEXT)),
            Span::styled("[R]", Style::default().fg(Colors::SCORE)),
            Span::styled(" Refresh  ", Style::default().fg(Colors::TEXT)),
            Span::styled("[ESC]", Style::default().fg(Colors::ERROR)),
            Span::styled(" Back", Style::default().fg(Colors::TEXT)),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, area);
    }
}

// Extension trait for getting session results and repository data
trait AnalyticsExt {
    fn get_session_result(
        &self,
        session_id: i64,
    ) -> Result<Option<crate::storage::daos::session_dao::SessionResultData>>;
    fn get_repository(
        &self,
        repo_id: i64,
    ) -> Result<Option<crate::storage::daos::StoredRepository>>;
}

impl AnalyticsExt for crate::storage::Database {
    fn get_session_result(
        &self,
        session_id: i64,
    ) -> Result<Option<crate::storage::daos::session_dao::SessionResultData>> {
        use crate::storage::daos::SessionDao;
        let dao = SessionDao::new(self);
        dao.get_session_result(session_id)
    }

    fn get_repository(
        &self,
        repo_id: i64,
    ) -> Result<Option<crate::storage::daos::StoredRepository>> {
        use crate::storage::daos::RepositoryDao;
        let dao = RepositoryDao::new(self);
        dao.get_repository_by_id(repo_id)
    }
}
