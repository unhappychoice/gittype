use crate::presentation::tui::screens::loading_screen::LoadingScreenState;
use crate::presentation::tui::views::loading::loading_description_view::LoadingDescriptionView;
use crate::presentation::tui::views::loading::loading_message_view::LoadingMessageView;
use crate::presentation::tui::views::loading::loading_progress_view::LoadingProgressView;
use crate::presentation::tui::views::loading::loading_repo_info_view::LoadingRepoInfoView;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub struct LoadingMainView;

impl LoadingMainView {
    pub fn render(frame: &mut Frame, state: &LoadingScreenState, colors: &Colors) {
        let size = frame.area();

        // Get repo info for bottom display
        let repo_info = state
            .repo_info
            .read()
            .map(|r| r.clone())
            .unwrap_or_default();

        // Calculate main content height (excluding repo info at bottom)
        let content_height = 2 + 8 + 1 + 3; // Loading message + Description + Spacing + Progress
        let vertical_margin = (size.height.saturating_sub(content_height)) / 2;

        // Create layout with main content centered and repo info at bottom
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(vertical_margin),
                Constraint::Length(2), // Loading message
                Constraint::Length(8), // Description
                Constraint::Length(1), // Spacing
                Constraint::Length(3), // Progress
                Constraint::Min(1),    // Flexible space
                Constraint::Length(1), // Repo info at bottom
            ])
            .split(size);

        // Draw loading message
        LoadingMessageView::render(frame, main_layout[1], colors);

        // Draw description
        LoadingDescriptionView::render(frame, main_layout[2], state, colors);

        // Skip main_layout[3] for spacing

        // Draw progress
        LoadingProgressView::render(frame, main_layout[4], state, colors);

        // Draw repo info at bottom if available
        if let Some(ref repo_info_text) = repo_info {
            LoadingRepoInfoView::render(frame, main_layout[6], repo_info_text, colors);
        }
    }
}
