use crate::presentation::ui::Colors;
use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct HeaderView;

impl HeaderView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
        let header = Paragraph::new(Line::from(vec![Span::styled(
            "=== SESSION FAILED ===",
            Style::default()
                .fg(Colors::error())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(header, area);
    }
}
