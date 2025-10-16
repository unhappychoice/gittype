use crate::presentation::ui::Colors;
use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct TitleView;

impl TitleView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
        let title = Paragraph::new(Line::from(vec![Span::styled(
            "=== SHARE YOUR RESULT ===",
            Style::default()
                .fg(Colors::info())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(title, area);
    }
}
