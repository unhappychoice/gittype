use crate::domain::models::SessionResult;
use crate::domain::repositories::session_repository::{BestRecords, BestStatus};
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct BestRecordsView;

impl BestRecordsView {
    pub fn render(
        f: &mut Frame,
        area: Rect,
        session_result: &SessionResult,
        best_status: Option<&BestStatus>,
        best_records: Option<&BestRecords>,
        colors: &Colors,
    ) {
        if let Some(best_records) = best_records {
            let mut lines = vec![
                Line::from(Span::styled(
                    "BEST RECORDS",
                    Style::default()
                        .fg(colors.warning())
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
            ];

            let records = [
                ("Today's Best", &best_records.todays_best),
                ("Weekly Best", &best_records.weekly_best),
                ("All time Best", &best_records.all_time_best),
            ];

            let max_label_width = records
                .iter()
                .map(|(label, _)| label.len())
                .max()
                .unwrap_or(0);

            for (label, record_data) in records.iter() {
                if let Some(record) = record_data {
                    // Use best_status if provided, otherwise fall back to direct comparison
                    let is_new_pb = if let Some(status) = best_status {
                        match *label {
                            "Today's Best" => status.is_todays_best,
                            "Weekly Best" => status.is_weekly_best,
                            "All time Best" => status.is_all_time_best,
                            _ => false,
                        }
                    } else {
                        session_result.session_score > record.score
                    };

                    let diff = session_result.session_score - record.score;

                    let mut spans = vec![];

                    if is_new_pb {
                        spans.push(Span::styled(
                            "*** NEW PB! ",
                            Style::default().fg(colors.warning()),
                        ));
                    }

                    spans.push(Span::styled(
                        format!("{:>width$}: ", label, width = max_label_width),
                        Style::default().fg(colors.text()),
                    ));

                    spans.push(Span::styled("Score ", Style::default().fg(colors.score())));
                    spans.push(Span::styled(
                        format!("{:.0}", record.score),
                        Style::default().fg(colors.text()),
                    ));
                    spans.push(Span::styled(" | ", Style::default().fg(colors.text())));

                    spans.push(Span::styled("CPM ", Style::default().fg(colors.cpm_wpm())));
                    spans.push(Span::styled(
                        format!("{:.0}", record.cpm),
                        Style::default().fg(colors.text()),
                    ));
                    spans.push(Span::styled(" | ", Style::default().fg(colors.text())));

                    spans.push(Span::styled("Acc ", Style::default().fg(colors.accuracy())));
                    spans.push(Span::styled(
                        format!("{:.1}%", record.accuracy),
                        Style::default().fg(colors.text()),
                    ));

                    if diff > 0.0 {
                        spans.push(Span::styled(
                            format!(" (+{:.0})", diff),
                            Style::default().fg(colors.success()),
                        ));
                    } else if diff < 0.0 {
                        spans.push(Span::styled(
                            format!(" ({:.0})", diff),
                            Style::default().fg(colors.error()),
                        ));
                    }

                    lines.push(Line::from(spans));
                } else {
                    let no_record_line = format!(
                        "{:>width$}: No previous record",
                        label,
                        width = max_label_width
                    );
                    lines.push(Line::from(Span::styled(
                        no_record_line,
                        Style::default().fg(colors.text_secondary()),
                    )));
                }
            }

            let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
            f.render_widget(paragraph, area);
        }
    }
}
