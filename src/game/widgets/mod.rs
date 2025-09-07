pub mod content_panel;
pub mod control_bar;
pub mod dialog_widget;
pub mod header;
pub mod progress_display;
pub mod scrollable_list;

// Re-export the main widgets
pub use content_panel::ContentPanelWidget;
pub use control_bar::ControlBarWidget;
pub use dialog_widget::DialogWidget;
pub use header::HeaderWidget;
pub use progress_display::ProgressDisplayWidget;
pub use scrollable_list::ScrollableListWidget;

// Re-export the old function-based API for backward compatibility during transition
mod legacy {
    use super::*;
    use crate::game::utils::LayoutHelpers;
    use ratatui::{
        layout::{Margin, Rect},
        style::{Color, Modifier, Style},
        widgets::{Block, List, ListItem, Scrollbar},
    };

    // Legacy function-based API - use new widgets internally
    pub fn default_block(title: &str) -> Block<'_> {
        ContentPanelWidget::new(title).block()
    }

    pub fn header_block(title: &str) -> Block<'_> {
        ContentPanelWidget::new(title)
            .border_style(Style::default())
            .block()
    }

    pub fn accent_block(title: &str, color: Color) -> Block<'_> {
        ContentPanelWidget::new(title).border_color(color).block()
    }

    pub fn default_list<'a>(items: Vec<ListItem<'a>>, title: &'a str) -> List<'a> {
        List::new(items)
            .block(default_block(title))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("► ")
    }

    pub fn default_scrollbar() -> Scrollbar<'static> {
        Scrollbar::default()
            .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
    }

    pub fn standard_padding_layout(area: Rect) -> [Rect; 3] {
        LayoutHelpers::standard_padding(area)
    }

    pub fn minimal_padding_layout(area: Rect) -> [Rect; 3] {
        LayoutHelpers::minimal_padding(area)
    }

    pub fn main_screen_layout(area: Rect) -> [Rect; 4] {
        LayoutHelpers::main_screen(area)
    }

    pub fn scrollbar_inner_area(area: Rect) -> Rect {
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        })
    }
}

// Re-export legacy functions for backward compatibility
pub use legacy::*;
