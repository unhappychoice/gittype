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
    pub fn render(f: &mut Frame, area: Rect, colors: &Colors) {
        let controls_line = Line::from(vec![
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(" Return", Style::default().fg(colors.text())),
        ]);

        let controls = Paragraph::new(controls_line).alignment(Alignment::Center);
        f.render_widget(controls, area);
    }
}
