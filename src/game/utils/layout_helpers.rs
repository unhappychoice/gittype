use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Helper struct for common layout patterns
pub struct LayoutHelpers;

impl LayoutHelpers {
    /// Create horizontal layout with padding on left and right
    pub fn horizontal_with_padding(area: Rect, left: u16, right: u16) -> [Rect; 3] {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(left),
                Constraint::Min(1),
                Constraint::Length(right),
            ])
            .split(area);
        [chunks[0], chunks[1], chunks[2]]
    }

    /// Standard padding layout (4 units on each side)
    pub fn standard_padding(area: Rect) -> [Rect; 3] {
        Self::horizontal_with_padding(area, 4, 4)
    }

    /// Minimal padding layout (2 units on each side)
    pub fn minimal_padding(area: Rect) -> [Rect; 3] {
        Self::horizontal_with_padding(area, 2, 2)
    }

    /// Main screen layout (header, tabs, content, controls)
    pub fn main_screen(area: Rect) -> [Rect; 4] {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Tabs/Navigation
                Constraint::Min(1),    // Content
                Constraint::Length(1), // Controls
            ])
            .split(area);
        [chunks[0], chunks[1], chunks[2], chunks[3]]
    }

    /// Two-column layout with specified percentages
    pub fn two_columns(area: Rect, left_pct: u16, right_pct: u16) -> [Rect; 2] {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(left_pct),
                Constraint::Percentage(right_pct),
            ])
            .split(area);
        [chunks[0], chunks[1]]
    }

    /// Equal two-column layout (50/50 split)
    pub fn two_columns_equal(area: Rect) -> [Rect; 2] {
        Self::two_columns(area, 50, 50)
    }

    /// Three-column layout
    pub fn three_columns(area: Rect, left_pct: u16, middle_pct: u16, right_pct: u16) -> [Rect; 3] {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(left_pct),
                Constraint::Percentage(middle_pct),
                Constraint::Percentage(right_pct),
            ])
            .split(area);
        [chunks[0], chunks[1], chunks[2]]
    }

    /// Vertical split with header and content
    pub fn header_content(area: Rect, header_height: u16) -> [Rect; 2] {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(header_height), Constraint::Min(1)])
            .split(area);
        [chunks[0], chunks[1]]
    }

    /// Vertical split with header, content, and footer
    pub fn header_content_footer(area: Rect, header_height: u16, footer_height: u16) -> [Rect; 3] {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(header_height),
                Constraint::Min(1),
                Constraint::Length(footer_height),
            ])
            .split(area);
        [chunks[0], chunks[1], chunks[2]]
    }
}
