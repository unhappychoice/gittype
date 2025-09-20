use crate::storage::daos::session_dao::SessionResultData;
use crate::ui::Colors;
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
    ) {
        let mut metrics_lines = Vec::new();

        metrics_lines.push(Line::from(""));

        if let Some(result) = session_result {
            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Tier/Rank: ", Style::default().fg(Colors::stage_info())),
                Span::styled(
                    format!(
                        "{}/{}",
                        result.tier_name.clone().unwrap_or("unknown".to_string()),
                        result.rank_name.clone().unwrap_or("unknown".to_string())
                    ),
                    Style::default().fg(Colors::text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Score: ", Style::default().fg(Colors::score())),
                Span::styled(
                    format!("{:.1}", result.score),
                    Style::default().fg(Colors::text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("CPM: ", Style::default().fg(Colors::cpm_wpm())),
                Span::styled(
                    format!("{:.1}", result.cpm),
                    Style::default().fg(Colors::text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("WPM: ", Style::default().fg(Colors::cpm_wpm())),
                Span::styled(
                    format!("{:.1}", result.wpm),
                    Style::default().fg(Colors::text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Accuracy: ", Style::default().fg(Colors::accuracy())),
                Span::styled(
                    format!("{:.1}%", result.accuracy),
                    Style::default().fg(Colors::text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Duration: ", Style::default().fg(Colors::duration())),
                Span::styled(
                    format!(
                        "{}m {}s",
                        result.duration_ms / 60000,
                        (result.duration_ms % 60000) / 1000
                    ),
                    Style::default().fg(Colors::text()),
                ),
            ]));

            metrics_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Completed Stage: ", Style::default().fg(Colors::stage_info())),
                Span::styled(
                    result.stages_completed.to_string(),
                    Style::default().fg(Colors::text()),
                ),
                Span::raw("/"),
                Span::styled(
                    result.stages_attempted.to_string(),
                    Style::default().fg(Colors::text()),
                ),
            ]));

            if result.stages_skipped > 0 {
                metrics_lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Skipped: ", Style::default().fg(Colors::error())),
                    Span::styled(
                        result.stages_skipped.to_string(),
                        Style::default().fg(Colors::text()),
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
                    .border_style(Style::default().fg(Colors::border()))
                    .title("Performance"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(performance, area);
    }
}
