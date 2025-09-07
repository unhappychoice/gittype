use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

/// Content panel widget for consistent bordered areas
pub struct ContentPanelWidget<'a> {
    title: &'a str,
    border_color: Color,
    border_style: Style,
}

impl<'a> ContentPanelWidget<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            border_color: Color::Blue,
            border_style: Style::default(),
        }
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Create the Block that can be used with other widgets
    pub fn block(self) -> Block<'a> {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.border_style.fg(self.border_color))
            .title(self.title)
    }
}

impl<'a> Widget for ContentPanelWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.block().render(area, buf);
    }
}
