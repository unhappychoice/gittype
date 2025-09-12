use crate::{game::typing_core::TypingCore, scoring::tracker::stage::StageTracker, ui::Colors};
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub struct TypingFooterView;

impl TypingFooterView {
    pub fn render_metrics(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        waiting_to_start: bool,
        countdown_active: bool,
        skips_remaining: usize,
        stage_tracker: &StageTracker,
        typing_core: &TypingCore,
    ) {
        let metrics_line = if waiting_to_start || countdown_active {
            // Show zeros during waiting and countdown
            format!(
                "WPM: 0 | CPM: 0 | Accuracy: 0% | Mistakes: 0 | Streak: 0 | Time: 0s | Skips: {}",
                skips_remaining
            )
        } else {
            let elapsed_time = stage_tracker.get_data().elapsed_time;

            // Use typing_core position (correctly typed characters) and mistakes for RealtimeCalculator
            let current_position = typing_core.current_position_to_type();
            let mistakes = typing_core.mistakes();

            let metrics = crate::scoring::RealTimeCalculator::calculate(
                current_position,
                mistakes,
                elapsed_time,
            );
            let elapsed_secs = elapsed_time.as_secs();

            let streak = stage_tracker.get_data().current_streak;
            format!(
                "WPM: {:.0} | CPM: {:.0} | Accuracy: {:.0}% | Mistakes: {} | Streak: {} | Time: {}s | Skips: {}",
                metrics.wpm, metrics.cpm, metrics.accuracy, metrics.mistakes, streak, elapsed_secs, skips_remaining
            )
        };

        let metrics_widget = Paragraph::new(vec![Line::from(vec![Span::styled(
            metrics_line,
            Style::default().fg(Colors::MUTED),
        )])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::SECONDARY))
                .title("Metrics")
                .title_style(Style::default().fg(Colors::SECONDARY))
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
        frame.render_widget(metrics_widget, area);
    }

    pub fn render_progress(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        waiting_to_start: bool,
        countdown_active: bool,
        typing_core: &TypingCore,
        chars_len: usize,
    ) {
        let progress_percent = if waiting_to_start || countdown_active {
            0 // Show 0% during waiting and countdown
        } else if chars_len > 0 {
            (typing_core.current_position_to_display() as f32 / chars_len as f32 * 100.0) as u8
        } else {
            0
        };

        let progress_widget = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::SECONDARY))
                    .title("Progress")
                    .title_style(Style::default().fg(Colors::SECONDARY)),
            )
            .gauge_style(Style::default().fg(Colors::PROGRESS_BAR))
            .percent(progress_percent as u16)
            .label(format!("{}%", progress_percent));
        frame.render_widget(progress_widget, area);
    }
}
