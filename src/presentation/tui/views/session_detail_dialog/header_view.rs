use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub struct HeaderView;

impl HeaderView {
    pub fn render(f: &mut Frame, area: Rect, colors: &Colors) {
        let title = Paragraph::new("=== SESSION DETAILS ===")
            .style(
                Style::default()
                    .fg(colors.info())
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        f.render_widget(title, area);
    }
}
