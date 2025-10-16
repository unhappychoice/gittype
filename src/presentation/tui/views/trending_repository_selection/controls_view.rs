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
    pub fn render(frame: &mut Frame, area: Rect) {
        let controls_line = Line::from(vec![
            Span::styled("[↑↓/JK]", Style::default().fg(Colors::key_navigation())),
            Span::styled(" Navigate  ", Style::default().fg(Colors::text())),
            Span::styled("[SPACE]", Style::default().fg(Colors::key_action())),
            Span::styled(" Play  ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::key_back())),
            Span::styled(" Return", Style::default().fg(Colors::text())),
        ]);
        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        frame.render_widget(controls, area);
    }
}
