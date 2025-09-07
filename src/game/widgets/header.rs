use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Header widget for screen titles with consistent styling
pub struct HeaderWidget<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    border_color: Color,
    title_color: Color,
    subtitle_color: Color,
    alignment: Alignment,
}

impl<'a> HeaderWidget<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            border_color: Color::Blue,
            title_color: Color::Cyan,
            subtitle_color: Color::Yellow,
            alignment: Alignment::Left,
        }
    }

    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    pub fn subtitle_color(mut self, color: Color) -> Self {
        self.subtitle_color = color;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<'a> Widget for HeaderWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![Line::from(vec![
            Span::raw("  "), // Left padding
            Span::styled(
                self.title,
                Style::default()
                    .fg(self.title_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ])];

        if let Some(subtitle) = self.subtitle {
            lines.push(Line::from(vec![
                Span::raw("  "), // Left padding
                Span::styled(subtitle, Style::default().fg(self.subtitle_color)),
            ]));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.border_color));

        let paragraph = Paragraph::new(lines).block(block).alignment(self.alignment);

        paragraph.render(area, buf);
    }
}
