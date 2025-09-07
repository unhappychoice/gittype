use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Control bar widget for displaying keyboard shortcuts and controls
pub struct ControlBarWidget<'a> {
    controls: Vec<(&'a str, &'a str, Color)>, // (key, description, key_color)
    alignment: Alignment,
    text_color: Color,
}

impl<'a> Default for ControlBarWidget<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ControlBarWidget<'a> {
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            alignment: Alignment::Center,
            text_color: Color::White,
        }
    }

    pub fn control(mut self, key: &'a str, description: &'a str, key_color: Color) -> Self {
        self.controls.push((key, description, key_color));
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Convenience methods for common controls
    pub fn navigation() -> Self {
        Self::new().control("[↑↓/JK]", " Navigate  ", Color::Blue)
    }

    pub fn back() -> Self {
        Self::new().control("[ESC]", " Back", Color::Red)
    }

    pub fn refresh() -> Self {
        Self::new().control("[R]", " Refresh  ", Color::Magenta)
    }

    pub fn space_action(action: &'a str) -> ControlBarWidget<'a> {
        // We need to avoid using format! in a builder pattern due to lifetime issues
        // Instead, callers should format the string themselves
        Self::new().control("[SPACE]", action, Color::Green)
    }
}

impl<'a> Widget for ControlBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = Vec::new();

        for (i, (key, description, key_color)) in self.controls.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  ")); // Space between controls
            }
            spans.push(Span::styled(*key, Style::default().fg(*key_color)));
            spans.push(Span::styled(
                *description,
                Style::default().fg(self.text_color),
            ));
        }

        let controls_line = Line::from(spans);
        let paragraph = Paragraph::new(controls_line).alignment(self.alignment);

        paragraph.render(area, buf);
    }
}
