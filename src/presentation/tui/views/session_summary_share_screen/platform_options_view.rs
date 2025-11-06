use crate::presentation::sharing::SharingPlatform;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct PlatformOptionsView;

impl PlatformOptionsView {
    pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, colors: &Colors) {
        let platforms = SharingPlatform::all();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); platforms.len()])
            .split(area);

        for (i, platform) in platforms.iter().enumerate() {
            let option_text = format!("[{}] {}", i + 1, platform.name());
            let option_line = Line::from(vec![Span::styled(
                option_text,
                Style::default().fg(colors.text()),
            )]);
            let option_widget = Paragraph::new(option_line).alignment(Alignment::Center);
            frame.render_widget(option_widget, chunks[i]);
        }
    }
}
