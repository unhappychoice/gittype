use crate::presentation::ui::Colors;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct CacheInfoView;

impl CacheInfoView {
    pub fn render(frame: &mut Frame, area: Rect, cache_dir: &str, colors: &Colors) {
        let cache_line = Line::from(vec![
            Span::styled(
                "Cache Directory: ",
                Style::default().fg(colors.text_secondary()),
            ),
            Span::styled(cache_dir, Style::default().fg(colors.text())),
        ]);
        let cache_info = Paragraph::new(cache_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border())),
        );
        frame.render_widget(cache_info, area);
    }
}
