use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

use super::content_panel::ContentPanelWidget;

/// Scrollable list widget with consistent styling and built-in scrollbar
pub struct ScrollableListWidget<'a> {
    title: &'a str,
    items: Vec<ListItem<'a>>,
    border_color: Color,
    highlight_color: Color,
    text_color: Color,
    highlight_symbol: &'a str,
}

impl<'a> ScrollableListWidget<'a> {
    pub fn new(title: &'a str, items: Vec<ListItem<'a>>) -> Self {
        Self {
            title,
            items,
            border_color: Color::Blue,
            highlight_color: Color::DarkGray,
            text_color: Color::White,
            highlight_symbol: "► ",
        }
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn highlight_color(mut self, color: Color) -> Self {
        self.highlight_color = color;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn highlight_symbol(mut self, symbol: &'a str) -> Self {
        self.highlight_symbol = symbol;
        self
    }

    /// Render the list with state
    pub fn render_stateful(
        self,
        area: Rect,
        buf: &mut Buffer,
        list_state: &mut ListState,
        scrollbar_state: &mut ScrollbarState,
    ) {
        let block = ContentPanelWidget::new(self.title)
            .border_color(self.border_color)
            .block();

        let list = List::new(self.items)
            .block(block)
            .style(Style::default().fg(self.text_color))
            .highlight_style(
                Style::default()
                    .bg(self.highlight_color)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(self.highlight_symbol);

        StatefulWidget::render(list, area, buf, list_state);

        // Render scrollbar
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let scrollbar_area = area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        StatefulWidget::render(scrollbar, scrollbar_area, buf, scrollbar_state);
    }
}

impl<'a> Widget for ScrollableListWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = ContentPanelWidget::new(self.title)
            .border_color(self.border_color)
            .block();

        let list = List::new(self.items)
            .block(block)
            .style(Style::default().fg(self.text_color))
            .highlight_style(
                Style::default()
                    .bg(self.highlight_color)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(self.highlight_symbol);

        Widget::render(list, area, buf);
    }
}
