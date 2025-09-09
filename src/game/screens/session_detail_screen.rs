use crate::game::screen_manager::{Screen, ScreenTransition, UpdateStrategy};
use std::io::Stdout;
use crate::storage::{
    daos::{
        session_dao::{SessionResultData, SessionStageResult},
        StoredRepository, StoredSession,
    },
    repositories::SessionRepository,
};
use crate::ui::Colors;
use crate::Result;
use chrono::{DateTime, Local};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::stdout;

#[derive(Debug, Clone)]
pub struct SessionDisplayData {
    pub session: StoredSession,
    pub repository: Option<StoredRepository>,
    pub session_result: Option<SessionResultData>,
}

pub enum SessionDetailAction {
    Return,
}

pub struct SessionDetailScreen {
    session_data: SessionDisplayData,
    stage_results: Vec<SessionStageResult>,
    stage_scroll_offset: usize,
}

impl SessionDetailScreen {
    pub fn show(session_data: SessionDisplayData) -> Result<SessionDetailAction> {
        let mut screen = Self::new(session_data)?;
        screen.run()
    }

    fn new(session_data: SessionDisplayData) -> Result<Self> {
        // Load stage results for this session
        let session_repo = SessionRepository::new()?;
        let stage_results = session_repo.get_session_stage_results(session_data.session.id)?;

        Ok(Self {
            session_data,
            stage_results,
            stage_scroll_offset: 0,
        })
    }

    fn run(&mut self) -> Result<SessionDetailAction> {
        // Don't use alternate screen - just clear the current screen
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        // Clear the screen once
        terminal.clear()?;

        let result = self.run_app(&mut terminal);

        // Clear screen when exiting to ensure clean state
        terminal.clear()?;

        result
    }

    fn run_app(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<SessionDetailAction> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(SessionDetailAction::Return),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(SessionDetailAction::Return);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.scroll_up();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.scroll_down();
                    }
                    _ => {}
                }
            }
        }
    }

    fn scroll_down(&mut self) {
        if !self.stage_results.is_empty()
            && self.stage_scroll_offset < self.stage_results.len().saturating_sub(1)
        {
            self.stage_scroll_offset += 1;
        }
    }

    fn scroll_up(&mut self) {
        if self.stage_scroll_offset > 0 {
            self.stage_scroll_offset -= 1;
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        // Add horizontal padding
        let outer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .split(f.area());

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(outer_chunks[1]);

        let title = Paragraph::new("Session Details")
            .style(
                Style::default()
                    .fg(Colors::INFO)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);
        f.render_widget(title, main_chunks[0]);

        // Split content area into top and bottom sections
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(12), Constraint::Min(1)])
            .split(main_chunks[1]);

        // Split top section into two columns
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_chunks[0]);

        // Render top blocks
        self.render_session_info(f, top_chunks[0]);
        self.render_performance_metrics(f, top_chunks[1]);

        // Render stage details at the bottom
        self.render_stage_details(f, content_chunks[1]);

        // Controls at the bottom
        let controls_line = Line::from(vec![
            Span::styled("[↑↓/JK] Scroll Stages  ", Style::default().fg(Colors::TEXT)),
            Span::styled("[ESC]", Style::default().fg(Colors::ERROR)),
            Span::styled(" Back", Style::default().fg(Colors::TEXT)),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, main_chunks[2]);
    }

    fn render_session_info(&self, f: &mut Frame, area: ratatui::prelude::Rect) {
        let mut info_lines = Vec::new();

        info_lines.push(Line::from(""));

        if let Some(ref repo) = self.session_data.repository {
            info_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Repository: ", Style::default().fg(Colors::ACCURACY)),
                Span::raw(format!("{}/{}", repo.user_name, repo.repository_name)),
            ]));
        }

        // Session basic info
        let local_time: DateTime<Local> = self.session_data.session.started_at.into();
        info_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("Started: ", Style::default().fg(Colors::ACCURACY)),
            Span::raw(local_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        ]));

        if let Some(ref branch) = self.session_data.session.branch {
            info_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Branch: ", Style::default().fg(Colors::ACCURACY)),
                Span::raw(branch.clone()),
            ]));
        }

        if let Some(ref commit) = self.session_data.session.commit_hash {
            info_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Commit: ", Style::default().fg(Colors::ACCURACY)),
                Span::raw(commit[..std::cmp::min(commit.len(), 12)].to_string()),
            ]));
        }

        let session_info = Paragraph::new(info_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Session"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(session_info, area);
    }

    fn render_performance_metrics(&self, f: &mut Frame, area: ratatui::prelude::Rect) {
        let mut metrics_lines = Vec::new();

        metrics_lines.push(Line::from(""));

        if let Some(ref result) = self.session_data.session_result {
            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Tier/Rank: ", Style::default().fg(Colors::STAGE_INFO)),
                Span::styled(
                    format!(
                        "{}/{}",
                        result.tier_name.clone().unwrap_or("unknown".to_string()),
                        result.rank_name.clone().unwrap_or("unknown".to_string())
                    ),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Score: ", Style::default().fg(Colors::SCORE)),
                Span::styled(
                    format!("{:.1}", result.score),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("CPM: ", Style::default().fg(Colors::CPM_WPM)),
                Span::styled(
                    format!("{:.1}", result.cpm),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("WPM: ", Style::default().fg(Colors::CPM_WPM)),
                Span::styled(
                    format!("{:.1}", result.wpm),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Accuracy: ", Style::default().fg(Colors::ACCURACY)),
                Span::styled(
                    format!("{:.1}%", result.accuracy),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Duration: ", Style::default().fg(Colors::DURATION)),
                Span::styled(
                    format!(
                        "{}m {}s",
                        result.duration_ms / 60000,
                        (result.duration_ms % 60000) / 1000
                    ),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Completed Stage: ", Style::default().fg(Colors::STAGE_INFO)),
                Span::styled(
                    result.stages_completed.to_string(),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("/"),
                Span::styled(
                    result.stages_attempted.to_string(),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            if result.stages_skipped > 0 {
                metrics_lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Skipped: ", Style::default().fg(Colors::ERROR)),
                    Span::styled(
                        result.stages_skipped.to_string(),
                        Style::default().fg(Colors::TEXT),
                    ),
                ]));
            }
        } else {
            metrics_lines.push(Line::from("No performance data available"));
        }

        let performance = Paragraph::new(metrics_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title("Performance"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(performance, area);
    }

    fn render_stage_details(&mut self, f: &mut Frame, area: ratatui::prelude::Rect) {
        if self.stage_results.is_empty() {
            let empty_msg = Paragraph::new("No stage data available")
                .style(Style::default().fg(Colors::MUTED))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::BORDER))
                        .title("Stage Details"),
                );
            f.render_widget(empty_msg, area);
            return;
        }

        // Create detailed text for each stage
        let mut stage_text_lines = Vec::new();

        let visible_height = area.height.saturating_sub(3) as usize; // Account for borders and title
        let start_idx = self.stage_scroll_offset;
        let end_idx = (start_idx + visible_height.saturating_sub(2)).min(self.stage_results.len());

        stage_text_lines.push(Line::from(""));

        for (i, stage) in self.stage_results[start_idx..end_idx].iter().enumerate() {
            let actual_idx = start_idx + i;

            // Stage header
            let status = if stage.was_failed {
                "FAILED"
            } else if stage.was_skipped {
                "SKIPPED"
            } else {
                "COMPLETED"
            };

            let status_color = if stage.was_failed {
                Colors::FAILED
            } else if stage.was_skipped {
                Colors::SKIPPED
            } else {
                Colors::COMPLETED
            };

            stage_text_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!("Stage #{} ", stage.stage_number),
                    Style::default()
                        .fg(Colors::INFO)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("[{}]", status), Style::default().fg(status_color)),
            ]));

            if let (Some(ref file_path), Some(start), Some(end)) =
                (stage.file_path.clone(), stage.start_line, stage.end_line)
            {
                stage_text_lines.push(Line::from(vec![
                    Span::raw("    "),
                    Span::styled("File: ", Style::default().fg(Colors::STAGE_INFO)),
                    Span::raw(format!("{}:{}-{}", file_path, start, end)),
                ]));
            }

            stage_text_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("Score: ", Style::default().fg(Colors::SCORE)),
                Span::styled(
                    format!("{:.1}", stage.score),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("CPM: ", Style::default().fg(Colors::CPM_WPM)),
                Span::styled(
                    format!("{:.1}", stage.cpm),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("    "),
                Span::styled("WPM: ", Style::default().fg(Colors::CPM_WPM)),
                Span::styled(
                    format!("{:.1}", stage.wpm),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            stage_text_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("Keystrokes: ", Style::default().fg(Colors::STAGE_INFO)),
                Span::styled(
                    format!("{}", stage.keystrokes),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("Mistakes: ", Style::default().fg(Colors::ERROR)),
                Span::styled(
                    format!("{}", stage.mistakes),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("Accuracy: ", Style::default().fg(Colors::ACCURACY)),
                Span::styled(
                    format!("{:.1}%", stage.accuracy),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("Duration: ", Style::default().fg(Colors::DURATION)),
                Span::styled(
                    format!("{}ms", stage.duration_ms),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            if actual_idx < self.stage_results.len() - 1 && i < end_idx - start_idx - 1 {
                stage_text_lines.push(Line::raw(""));
            }
        }

        // Add scroll indicator
        let scroll_info = if self.stage_results.len() > visible_height.saturating_sub(2) {
            format!(
                " ({}/{} stages shown, ↑↓ to scroll)",
                end_idx - start_idx,
                self.stage_results.len()
            )
        } else {
            format!(" ({} stages)", self.stage_results.len())
        };

        let stage_paragraph = Paragraph::new(stage_text_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title(format!("Stage Details{}", scroll_info))
                    .title_style(
                        Style::default()
                            .fg(Colors::TEXT)
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(stage_paragraph, area);
    }
}

// Basic Screen trait implementation for ScreenManager compatibility
pub struct ScreenState {
    should_exit: bool,
}

impl ScreenState {
    pub fn new() -> Self {
        Self { should_exit: false }
    }
}

impl Screen for ScreenState {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> crate::Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.should_exit = true;
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_exit = true;
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm(&self, _stdout: &mut Stdout) -> crate::Result<()> {
        // TODO: Use real SessionDisplayData instead of dummy
        let dummy_session = crate::storage::daos::StoredSession {
            id: 1,
            started_at: chrono::Utc::now(),
            completed_at: None,
            branch: Some("main".to_string()),
            commit_hash: Some("dummy".to_string()),
            is_dirty: false,
            game_mode: "normal".to_string(),
            repository_id: None,
            max_stages: Some(1),
            difficulty_level: Some("easy".to_string()),
            time_limit_seconds: None,
        };
        let dummy_data = SessionDisplayData {
            session: dummy_session,
            repository: None,
            session_result: None,
        };
        let _ = SessionDetailScreen::show(dummy_data);
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }
}
