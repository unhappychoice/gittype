use crate::storage::daos::session_dao::SessionStageResult;
use crate::ui::Colors;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct StageDetailsView;

impl StageDetailsView {
    pub fn render(
        f: &mut Frame,
        area: ratatui::prelude::Rect,
        stage_results: &[SessionStageResult],
        stage_scroll_offset: usize,
    ) {
        if stage_results.is_empty() {
            let empty_msg = Paragraph::new("No stage data available")
                .style(Style::default().fg(Colors::MUTED))
                .alignment(ratatui::layout::Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Colors::BORDER))
                        .title("Stage Details"),
                );
            f.render_widget(empty_msg, area);
            return;
        }

        let mut stage_text_lines = Vec::new();

        let visible_height = area.height.saturating_sub(3) as usize;
        let start_idx = stage_scroll_offset;
        let end_idx = (start_idx + visible_height.saturating_sub(2)).min(stage_results.len());

        stage_text_lines.push(Line::from(""));

        for (i, stage) in stage_results[start_idx..end_idx].iter().enumerate() {
            let actual_idx = start_idx + i;

            let status = if stage.was_failed {
                "FAILED"
            } else if stage.was_skipped {
                "SKIPPED"
            } else {
                "COMPLETED"
            };

            let status_color = if stage.was_failed {
                Colors::FAILED
            } else if stage.was_skipped {
                Colors::SKIPPED
            } else {
                Colors::COMPLETED
            };

            stage_text_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!("Stage #{} ", stage.stage_number),
                    Style::default()
                        .fg(Colors::INFO)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("[{}]", status), Style::default().fg(status_color)),
            ]));

            if let (Some(ref file_path), Some(start), Some(end)) =
                (stage.file_path.clone(), stage.start_line, stage.end_line)
            {
                stage_text_lines.push(Line::from(vec![
                    Span::raw("    "),
                    Span::styled("File: ", Style::default().fg(Colors::STAGE_INFO)),
                    Span::raw(format!("{}:{}-{}", file_path, start, end)),
                ]));
            }

            stage_text_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("Score: ", Style::default().fg(Colors::SCORE)),
                Span::styled(
                    format!("{:.1}", stage.score),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("CPM: ", Style::default().fg(Colors::CPM_WPM)),
                Span::styled(
                    format!("{:.1}", stage.cpm),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("    "),
                Span::styled("WPM: ", Style::default().fg(Colors::CPM_WPM)),
                Span::styled(
                    format!("{:.1}", stage.wpm),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            stage_text_lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("Keystrokes: ", Style::default().fg(Colors::STAGE_INFO)),
                Span::styled(
                    format!("{}", stage.keystrokes),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("Mistakes: ", Style::default().fg(Colors::ERROR)),
                Span::styled(
                    format!("{}", stage.mistakes),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("Accuracy: ", Style::default().fg(Colors::ACCURACY)),
                Span::styled(
                    format!("{:.1}%", stage.accuracy),
                    Style::default().fg(Colors::TEXT),
                ),
                Span::raw("  "),
                Span::styled("Duration: ", Style::default().fg(Colors::DURATION)),
                Span::styled(
                    format!("{}ms", stage.duration_ms),
                    Style::default().fg(Colors::TEXT),
                ),
            ]));

            if actual_idx < stage_results.len() - 1 && i < end_idx - start_idx - 1 {
                stage_text_lines.push(Line::raw(""));
            }
        }

        let scroll_info = if stage_results.len() > visible_height.saturating_sub(2) {
            format!(
                " ({}/{} stages shown, ↑↓ to scroll)",
                end_idx - start_idx,
                stage_results.len()
            )
        } else {
            format!(" ({} stages)", stage_results.len())
        };

        let stage_paragraph = Paragraph::new(stage_text_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::BORDER))
                    .title(format!("Stage Details{}", scroll_info))
                    .title_style(
                        Style::default()
                            .fg(Colors::TEXT)
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(stage_paragraph, area);
    }
}
