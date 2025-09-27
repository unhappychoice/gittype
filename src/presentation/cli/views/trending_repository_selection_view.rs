use crate::infrastructure::cache::TrendingRepository;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
    Terminal,
};
use std::io;

pub fn render_trending_ui(repositories: Vec<TrendingRepository>) -> Result<Option<usize>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // State for list selection
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let result: Result<Option<usize>> = loop {
        terminal.draw(|f| {
            // Add horizontal padding
            let outer_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(2), // Left padding
                    Constraint::Min(1),    // Main content
                    Constraint::Length(2), // Right padding
                ])
                .split(f.area());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(1),    // Repository list
                    Constraint::Length(1), // Controls at bottom
                ])
                .split(outer_chunks[1]);

            // Header
            let header_lines = vec![Line::from(vec![
                Span::raw("  "), // Left padding
                Span::styled(
                    "🔥 Select Trending Repository to Play",
                    Style::default()
                        .fg(Colors::info())
                        .add_modifier(Modifier::BOLD),
                ),
            ])];

            let sub_header = vec![Line::from(vec![Span::raw(
                "  🔥 Currently trending repositories",
            )])];

            let mut all_header_lines = header_lines;
            all_header_lines.extend(sub_header);
            let header = Paragraph::new(all_header_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("GitType - Trending"),
            );
            f.render_widget(header, chunks[0]);

            // Repository list
            let items: Vec<ListItem> = repositories
                .iter()
                .enumerate()
                .map(|(i, repo)| {
                    let language = repo.primary_language.as_deref().unwrap_or("Unknown");
                    let description = repo.description.as_deref().unwrap_or("No description");

                    // Truncate repository name if too long (safe for UTF-8)
                    let truncated_repo_name = if repo.repo_name.chars().count() > 35 {
                        let truncated: String = repo.repo_name.chars().take(32).collect();
                        format!("{}...", truncated)
                    } else {
                        repo.repo_name.clone()
                    };

                    // Truncate description if too long (safe for UTF-8)
                    let truncated_desc = if description.chars().count() > 50 {
                        let truncated: String = description.chars().take(47).collect();
                        format!("{}...", truncated)
                    } else {
                        description.to_string()
                    };

                    let line_spans = vec![
                        Span::styled(
                            format!("{:2}. ", i + 1),
                            Style::default().fg(Colors::text_secondary()),
                        ),
                        Span::styled(
                            format!("{:<35}", truncated_repo_name),
                            Style::default()
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{:>12} ", format!("({})", language)),
                            Style::default().fg(Colors::success()),
                        ),
                        Span::styled(
                            truncated_desc,
                            Style::default().fg(Colors::text_secondary()),
                        ),
                    ];

                    ListItem::new(Line::from(line_spans))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::border()))
                        .title("Trending Repositories")
                        .title_style(
                            Style::default()
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        )
                        .padding(Padding::uniform(1)),
                )
                .style(Style::default().fg(Colors::text()))
                .highlight_style(
                    Style::default()
                        .bg(Colors::background_secondary())
                        .add_modifier(Modifier::BOLD),
                );
            f.render_stateful_widget(list, chunks[1], &mut list_state);

            // Controls at bottom
            let controls_line = Line::from(vec![
                Span::styled("[↑↓/JK]", Style::default().fg(Colors::key_navigation())),
                Span::styled(" Navigate  ", Style::default().fg(Colors::text())),
                Span::styled("[SPACE]", Style::default().fg(Colors::key_action())),
                Span::styled(" Play  ", Style::default().fg(Colors::text())),
                Span::styled("[ESC]", Style::default().fg(Colors::key_back())),
                Span::styled(" Return", Style::default().fg(Colors::text())),
            ]);
            let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
            f.render_widget(controls, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => break Ok(None),
                    KeyCode::Char('j') | KeyCode::Down => {
                        if let Some(selected) = list_state.selected() {
                            if selected < repositories.len() - 1 {
                                list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if let Some(selected) = list_state.selected() {
                            if selected > 0 {
                                list_state.select(Some(selected - 1));
                            }
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(selected) = list_state.selected() {
                            break Ok(Some(selected));
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
