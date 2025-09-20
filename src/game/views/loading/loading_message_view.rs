use crate::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct LoadingMessageView;

impl LoadingMessageView {
    pub fn render(frame: &mut Frame, area: Rect) {
        let loading_msg = Line::from(vec![
            Span::styled("» ", Style::default().fg(Colors::warning())),
            Span::styled(
                "Loading...",
                Style::default()
                    .fg(Colors::warning())
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let loading = Paragraph::new(vec![loading_msg]).alignment(Alignment::Center);

        frame.render_widget(loading, area);
    }
}
