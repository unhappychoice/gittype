use crate::domain::models::storage::StoredRepositoryWithLanguages;
use crate::domain::models::Languages;
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
        repositories: &[(StoredRepositoryWithLanguages, bool)],
        list_state: &mut ListState,
        colors: &Colors,
    ) {
        let items: Vec<ListItem> = repositories
            .iter()
            .map(|(repo, is_cached)| {
                let repo_name = format!("{}/{}", repo.user_name, repo.repository_name);
                let cache_indicator = if *is_cached { "●" } else { "○" };
                let cache_color = if *is_cached {
                    colors.success()
                } else {
                    colors.text_secondary()
                };

                let language_spans = if repo.languages.is_empty() {
                    vec![Span::styled(
                        "No challenges",
                        Style::default().fg(colors.text_secondary()),
                    )]
                } else {
                    let mut spans = Vec::new();
                    for (i, lang) in repo.languages.iter().enumerate() {
                        if i > 0 {
                            spans.push(Span::styled(
                                ", ",
                                Style::default().fg(colors.text_secondary()),
                            ));
                        }
                        spans.push(Span::styled(
                            Languages::get_display_name(Some(lang)),
                            Style::default().fg(colors.info()),
                        ));
                    }
                    spans
                };

                let mut line_spans = vec![
                    Span::raw("  "),
                    Span::styled(cache_indicator, Style::default().fg(cache_color)),
                    Span::raw(" "),
                    Span::styled(
                        format!("{:<32}", repo_name),
                        Style::default()
                            .fg(colors.text())
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
                    .border_style(Style::default().fg(colors.border()))
                    .title("Played Repositories")
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
