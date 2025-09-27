use crate::domain::models::GitRepository;
use crate::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct StageResultsView;

impl StageResultsView {
    pub fn render(
        f: &mut Frame,
        area: Rect,
        session_result: &crate::domain::models::SessionResult,
        repo_info: &Option<GitRepository>,
    ) {
        if !session_result.stage_results.is_empty() {
            let mut lines = vec![];

            let stage_label = if let Some(repo) = repo_info {
                format!(
                    "Stage Results: [{}/{}]",
                    repo.user_name, repo.repository_name
                )
            } else {
                "Stage Results:".to_string()
            };

            lines.push(Line::from(Span::styled(
                stage_label,
                Style::default()
                    .fg(Colors::info())
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            for (i, stage_result) in session_result.stage_results.iter().enumerate() {
                let stage_name = Self::get_stage_name(stage_result, i);

                // Stage name line
                lines.push(Line::from(Span::styled(
                    format!("{}:", stage_name),
                    Style::default()
                        .fg(Colors::text())
                        .add_modifier(Modifier::BOLD),
                )));

                // Metrics line (indented)
                let mut metrics_spans = vec![];
                metrics_spans.push(Span::styled("  ", Style::default()));

                metrics_spans.push(Span::styled(
                    "Score: ",
                    Style::default().fg(Colors::score()),
                ));
                metrics_spans.push(Span::styled(
                    format!("{:.0}", stage_result.challenge_score),
                    Style::default().fg(Colors::text()),
                ));
                metrics_spans.push(Span::styled(" | ", Style::default().fg(Colors::text())));

                metrics_spans.push(Span::styled(
                    "CPM: ",
                    Style::default().fg(Colors::cpm_wpm()),
                ));
                metrics_spans.push(Span::styled(
                    format!("{:.0}", stage_result.cpm),
                    Style::default().fg(Colors::text()),
                ));
                metrics_spans.push(Span::styled(" | ", Style::default().fg(Colors::text())));

                metrics_spans.push(Span::styled(
                    "Acc: ",
                    Style::default().fg(Colors::accuracy()),
                ));
                metrics_spans.push(Span::styled(
                    format!("{:.1}%", stage_result.accuracy),
                    Style::default().fg(Colors::text()),
                ));

                lines.push(Line::from(metrics_spans));
            }

            let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
            f.render_widget(paragraph, area);
        }
    }

    fn get_stage_name(stage_result: &crate::domain::models::StageResult, index: usize) -> String {
        if !stage_result.challenge_path.is_empty() {
            stage_result.challenge_path.clone()
        } else {
            format!("Stage {}", index + 1)
        }
    }
}
