use crate::domain::models::GitRepository;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct GitRepositoryView;

impl GitRepositoryView {
    pub fn render(frame: &mut Frame, git_repository: Option<&GitRepository>) {
        if let Some(info) = git_repository {
            let area = frame.area();

            // Build git info string
            let mut parts = vec![format!("📁 {}/{}", info.user_name, info.repository_name)];

            if let Some(ref branch) = info.branch {
                parts.push(format!("🌿 {}", branch));
            }

            if let Some(ref commit) = info.commit_hash {
                parts.push(format!("📝 {}", &commit[..8]));
            }

            let status_symbol = if info.is_dirty { "⚠️" } else { "✓" };
            parts.push(status_symbol.to_string());

            let git_text = parts.join(" • ");

            // Place at bottom of screen
            let bottom_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(area);

            let git_info = Paragraph::new(Line::from(vec![Span::styled(
                git_text,
                Style::default().fg(Colors::text_secondary()),
            )]))
            .alignment(Alignment::Center);

            frame.render_widget(git_info, bottom_chunks[1]);
        }
    }
}
