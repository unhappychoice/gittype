use crate::presentation::ui::Colors;
use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct BackOptionView;

impl BackOptionView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, colors: &Colors) {
        let back_line = Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(colors.success())),
            Span::styled(" Back to Results", Style::default().fg(colors.text())),
        ]);
        let back_widget = Paragraph::new(back_line).alignment(Alignment::Center);
        frame.render_widget(back_widget, area);
    }
}
