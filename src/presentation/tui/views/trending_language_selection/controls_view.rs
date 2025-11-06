use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct ControlsView;

impl ControlsView {
    pub fn render(frame: &mut Frame, area: Rect, colors: &Colors) {
        let controls_line = Line::from(vec![
            Span::styled("[↑↓/JK]", Style::default().fg(colors.key_navigation())),
            Span::styled(" Navigate  ", Style::default().fg(colors.text())),
            Span::styled("[SPACE]", Style::default().fg(colors.key_action())),
            Span::styled(" Select  ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.key_back())),
            Span::styled(" Return", Style::default().fg(colors.text())),
        ]);
        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        frame.render_widget(controls, area);
    }
}
