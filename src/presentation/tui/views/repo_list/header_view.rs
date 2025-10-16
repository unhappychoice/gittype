use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct HeaderView;

impl HeaderView {
    pub fn render(frame: &mut Frame, area: Rect) {
        let header_line = Line::from(vec![Span::styled(
            "GitType - Played Repositories",
            Style::default()
                .fg(Colors::info())
                .add_modifier(Modifier::BOLD),
        )]);
        let header = Paragraph::new(header_line)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border())),
            );
        frame.render_widget(header, area);
    }
}
