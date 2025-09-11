use crate::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub struct HeaderView;

impl HeaderView {
    pub fn render(f: &mut Frame, area: Rect) {
        let title = Paragraph::new("=== SESSION DETAILS ===")
            .style(
                Style::default()
                    .fg(Colors::INFO)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        f.render_widget(title, area);
    }
}
