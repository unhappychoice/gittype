use crate::game::screens::analytics_screen::AnalyticsData;
use crate::ui::Colors;
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

pub struct LanguagesView;

impl LanguagesView {
    pub fn render_with_state(
        f: &mut Frame,
        area: Rect,
        data: &AnalyticsData,
        language_list_state: &mut ListState,
        language_scroll_state: &mut ScrollbarState,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        // Language list with StatefulWidget
        let mut items: Vec<ListItem> = Vec::new();

        if data.top_languages.is_empty() {
            items.push(ListItem::new("No languages available"));
        } else {
            let available_width = chunks[0].width.saturating_sub(4) as usize;
            let cpm_count_width = 12;
            let name_width = available_width.saturating_sub(cpm_count_width);

            for (lang_name, avg_cpm, session_count) in data.top_languages.iter() {
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
        *language_scroll_state = language_scroll_state.content_length(data.top_languages.len());

        // Render with stateful widget
        f.render_stateful_widget(list, chunks[0], language_list_state);

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
            language_scroll_state,
        );

        // Right side: Language details
        Self::render_language_details(f, chunks[1], data, language_list_state);
    }

    fn render_language_details(
        f: &mut Frame,
        area: Rect,
        data: &AnalyticsData,
        language_list_state: &ListState,
    ) {
        let selected_index = language_list_state.selected();

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
                        Span::styled("• Average CPM: ", Style::default().fg(Colors::CPM_WPM)),
                        Span::styled(
                            format!("{:.1}", stats.avg_cpm),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Average WPM: ", Style::default().fg(Colors::CPM_WPM)),
                        Span::styled(
                            format!("{:.1}", stats.avg_wpm),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best CPM: ", Style::default().fg(Colors::CPM_WPM)),
                        Span::styled(
                            format!("{:.1}", stats.best_cpm),
                            Style::default().fg(Colors::TEXT),
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
                        Span::styled(
                            "• Average Accuracy: ",
                            Style::default().fg(Colors::ACCURACY),
                        ),
                        Span::styled(
                            format!("{:.1}%", stats.avg_accuracy),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Best Accuracy: ", Style::default().fg(Colors::ACCURACY)),
                        Span::styled(
                            format!("{:.1}%", stats.best_accuracy),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Mistakes: ", Style::default().fg(Colors::ERROR)),
                        Span::styled(
                            format!("{}", stats.total_mistakes),
                            Style::default().fg(Colors::TEXT),
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
                        Span::styled(
                            "• Total Sessions: ",
                            Style::default().fg(Colors::STAGE_INFO),
                        ),
                        Span::styled(
                            format!("{}", stats.total_sessions),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(
                            "• Total Keystrokes: ",
                            Style::default().fg(Colors::STAGE_INFO),
                        ),
                        Span::styled(
                            format!("{}", stats.total_keystrokes),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Time: ", Style::default().fg(Colors::DURATION)),
                        Span::styled(
                            format!(
                                "{}h {}m",
                                stats.total_duration_ms / 3600000,
                                (stats.total_duration_ms % 3600000) / 60000
                            ),
                            Style::default().fg(Colors::TEXT),
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
                        Span::styled("• Average Score: ", Style::default().fg(Colors::SCORE)),
                        Span::styled(
                            format!("{:.0}", stats.avg_score),
                            Style::default().fg(Colors::TEXT),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled("• Total Stages: ", Style::default().fg(Colors::STAGE_INFO)),
                        Span::styled(
                            format!(
                                "{}/{} completed",
                                stats.stages_completed, stats.stages_attempted
                            ),
                            Style::default().fg(Colors::TEXT),
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
                            Style::default().fg(Colors::CPM_WPM),
                        ),
                    ]),
                    Line::from(vec![
                        Span::raw("  "),
                        Span::styled("• Session Count: ", Style::default().fg(Colors::STAGE_INFO)),
                        Span::styled(
                            format!("{}", lang_data.2),
                            Style::default().fg(Colors::TEXT),
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
                    Span::styled(" to navigate languages", Style::default().fg(Colors::MUTED)),
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
}
