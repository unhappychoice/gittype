use crate::domain::models::storage::StoredRepositoryWithLanguages;
use crate::domain::models::Languages;
use crate::infrastructure::git::RemoteGitRepositoryClient;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding},
    Frame,
};

pub struct RepositoryListView;

impl RepositoryListView {
    pub fn render(frame: &mut Frame, area: Rect, repositories: &[StoredRepositoryWithLanguages]) {
        let repo_width = 35;
        let lang_width = 25;

        let items: Vec<ListItem> = repositories
            .iter()
            .map(|repo| {
                let repo_name = format!("{}/{}", repo.user_name, repo.repository_name);
                let is_cached = RemoteGitRepositoryClient::is_repository_cached(&repo.remote_url);
                let cache_indicator = if is_cached { "●" } else { "○" };

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
                        let lang_name = Languages::get_display_name(Some(lang));
                        if current_length + lang_name.len() <= lang_width {
                            line_spans.push(Span::styled(
                                lang_name.clone(),
                                Style::default().fg(Languages::get_color(Some(lang))),
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
                    repo.http_url(),
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
        frame.render_widget(list, area);
    }
}
