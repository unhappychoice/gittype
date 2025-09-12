use crate::models::GitRepository;
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
        session_result: &crate::models::SessionResult,
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
                    .fg(Colors::INFO)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            let max_stage_name_width = Self::calculate_max_stage_name_width(session_result);

            for (i, stage_result) in session_result.stage_results.iter().enumerate() {
                let stage_name = Self::get_stage_name(stage_result, i);

                let mut spans = vec![];

                spans.push(Span::styled(
                    format!("{:>width$}: ", stage_name, width = max_stage_name_width),
                    Style::default().fg(Colors::TEXT),
                ));

                spans.push(Span::styled("Score ", Style::default().fg(Colors::SCORE)));
                spans.push(Span::styled(
                    format!("{:.0}", stage_result.challenge_score),
                    Style::default().fg(Colors::TEXT),
                ));
                spans.push(Span::styled(" | ", Style::default().fg(Colors::TEXT)));

                spans.push(Span::styled("CPM ", Style::default().fg(Colors::CPM_WPM)));
                spans.push(Span::styled(
                    format!("{:.0}", stage_result.cpm),
                    Style::default().fg(Colors::TEXT),
                ));
                spans.push(Span::styled(" | ", Style::default().fg(Colors::TEXT)));

                spans.push(Span::styled("Acc ", Style::default().fg(Colors::ACCURACY)));
                spans.push(Span::styled(
                    format!("{:.1}%", stage_result.accuracy),
                    Style::default().fg(Colors::TEXT),
                ));

                lines.push(Line::from(spans));
            }

            let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
            f.render_widget(paragraph, area);
        }
    }

    fn calculate_max_stage_name_width(session_result: &crate::models::SessionResult) -> usize {
        session_result
            .stage_results
            .iter()
            .enumerate()
            .map(|(i, stage)| {
                if !stage.challenge_path.is_empty() {
                    stage.challenge_path.len()
                } else {
                    format!("Stage {}", i + 1).len()
                }
            })
            .max()
            .unwrap_or(20)
    }

    fn get_stage_name(stage_result: &crate::models::StageResult, index: usize) -> String {
        if !stage_result.challenge_path.is_empty() {
            stage_result.challenge_path.clone()
        } else {
            format!("Stage {}", index + 1)
        }
    }
}
