use crate::presentation::ui::Colors;
use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct FooterView;

impl FooterView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
        let nav_line = Line::from(vec![
            Span::styled("[R]", Style::default().fg(Colors::success())),
            Span::styled(" Retry | ", Style::default().fg(Colors::text())),
            Span::styled("[T]", Style::default().fg(Colors::success())),
            Span::styled(" Back to Title | ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(
                " Session Summary & Exit",
                Style::default().fg(Colors::text()),
            ),
        ]);
        let navigation = Paragraph::new(nav_line).alignment(Alignment::Center);
        frame.render_widget(navigation, area);
    }
}
