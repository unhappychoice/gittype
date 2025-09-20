use crate::game::screens::analytics_screen::AnalyticsData;
use crate::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, Paragraph},
    Frame,
};

pub struct OverviewView;

impl OverviewView {
    pub fn render(f: &mut Frame, area: Rect, data: &AnalyticsData) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Stats summary
                Constraint::Min(3),    // Chart area
                Constraint::Length(8), // Top repositories and languages
            ])
            .split(area);

        Self::render_overview_stats(f, chunks[0], data);
        Self::render_simple_chart(f, chunks[1], data);
        Self::render_bottom_stats(f, chunks[2], data);
    }

    fn render_overview_stats(f: &mut Frame, area: Rect, data: &AnalyticsData) {
        // Overview stats (two lines)
        let overview_text = vec![
            Line::from(vec![
                Span::raw("  "), // Left padding
                Span::styled("Sessions: ", Style::default().fg(Colors::stage_info())),
                Span::styled(
                    data.total_sessions.to_string(),
                    Style::default().fg(Colors::text()),
                ),
                Span::raw("  │  "),
                Span::styled("Avg CPM: ", Style::default().fg(Colors::cpm_wpm())),
                Span::styled(
                    format!("{:.1}", data.avg_cpm),
                    Style::default().fg(Colors::text()),
                ),
                Span::raw("  │  "),
                Span::styled("Best CPM: ", Style::default().fg(Colors::cpm_wpm())),
                Span::styled(
                    format!("{:.1}", data.best_cpm),
                    Style::default().fg(Colors::text()),
                ),
                Span::raw("  │  "),
                Span::styled("Avg Accuracy: ", Style::default().fg(Colors::accuracy())),
                Span::styled(
                    format!("{:.1}%", data.avg_accuracy),
                    Style::default().fg(Colors::text()),
                ),
            ]),
            Line::from(vec![
                Span::styled("Total Time: ", Style::default().fg(Colors::duration())),
                Span::styled(
                    format!("{:.1}h", data.total_time_hours),
                    Style::default().fg(Colors::text()),
                ),
                Span::raw("  │  "),
                Span::styled("Avg Session: ", Style::default().fg(Colors::duration())),
                Span::styled(
                    format!("{:.1}m", data.avg_session_duration),
                    Style::default().fg(Colors::text()),
                ),
                Span::raw("  │  "),
                Span::styled("Total Mistakes: ", Style::default().fg(Colors::error())),
                Span::styled(
                    data.total_mistakes.to_string(),
                    Style::default().fg(Colors::text()),
                ),
                Span::raw("  │  "),
                Span::styled("Repositories: ", Style::default().fg(Colors::action_key())),
                Span::styled(
                    data.top_repositories.len().to_string(),
                    Style::default().fg(Colors::text()),
                ),
            ]),
        ];

        let overview = Paragraph::new(overview_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Overview (Last 7 days)"),
            );
        f.render_widget(overview, area);
    }

    fn render_simple_chart(f: &mut Frame, area: Rect, data: &AnalyticsData) {
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
                    .border_style(Style::default().fg(Colors::border()))
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

        let max_value = data.daily_sessions.values().max().copied().unwrap_or(0) as u64;
        let total_sessions: usize = data.daily_sessions.values().sum();

        let chart = BarChart::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
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
            .bar_style(Style::default().fg(Colors::cpm_wpm()))
            .value_style(
                Style::default()
                    .fg(Colors::accuracy())
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(Style::default().fg(Colors::info()))
            .max(max_value);

        f.render_widget(chart, area);
    }

    fn render_bottom_stats(f: &mut Frame, area: Rect, data: &AnalyticsData) {
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
                    Span::styled(index_text, Style::default().fg(Colors::muted())),
                    Span::styled(display_name, Style::default().fg(Colors::info())),
                    Span::raw(" ".repeat(spaces_needed)), // Spacer to push CPM right
                    Span::styled(cpm_text, Style::default().fg(Colors::cpm_wpm())),
                ]));
            }
        }

        let repositories = Paragraph::new(repo_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::border()))
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
                use crate::extractor::models::language::LanguageRegistry;
                let display_name_full = LanguageRegistry::get_display_name(Some(lang_name));

                // Truncate name to fit available space
                let display_name = if display_name_full.len() > name_width {
                    format!("{}...", &display_name_full[..name_width.saturating_sub(3)])
                } else {
                    display_name_full
                };

                let index_text = format!("{}. ", i + 1);
                let cpm_text = format!("{:.1} CPM", avg_cpm);

                // Calculate spaces needed to push CPM to the right
                let used_width = 2 + index_text.len() + display_name.len(); // 2 for left padding
                let spaces_needed = available_width.saturating_sub(used_width + cpm_text.len());

                lang_lines.push(Line::from(vec![
                    Span::raw("  "), // Left padding
                    Span::styled(index_text, Style::default().fg(Colors::muted())),
                    Span::styled(display_name, Style::default().fg(Colors::info())),
                    Span::raw(" ".repeat(spaces_needed)), // Spacer to push CPM right
                    Span::styled(cpm_text, Style::default().fg(Colors::cpm_wpm())),
                ]));
            }
        }

        let languages = Paragraph::new(lang_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::border()))
                .title("Top Languages (Last 90 Days)"),
        );
        f.render_widget(languages, chunks[1]);
    }
}
