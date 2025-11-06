use crate::domain::models::SessionResult;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct SummaryView;

impl SummaryView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        session_result: &SessionResult,
        colors: &Colors,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Line 1: CPM | WPM | Time
                Constraint::Length(1), // Line 2: Keystrokes | Mistakes | Accuracy
            ])
            .split(area);

        // Line 1: CPM | WPM | Time
        let line1 = Line::from(vec![
            Span::styled("CPM: ", Style::default().fg(colors.cpm_wpm())),
            Span::styled(
                format!("{:.0}", session_result.overall_cpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("WPM: ", Style::default().fg(colors.cpm_wpm())),
            Span::styled(
                format!("{:.0}", session_result.overall_wpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("Time: ", Style::default().fg(colors.duration())),
            Span::styled(
                format!("{:.1}s", session_result.session_duration.as_secs_f64()),
                Style::default().fg(colors.text()),
            ),
        ]);
        let line1_widget = Paragraph::new(line1).alignment(Alignment::Center);
        frame.render_widget(line1_widget, chunks[0]);

        // Line 2: Keystrokes | Mistakes | Accuracy
        let total_keystrokes = session_result.valid_keystrokes + session_result.invalid_keystrokes;
        let total_mistakes = session_result.valid_mistakes + session_result.invalid_mistakes;

        let line2 = Line::from(vec![
            Span::styled("Keystrokes: ", Style::default().fg(colors.stage_info())),
            Span::styled(
                format!("{}", total_keystrokes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("Mistakes: ", Style::default().fg(colors.error())),
            Span::styled(
                format!("{}", total_mistakes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled("Accuracy: ", Style::default().fg(colors.accuracy())),
            Span::styled(
                format!("{:.1}%", session_result.overall_accuracy),
                Style::default().fg(colors.text()),
            ),
        ]);
        let line2_widget = Paragraph::new(line2).alignment(Alignment::Center);
        frame.render_widget(line2_widget, chunks[1]);
    }
}
