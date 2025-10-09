use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct HeaderView;

impl HeaderView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Session title
                Constraint::Length(2), // Spacing
                Constraint::Length(1), // YOU'RE label
            ])
            .split(area);

        let session_title = Paragraph::new(Line::from(vec![Span::styled(
            "=== SESSION COMPLETE ===",
            Style::default()
                .fg(Colors::info())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(session_title, chunks[0]);

        let youre_label = Paragraph::new(Line::from(vec![Span::styled(
            "YOU'RE:",
            Style::default()
                .fg(Colors::info())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(youre_label, chunks[2]);
    }
}
