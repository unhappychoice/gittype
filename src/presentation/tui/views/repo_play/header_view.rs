use crate::presentation::ui::Colors;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct HeaderView;

impl HeaderView {
    pub fn render(frame: &mut Frame, area: Rect, colors: &Colors) {
        let header_lines = vec![Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Select Repository to Play",
                Style::default()
                    .fg(colors.info())
                    .add_modifier(Modifier::BOLD),
            ),
        ])];

        let header = Paragraph::new(header_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border()))
                .title("GitType"),
        );
        frame.render_widget(header, area);
    }
}
