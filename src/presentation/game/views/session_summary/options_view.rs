use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct OptionsView;

impl OptionsView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Row 1
                Constraint::Length(1), // Row 2
            ])
            .split(area);

        // Row 1: [D] Show Detail  [S] Share Result
        let row1 = Line::from(vec![
            Span::styled("[D]", Style::default().fg(Colors::info())),
            Span::styled(" Show Detail", Style::default().fg(Colors::text())),
            Span::styled("  ", Style::default().fg(Colors::text())),
            Span::styled("[S]", Style::default().fg(Colors::info())),
            Span::styled(" Share Result", Style::default().fg(Colors::text())),
        ]);
        let row1_widget = Paragraph::new(row1).alignment(Alignment::Center);
        frame.render_widget(row1_widget, chunks[0]);

        // Row 2: [R] Retry  [T] Back to Title  [ESC] Quit
        let row2 = Line::from(vec![
            Span::styled("[R]", Style::default().fg(Colors::success())),
            Span::styled(" Retry", Style::default().fg(Colors::text())),
            Span::styled("  ", Style::default().fg(Colors::text())),
            Span::styled("[T]", Style::default().fg(Colors::success())),
            Span::styled(" Back to Title", Style::default().fg(Colors::text())),
            Span::styled("  ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Quit", Style::default().fg(Colors::text())),
        ]);
        let row2_widget = Paragraph::new(row2).alignment(Alignment::Center);
        frame.render_widget(row2_widget, chunks[1]);
    }
}
