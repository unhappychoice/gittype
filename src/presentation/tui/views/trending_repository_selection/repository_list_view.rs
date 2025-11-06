use crate::domain::repositories::trending_repository::TrendingRepositoryInfo;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Padding},
    Frame,
};

pub struct RepositoryListView;

impl RepositoryListView {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        repositories: &[TrendingRepositoryInfo],
        list_state: &mut ListState,
        colors: &Colors,
    ) {
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
                        Style::default().fg(colors.text_secondary()),
                    ),
                    Span::styled(
                        format!("{:<35}", truncated_repo_name),
                        Style::default()
                            .fg(colors.text())
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:>12} ", format!("({})", language)),
                        Style::default().fg(colors.success()),
                    ),
                    Span::styled(truncated_desc, Style::default().fg(colors.text_secondary())),
                ];

                ListItem::new(Line::from(line_spans))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .title("Trending Repositories")
                    .title_style(
                        Style::default()
                            .fg(colors.text())
                            .add_modifier(Modifier::BOLD),
                    )
                    .padding(Padding::uniform(1)),
            )
            .style(Style::default().fg(colors.text()))
            .highlight_style(
                Style::default()
                    .bg(colors.background_secondary())
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_stateful_widget(list, area, list_state);
    }
}
