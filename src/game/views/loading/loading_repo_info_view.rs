use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct LoadingRepoInfoView;

impl LoadingRepoInfoView {
    pub fn render(frame: &mut Frame, area: Rect, repo_info: &str) {
        // Use same style as title_screen: DarkGrey color and centered
        let repo_line = Line::from(Span::styled(
            repo_info,
            Style::default().fg(Colors::text_secondary()),
        ));

        let repo_widget = Paragraph::new(vec![repo_line]).alignment(Alignment::Center);

        frame.render_widget(repo_widget, area);
    }
}
