use super::repo_utils;
use crate::extractor::models::language::LanguageRegistry;
use crate::storage::daos::repository_dao::StoredRepositoryWithLanguages;
use crate::ui::colors::Colors;
use crate::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
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

pub fn render_repo_play_ui(
    repositories: Vec<StoredRepositoryWithLanguages>,
) -> Result<Option<usize>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // State for list selection
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let result: Result<Option<usize>> = loop {
        terminal.draw(|f| {
            // Add horizontal padding like record_screen
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

            // Header - simplified like history_screen
            let header_lines = vec![Line::from(vec![
                Span::raw("  "), // Left padding
                Span::styled(
                    "Select Repository to Play",
                    Style::default()
                        .fg(Colors::info())
                        .add_modifier(Modifier::BOLD),
                ),
            ])];

            let header = Paragraph::new(header_lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("GitType"),
            );
            f.render_widget(header, chunks[0]);

            // Repository list
            let items: Vec<ListItem> = repositories
                .iter()
                .map(|repo| {
                    let repo_name = format!("{}/{}", repo.user_name, repo.repository_name);
                    let is_cached = repo_utils::is_repository_cached(&repo.remote_url);
                    let cache_indicator = if is_cached { "●" } else { "○" };
                    let cache_color = if is_cached {
                        Colors::success()
                    } else {
                        Colors::text_secondary()
                    };

                    let language_spans = if repo.languages.is_empty() {
                        vec![Span::styled(
                            "No challenges",
                            Style::default().fg(Colors::text_secondary()),
                        )]
                    } else {
                        let mut spans = Vec::new();
                        for (i, lang) in repo.languages.iter().enumerate() {
                            if i > 0 {
                                spans.push(Span::styled(", ", Style::default().fg(Colors::text_secondary())));
                            }
                            spans.push(Span::styled(
                                LanguageRegistry::get_display_name(Some(lang)),
                                Style::default().fg(LanguageRegistry::get_color(Some(lang))),
                            ));
                        }
                        spans
                    };

                    let mut line_spans = vec![
                        Span::raw("  "), // Left padding for list items
                        Span::styled(cache_indicator, Style::default().fg(cache_color)),
                        Span::raw(" "),
                        Span::styled(
                            format!("{:<32}", repo_name),
                            Style::default()
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" "),
                    ];
                    line_spans.extend(language_spans);

                    ListItem::new(Line::from(line_spans))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::border()))
                        .title("Played Repositories")
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

            // Controls at bottom - using semantic color for navigation keys
            let controls_line = Line::from(vec![
                Span::styled("[↑↓/JK]", Style::default().fg(Colors::key_navigation())),
                Span::styled(" Navigate  ", Style::default().fg(Colors::text())),
                Span::styled("[SPACE]", Style::default().fg(Colors::key_action())),
                Span::styled(" Play  ", Style::default().fg(Colors::text())),
                Span::styled("[ESC]", Style::default().fg(Colors::key_back())),
                Span::styled(" Return  ", Style::default().fg(Colors::text())),
                Span::styled("●", Style::default().fg(Colors::success())),
                Span::styled(" Cached ", Style::default().fg(Colors::text())),
                Span::styled("○", Style::default().fg(Colors::text_secondary())),
                Span::styled(" Not Cached", Style::default().fg(Colors::text())),
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
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}
