use crate::domain::models::{GitRepository, Rank, SessionResult};
use crate::presentation::ui::Colors;
use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct PreviewView;

impl PreviewView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        metrics: &SessionResult,
        repo_info: &Option<GitRepository>,
    ) {
        let best_rank = Rank::for_score(metrics.session_score);
        let total_mistakes = metrics.valid_mistakes + metrics.invalid_mistakes;

        let mut spans = vec![
            Span::styled("\"", Style::default().fg(Colors::text())),
            Span::styled(best_rank.name(), Style::default().fg(best_rank.color())),
            Span::styled("\" with ", Style::default().fg(Colors::text())),
            Span::styled(
                format!("{:.0}pts", metrics.session_score),
                Style::default().fg(Colors::score()),
            ),
        ];

        if let Some(repo) = repo_info {
            spans.push(Span::styled(" on [", Style::default().fg(Colors::text())));
            spans.push(Span::styled(
                format!("{}/{}", repo.user_name, repo.repository_name),
                Style::default().fg(Colors::info()),
            ));
            spans.push(Span::styled("]", Style::default().fg(Colors::text())));
        }

        spans.extend(vec![
            Span::styled(" - ", Style::default().fg(Colors::text())),
            Span::styled("CPM: ", Style::default().fg(Colors::cpm_wpm())),
            Span::styled(
                format!("{:.0}", metrics.overall_cpm),
                Style::default().fg(Colors::text()),
            ),
            Span::styled(", ", Style::default().fg(Colors::text())),
            Span::styled("Mistakes: ", Style::default().fg(Colors::error())),
            Span::styled(
                format!("{}", total_mistakes),
                Style::default().fg(Colors::text()),
            ),
        ]);

        let preview_line = Line::from(spans);
        let preview_widget = Paragraph::new(preview_line).alignment(Alignment::Center);
        frame.render_widget(preview_widget, area);
    }
}
