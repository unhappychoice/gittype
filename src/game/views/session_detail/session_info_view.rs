use crate::storage::daos::StoredRepository;
use crate::storage::daos::StoredSession;
use crate::presentation::ui::Colors;
use chrono::{DateTime, Local};
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct SessionInfoView;

impl SessionInfoView {
    pub fn render(
        f: &mut Frame,
        area: ratatui::prelude::Rect,
        session: &StoredSession,
        repository: Option<&StoredRepository>,
    ) {
        let mut info_lines = Vec::new();

        info_lines.push(Line::from(""));

        if let Some(repo) = repository {
            info_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Repository: ", Style::default().fg(Colors::accuracy())),
                Span::raw(format!("{}/{}", repo.user_name, repo.repository_name)),
            ]));
        }

        let local_time: DateTime<Local> = session.started_at.into();
        info_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("Started: ", Style::default().fg(Colors::accuracy())),
            Span::raw(local_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        ]));

        if let Some(ref branch) = session.branch {
            info_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Branch: ", Style::default().fg(Colors::accuracy())),
                Span::raw(branch.clone()),
            ]));
        }

        if let Some(ref commit) = session.commit_hash {
            info_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Commit: ", Style::default().fg(Colors::accuracy())),
                Span::raw(commit[..std::cmp::min(commit.len(), 12)].to_string()),
            ]));
        }

        let session_info = Paragraph::new(info_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Session"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(session_info, area);
    }
}
