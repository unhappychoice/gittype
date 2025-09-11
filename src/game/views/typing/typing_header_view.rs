use crate::{
    models::{Challenge, GitRepository},
    ui::Colors,
};
use ratatui::{
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TypingHeaderView;

impl TypingHeaderView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        challenge: Option<&Challenge>,
        git_repository: Option<&GitRepository>,
    ) {
        let header_text = if let Some(challenge) = challenge {
            let difficulty_text = match &challenge.difficulty_level {
                Some(difficulty) => format!("{:?}", difficulty),
                None => "Unknown".to_string(),
            };
            format!(
                "{} [{}]",
                challenge.get_display_title_with_repo(&git_repository.cloned()),
                difficulty_text
            )
        } else {
            "[Challenge]".to_string()
        };

        let header = Paragraph::new(vec![Line::from(header_text)]).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Colors::BORDER))
                .title("Challenge")
                .title_style(Style::default().fg(Colors::BORDER))
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
        frame.render_widget(header, area);
    }
}
