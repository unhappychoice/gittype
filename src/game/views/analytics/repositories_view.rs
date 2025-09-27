use crate::game::screens::analytics_screen::AnalyticsData;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
    Frame,
};

pub struct RepositoriesView;

impl RepositoriesView {
    pub fn render_with_state(
        f: &mut Frame,
        area: Rect,
        data: &AnalyticsData,
        repository_list_state: &mut ListState,
        repository_scroll_state: &mut ScrollbarState,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        // Repository list with StatefulWidget
        let mut items: Vec<ListItem> = Vec::new();

        if data.top_repositories.is_empty() {
            items.push(ListItem::new("No repositories available"));
        } else {
            let available_width = chunks[0].width.saturating_sub(4) as usize;
            let cpm_text_width = 10;
            let name_width = available_width.saturating_sub(cpm_text_width);

            for (repo_name, avg_cpm) in data.top_repositories.iter() {
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
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Repositories"),
            )
            .style(Style::default().fg(Colors::text()))
            .highlight_style(
                Style::default()
                    .bg(Colors::background_secondary())
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("► ");

        // Update scrollbar content length
        *repository_scroll_state =
            repository_scroll_state.content_length(data.top_repositories.len());

        // Render with stateful widget
        f.render_stateful_widget(list, chunks[0], repository_list_state);

        // Render scrollbar
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
            repository_scroll_state,
        );

        // Right side: Repository details
        Self::render_repository_details(f, chunks[1], data, repository_list_state);
    }

    fn render_repository_details(
        f: &mut Frame,
        area: Rect,
        data: &AnalyticsData,
        repository_list_state: &ListState,
    ) {
        let selected_index = repository_list_state.selected();

        let detail_lines = if let (Some(_), Some(repo_data)) = (
            selected_index,
            data.top_repositories.get(selected_index.unwrap_or(0)),
        ) {
            let repo_name = &repo_data.0;
            let detailed_stats = data.repository_stats.get(repo_name);

            let mut lines = vec![
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Repository: ", Style::default().fg(Colors::text())),
                    Span::styled(
                        &repo_data.0,
                        Style::default()
                            .fg(Colors::info())
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
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average CPM: ", Style::default().fg(Colors::cpm_wpm())),
                        Span::styled(
                            format!("{:.1}", stats.avg_cpm),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average WPM: ", Style::default().fg(Colors::cpm_wpm())),
                        Span::styled(
                            format!("{:.1}", stats.avg_wpm),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best CPM: ", Style::default().fg(Colors::cpm_wpm())),
                        Span::styled(
                            format!("{:.1}", stats.best_cpm),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "🎯 Accuracy & Quality:",
                            Style::default()
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(
                            "• Average Accuracy: ",
                            Style::default().fg(Colors::accuracy()),
                        ),
                        Span::styled(
                            format!("{:.1}%", stats.avg_accuracy),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best Accuracy: ", Style::default().fg(Colors::accuracy())),
                        Span::styled(
                            format!("{:.1}%", stats.best_accuracy),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Mistakes: ", Style::default().fg(Colors::error())),
                        Span::styled(
                            format!("{}", stats.total_mistakes),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "📊 Volume & Activity:",
                            Style::default()
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(
                            "• Total Sessions: ",
                            Style::default().fg(Colors::stage_info()),
                        ),
                        Span::styled(
                            format!("{}", stats.total_sessions),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(
                            "• Total Keystrokes: ",
                            Style::default().fg(Colors::stage_info()),
                        ),
                        Span::styled(
                            format!("{}", stats.total_keystrokes),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Time: ", Style::default().fg(Colors::duration())),
                        Span::styled(
                            format!(
                                "{}h {}m",
                                stats.total_duration_ms / 3600000,
                                (stats.total_duration_ms % 3600000) / 60000
                            ),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            "🏆 Progress:",
                            Style::default()
                                .fg(Colors::text())
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average Score: ", Style::default().fg(Colors::score())),
                        Span::styled(
                            format!("{:.0}", stats.avg_score),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(
                            "• Total Stages: ",
                            Style::default().fg(Colors::stage_info()),
                        ),
                        Span::styled(
                            format!(
                                "{}/{} completed",
                                stats.stages_completed, stats.stages_attempted
                            ),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                ]);
            } else {
                lines.extend_from_slice(&[
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• Average CPM: ", Style::default().fg(Colors::cpm_wpm())),
                        Span::styled(
                            format!("{:.1}", repo_data.1),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• WPM Equivalent: ", Style::default().fg(Colors::cpm_wpm())),
                        Span::styled(
                            format!("{:.1}", repo_data.1 / 5.0),
                            Style::default().fg(Colors::text()),
                        ),
                    ]),
                ]);
            }

            lines
        } else {
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        "No Repository Selected",
                        Style::default().fg(Colors::text_secondary()),
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
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Repository Details (Last 90 Days)"),
            );
        f.render_widget(details, area);
    }
}
