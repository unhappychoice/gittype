use super::repo_utils;
use crate::domain::services::extractor::LanguageRegistry;
use crate::storage::daos::repository_dao::StoredRepositoryWithLanguages;
use crate::presentation::ui::colors::Colors;
use crate::Result;
use crossterm::{
    execute,
    style::{ResetColor, SetForegroundColor},
};
use ratatui::{
    backend::TestBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
    Terminal,
};
use std::io::stdout;

pub fn render_repo_list(repositories: Vec<StoredRepositoryWithLanguages>) -> Result<()> {
    // Calculate terminal dimensions
    let width = 120;
    let height = repositories.len() as u16 + 10; // Extra space for headers and borders

    // Create test backend to render widgets to buffer
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Cache info
                Constraint::Length(1), // Spacer
                Constraint::Min(1),    // Repository list
                Constraint::Length(3), // Legend
            ])
            .split(f.area());

        // Main header
        let header_line = Line::from(vec![Span::styled(
            "GitType - Played Repositories",
            Style::default()
                .fg(Colors::info())
                .add_modifier(Modifier::BOLD),
        )]);
        let header = Paragraph::new(header_line)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border())),
            );
        f.render_widget(header, chunks[0]);

        // Cache directory info
        let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let cache_dir = home_dir.join(".gittype").join("repos");
        let cache_line = Line::from(vec![
            Span::styled(
                "Cache Directory: ",
                Style::default().fg(Colors::text_secondary()),
            ),
            Span::styled(
                cache_dir.to_string_lossy().to_string(),
                Style::default().fg(Colors::text()),
            ),
        ]);
        let cache_info = Paragraph::new(cache_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::border())),
        );
        f.render_widget(cache_info, chunks[2]);

        // Repository list
        let repo_width = 35;
        let lang_width = 25;

        let items: Vec<ListItem> = repositories
            .iter()
            .map(|repo| {
                let repo_name = format!("{}/{}", repo.user_name, repo.repository_name);
                let is_cached = repo_utils::is_repository_cached(&repo.remote_url);
                let cache_indicator = if is_cached { "●" } else { "○" };

                // Convert various git URL formats to HTTP URLs
                let url = repo_utils::format_http_url(&repo.remote_url);

                let mut line_spans = vec![
                    Span::styled(
                        format!("{} ", cache_indicator),
                        Style::default().fg(if is_cached {
                            Colors::success()
                        } else {
                            Colors::text_secondary()
                        }),
                    ),
                    Span::styled(
                        format!("{:<width$}", repo_name, width = repo_width),
                        Style::default()
                            .fg(Colors::text())
                            .add_modifier(Modifier::BOLD),
                    ),
                ];

                // Add language spans with proper color but fixed total width
                if repo.languages.is_empty() {
                    line_spans.push(Span::styled(
                        format!("{:<width$}", "No challenges", width = lang_width),
                        Style::default().fg(Colors::text_secondary()),
                    ));
                } else {
                    let mut current_length = 0;
                    for (i, lang) in repo.languages.iter().enumerate() {
                        if i > 0 {
                            if current_length + 2 <= lang_width {
                                line_spans.push(Span::styled(
                                    ", ",
                                    Style::default().fg(Colors::text_secondary()),
                                ));
                                current_length += 2;
                            } else {
                                break;
                            }
                        }
                        let lang_name = LanguageRegistry::get_display_name(Some(lang));
                        if current_length + lang_name.len() <= lang_width {
                            line_spans.push(Span::styled(
                                lang_name.clone(),
                                Style::default().fg(LanguageRegistry::get_color(Some(lang))),
                            ));
                            current_length += lang_name.len();
                        } else if current_length + 3 <= lang_width {
                            line_spans.push(Span::styled(
                                "...",
                                Style::default().fg(Colors::text_secondary()),
                            ));
                            current_length += 3;
                            break;
                        } else {
                            break;
                        }
                    }
                    // Pad to fixed width
                    if current_length < lang_width {
                        line_spans.push(Span::raw(" ".repeat(lang_width - current_length)));
                    }
                }

                line_spans.push(Span::styled(" ", Style::default()));
                line_spans.push(Span::styled(
                    url,
                    Style::default().fg(Colors::text_secondary()),
                ));

                ListItem::new(Line::from(line_spans))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Repository List")
                    .title_style(
                        Style::default()
                            .fg(Colors::text())
                            .add_modifier(Modifier::BOLD),
                    )
                    .padding(Padding::horizontal(1)),
            )
            .style(Style::default().fg(Colors::text()));
        f.render_widget(list, chunks[4]);

        // Legend
        let legend_line = Line::from(vec![
            Span::styled("●", Style::default().fg(Colors::success())),
            Span::styled(" Cached  ", Style::default().fg(Colors::text())),
            Span::styled("○", Style::default().fg(Colors::text_secondary())),
            Span::styled(" Not Cached", Style::default().fg(Colors::text())),
        ]);
        let legend = Paragraph::new(legend_line)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border())),
            );
        f.render_widget(legend, chunks[5]);
    })?;

    // Convert buffer to string and print with colors
    let buffer = terminal.backend().buffer();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = &buffer[(x, y)];
            // Convert ratatui color to crossterm using Colors utility
            let fg_color = Colors::to_crossterm(cell.fg);
            execute!(stdout(), SetForegroundColor(fg_color))?;
            print!("{}", cell.symbol());
            execute!(stdout(), ResetColor)?;
        }
        println!();
    }

    Ok(())
}
