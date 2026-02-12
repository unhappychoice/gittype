use crate::domain::models::ui::{ascii_digits::get_digit_patterns, rank_colors};
use crate::domain::models::Rank;
use crate::domain::services::scoring::StageResult;
use crate::presentation::ui::{Colors, GradationText};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct StageCompletionView;

impl StageCompletionView {
    pub fn render(
        frame: &mut Frame,
        metrics: &StageResult,
        current_stage: usize,
        total_stages: usize,
        has_next_stage: bool,
        keystrokes: usize,
        colors: &Colors,
    ) {
        let area = frame.area();

        // Calculate total content height
        let title_height = 1;
        let title_spacing = 3;
        let score_label_height = 1;
        let ascii_score_height = 4;
        let ascii_spacing = if !metrics.was_failed && !metrics.was_skipped {
            2 // 2 lines before metrics
        } else {
            1 // 1 line before progress
        };
        let metrics_height = if !metrics.was_failed && !metrics.was_skipped {
            2
        } else {
            0
        };
        let metrics_spacing = if !metrics.was_failed && !metrics.was_skipped {
            1
        } else {
            0
        };
        let progress_height = if has_next_stage { 3 } else { 1 };
        let progress_spacing = 1;
        let options_height = 1;

        let total_content_height = title_height
            + title_spacing
            + score_label_height
            + ascii_score_height
            + ascii_spacing
            + metrics_height
            + metrics_spacing
            + progress_height
            + progress_spacing
            + options_height;

        let top_padding = (area.height.saturating_sub(total_content_height as u16)) / 2;

        let mut constraints = vec![
            Constraint::Length(top_padding),
            Constraint::Length(title_height as u16),
            Constraint::Length(title_spacing as u16),
            Constraint::Length(score_label_height as u16),
            Constraint::Length(ascii_score_height as u16),
            Constraint::Length(ascii_spacing as u16),
        ];

        if !metrics.was_failed && !metrics.was_skipped {
            constraints.push(Constraint::Length(metrics_height as u16));
            constraints.push(Constraint::Length(metrics_spacing as u16));
        }

        constraints.push(Constraint::Length(progress_height as u16));
        constraints.push(Constraint::Length(progress_spacing as u16));
        constraints.push(Constraint::Length(options_height as u16));
        constraints.push(Constraint::Min(0)); // bottom

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        let mut chunk_idx = 1;

        // Render stage title
        Self::render_stage_title(colors, frame, chunks[chunk_idx], metrics, current_stage);
        chunk_idx += 2; // title + spacing

        // Render score section
        Self::render_score_label(colors, frame, chunks[chunk_idx], metrics);
        chunk_idx += 1;
        Self::render_score_ascii(colors, frame, chunks[chunk_idx], metrics);
        chunk_idx += 2; // ascii + spacing

        // Display metrics only for completed challenges
        if !metrics.was_failed && !metrics.was_skipped {
            Self::render_metrics(colors, frame, chunks[chunk_idx], metrics, keystrokes);
            chunk_idx += 2; // metrics + spacing
        }

        // Render progress indicator
        Self::render_progress_indicator(
            colors,
            frame,
            chunks[chunk_idx],
            current_stage,
            total_stages,
            has_next_stage,
        );
        chunk_idx += 2; // progress + spacing

        // Render options
        Self::render_options(colors, frame, chunks[chunk_idx]);
    }

    fn create_ascii_numbers(score: &str) -> Vec<String> {
        let digit_patterns = get_digit_patterns();
        let max_height = 4;
        let mut result = vec![String::new(); max_height];

        for ch in score.chars() {
            if let Some(digit) = ch.to_digit(10) {
                let pattern = &digit_patterns[digit as usize];
                for (i, line) in pattern.iter().enumerate() {
                    result[i].push_str(line);
                    result[i].push(' ');
                }
            }
        }

        result
    }

    fn render_stage_title(
        colors: &Colors,
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        metrics: &StageResult,
        current_stage: usize,
    ) {
        let stage_title = if metrics.was_failed {
            format!("=== STAGE {} FAILED ===", current_stage)
        } else if metrics.was_skipped {
            format!("=== STAGE {} SKIPPED ===", current_stage)
        } else {
            format!("=== STAGE {} COMPLETE ===", current_stage)
        };

        let color = if metrics.was_failed {
            colors.error()
        } else if metrics.was_skipped {
            colors.warning()
        } else {
            colors.success()
        };

        let title = Paragraph::new(Line::from(vec![Span::styled(
            stage_title,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);

        frame.render_widget(title, area);
    }

    fn render_score_label(
        colors: &Colors,
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        metrics: &StageResult,
    ) {
        let score_label = if metrics.was_failed {
            "FAILED AFTER"
        } else if metrics.was_skipped {
            "SKIPPED"
        } else {
            "SCORE"
        };

        let color = if metrics.was_failed || metrics.was_skipped {
            colors.error()
        } else {
            colors.success()
        };

        let label = Paragraph::new(Line::from(vec![Span::styled(
            score_label,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);

        frame.render_widget(label, area);
    }

    fn render_score_ascii(
        colors: &Colors,
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        metrics: &StageResult,
    ) {
        let score_value = if metrics.was_failed {
            format!("{:.1}", metrics.completion_time.as_secs_f64())
        } else if metrics.was_skipped {
            "---".to_string()
        } else {
            format!("{:.0}", metrics.challenge_score)
        };

        let ascii_numbers = Self::create_ascii_numbers(&score_value);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); 4])
            .split(area);

        if metrics.was_failed || metrics.was_skipped {
            // For failed/skipped, use solid color
            let color = if metrics.was_failed {
                colors.error()
            } else {
                colors.warning()
            };

            for (i, line) in ascii_numbers.iter().enumerate() {
                let para = Paragraph::new(Line::from(vec![Span::styled(
                    line.as_str(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )]))
                .alignment(Alignment::Center);
                frame.render_widget(para, chunks[i]);
            }
        } else {
            // For completed challenges, use gradation
            let rank = Rank::for_score(metrics.challenge_score);
            let tier_colors = rank_colors::get_tier_colors(&rank.tier);

            for (i, line) in ascii_numbers.iter().enumerate() {
                let widget =
                    GradationText::new(line.as_str(), tier_colors).alignment(Alignment::Center);
                frame.render_widget(widget, chunks[i]);
            }
        }
    }

    fn render_metrics(
        colors: &Colors,
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        metrics: &StageResult,
        keystrokes: usize,
    ) {
        let time_secs = metrics.completion_time.as_secs_f64();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(area);

        // Line 1: CPM, WPM, Time
        let line1 = Line::from(vec![
            Span::styled("CPM: ", Style::default().fg(colors.cpm_wpm())),
            Span::styled(
                format!("{:.0}", metrics.cpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("WPM: ", Style::default().fg(colors.cpm_wpm())),
            Span::styled(
                format!("{:.0}", metrics.wpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("Time: ", Style::default().fg(colors.duration())),
            Span::styled(
                format!("{:.1}s", time_secs),
                Style::default().fg(colors.text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line1).alignment(Alignment::Center),
            chunks[0],
        );

        // Line 2: Keystrokes, Mistakes, Accuracy
        let line2 = Line::from(vec![
            Span::styled("Keystrokes: ", Style::default().fg(colors.stage_info())),
            Span::styled(
                format!("{}", keystrokes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("Mistakes: ", Style::default().fg(colors.error())),
            Span::styled(
                format!("{}", metrics.mistakes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("Accuracy: ", Style::default().fg(colors.accuracy())),
            Span::styled(
                format!("{:.1}%", metrics.accuracy),
                Style::default().fg(colors.text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line2).alignment(Alignment::Center),
            chunks[1],
        );
    }

    fn render_progress_indicator(
        colors: &Colors,
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        current_stage: usize,
        total_stages: usize,
        has_next_stage: bool,
    ) {
        if has_next_stage {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // progress
                    Constraint::Length(1), // spacing
                    Constraint::Length(1), // next message
                ])
                .split(area);

            let progress_text = format!("Stage {} of {}", current_stage, total_stages);
            let progress = Paragraph::new(Line::from(vec![Span::styled(
                progress_text,
                Style::default().fg(colors.text()),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(progress, chunks[0]);

            let next_text = "Next stage starting...";
            let next = Paragraph::new(Line::from(vec![Span::styled(
                next_text,
                Style::default().fg(colors.warning()),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(next, chunks[2]);
        } else {
            let progress_text = format!("Stage {} of {}", current_stage, total_stages);
            let progress = Paragraph::new(Line::from(vec![Span::styled(
                progress_text,
                Style::default().fg(colors.text()),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(progress, area);
        }
    }

    fn render_options(colors: &Colors, frame: &mut Frame, area: ratatui::layout::Rect) {
        let options = Line::from(vec![
            Span::styled("[SPACE]", Style::default().fg(colors.success())),
            Span::styled(" Continue  ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Quit", Style::default().fg(colors.text())),
        ]);

        let options_widget = Paragraph::new(options).alignment(Alignment::Center);
        frame.render_widget(options_widget, area);
    }
}
