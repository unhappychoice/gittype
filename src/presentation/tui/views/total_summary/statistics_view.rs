use crate::domain::models::TotalResult;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct StatisticsView;

impl StatisticsView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, total_summary: &TotalResult) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Line 1: CPM, WPM, Accuracy
                Constraint::Length(1), // Line 2: Sessions and Stages
                Constraint::Length(1), // Line 3: Keystrokes, Mistakes, Skipped
                Constraint::Length(1), // Line 4: Best/Worst sessions
            ])
            .split(area);

        // Line 1: Overall CPM, WPM, Accuracy
        let line1 = Line::from(vec![
            Span::styled("Overall ", Style::default().fg(Colors::text())),
            Span::styled("CPM: ", Style::default().fg(Colors::cpm_wpm())),
            Span::styled(
                format!("{:.1}", total_summary.overall_cpm),
                Style::default().fg(Colors::text()),
            ),
            Span::styled(" | ", Style::default().fg(Colors::text())),
            Span::styled("WPM: ", Style::default().fg(Colors::cpm_wpm())),
            Span::styled(
                format!("{:.1}", total_summary.overall_wpm),
                Style::default().fg(Colors::text()),
            ),
            Span::styled(" | ", Style::default().fg(Colors::text())),
            Span::styled("Accuracy: ", Style::default().fg(Colors::accuracy())),
            Span::styled(
                format!("{:.1}%", total_summary.overall_accuracy),
                Style::default().fg(Colors::text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line1).alignment(Alignment::Center),
            chunks[0],
        );

        // Line 2: Sessions and Stages
        let line2 = Line::from(vec![
            Span::styled("Total ", Style::default().fg(Colors::text())),
            Span::styled("Sessions: ", Style::default().fg(Colors::stage_info())),
            Span::styled(
                format!("{}", total_summary.total_sessions_attempted),
                Style::default().fg(Colors::text()),
            ),
            Span::styled(" | ", Style::default().fg(Colors::text())),
            Span::styled("Completed: ", Style::default().fg(Colors::success())),
            Span::styled(
                format!("{}", total_summary.total_sessions_completed),
                Style::default().fg(Colors::text()),
            ),
            Span::styled(" | ", Style::default().fg(Colors::text())),
            Span::styled("Stages: ", Style::default().fg(Colors::stage_info())),
            Span::styled(
                format!(
                    "{}/{}",
                    total_summary.total_stages_completed, total_summary.total_stages_attempted
                ),
                Style::default().fg(Colors::text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line2).alignment(Alignment::Center),
            chunks[1],
        );

        // Line 3: Keystrokes, Mistakes, Skipped
        let line3 = Line::from(vec![
            Span::styled("Total ", Style::default().fg(Colors::text())),
            Span::styled("Keystrokes: ", Style::default().fg(Colors::stage_info())),
            Span::styled(
                format!("{}", total_summary.total_keystrokes),
                Style::default().fg(Colors::text()),
            ),
            Span::styled(" | ", Style::default().fg(Colors::text())),
            Span::styled("Mistakes: ", Style::default().fg(Colors::error())),
            Span::styled(
                format!("{}", total_summary.total_mistakes),
                Style::default().fg(Colors::text()),
            ),
            Span::styled(" | ", Style::default().fg(Colors::text())),
            Span::styled("Skipped: ", Style::default().fg(Colors::warning())),
            Span::styled(
                format!("{}", total_summary.total_stages_skipped),
                Style::default().fg(Colors::text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line3).alignment(Alignment::Center),
            chunks[2],
        );

        // Line 4: Best/Worst sessions
        let line4 = Line::from(vec![
            Span::styled("Best Session: ", Style::default().fg(Colors::text())),
            Span::styled(
                format!("{:.0} CPM", total_summary.best_session_wpm * 5.0),
                Style::default().fg(Colors::cpm_wpm()),
            ),
            Span::styled(", ", Style::default().fg(Colors::text())),
            Span::styled(
                format!("{:.1}%", total_summary.best_session_accuracy),
                Style::default().fg(Colors::accuracy()),
            ),
            Span::styled(" | Worst: ", Style::default().fg(Colors::text())),
            Span::styled(
                format!("{:.0} CPM", total_summary.worst_session_wpm * 5.0),
                Style::default().fg(Colors::cpm_wpm()),
            ),
            Span::styled(", ", Style::default().fg(Colors::text())),
            Span::styled(
                format!("{:.1}%", total_summary.worst_session_accuracy),
                Style::default().fg(Colors::accuracy()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line4).alignment(Alignment::Center),
            chunks[3],
        );
    }
}
