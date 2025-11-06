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
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, colors: &Colors) {
        let nav_line = Line::from(vec![
            Span::styled("[R]", Style::default().fg(colors.success())),
            Span::styled(" Retry | ", Style::default().fg(colors.text())),
            Span::styled("[T]", Style::default().fg(colors.success())),
            Span::styled(" Back to Title | ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(
                " Session Summary & Exit",
                Style::default().fg(colors.text()),
            ),
        ]);
        let navigation = Paragraph::new(nav_line).alignment(Alignment::Center);
        frame.render_widget(navigation, area);
    }
}
