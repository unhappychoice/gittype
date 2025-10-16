use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct LegendView;

impl LegendView {
    pub fn render(frame: &mut Frame, area: Rect) {
        let legend_line = Line::from(vec![
            Span::styled("●", Style::default().fg(Colors::success())),
            Span::styled(" Cached  ", Style::default().fg(Colors::text())),
            Span::styled("○", Style::default().fg(Colors::text_secondary())),
            Span::styled(" Not Cached", Style::default().fg(Colors::text())),
        ]);
        let legend = Paragraph::new(legend_line)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Colors::border())),
            );
        frame.render_widget(legend, area);
    }
}
