use crate::domain::models::storage::SessionResultData;
use crate::presentation::ui::Colors;
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct PerformanceMetricsView;

impl PerformanceMetricsView {
    pub fn render(
        f: &mut Frame,
        area: ratatui::prelude::Rect,
        session_result: Option<&SessionResultData>,
        colors: &Colors,
    ) {
        let mut metrics_lines = Vec::new();

        metrics_lines.push(Line::from(""));

        if let Some(result) = session_result {
            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Tier/Rank: ", Style::default().fg(colors.stage_info())),
                Span::styled(
                    format!(
                        "{}/{}",
                        result.tier_name.clone().unwrap_or("unknown".to_string()),
                        result.rank_name.clone().unwrap_or("unknown".to_string())
                    ),
                    Style::default().fg(colors.text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Score: ", Style::default().fg(colors.score())),
                Span::styled(
                    format!("{:.1}", result.score),
                    Style::default().fg(colors.text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("CPM: ", Style::default().fg(colors.cpm_wpm())),
                Span::styled(
                    format!("{:.1}", result.cpm),
                    Style::default().fg(colors.text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("WPM: ", Style::default().fg(colors.cpm_wpm())),
                Span::styled(
                    format!("{:.1}", result.wpm),
                    Style::default().fg(colors.text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Accuracy: ", Style::default().fg(colors.accuracy())),
                Span::styled(
                    format!("{:.1}%", result.accuracy),
                    Style::default().fg(colors.text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Duration: ", Style::default().fg(colors.duration())),
                Span::styled(
                    format!(
                        "{}m {}s",
                        result.duration_ms / 60000,
                        (result.duration_ms % 60000) / 1000
                    ),
                    Style::default().fg(colors.text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    "Completed Stage: ",
                    Style::default().fg(colors.stage_info()),
                ),
                Span::styled(
                    result.stages_completed.to_string(),
                    Style::default().fg(colors.text()),
                ),
                Span::raw("/"),
                Span::styled(
                    result.stages_attempted.to_string(),
                    Style::default().fg(colors.text()),
                ),
            ]));

            if result.stages_skipped > 0 {
                metrics_lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Skipped: ", Style::default().fg(colors.error())),
                    Span::styled(
                        result.stages_skipped.to_string(),
                        Style::default().fg(colors.text()),
                    ),
                ]));
            }
        } else {
            metrics_lines.push(Line::from("No performance data available"));
        }

        let performance = Paragraph::new(metrics_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .title("Performance"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(performance, area);
    }
}
