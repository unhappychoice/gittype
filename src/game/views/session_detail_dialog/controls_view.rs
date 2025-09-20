use crate::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct ControlsView;

impl ControlsView {
    pub fn render(f: &mut Frame, area: Rect) {
        let controls_line = Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Return", Style::default().fg(Colors::text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, area);
    }
}
