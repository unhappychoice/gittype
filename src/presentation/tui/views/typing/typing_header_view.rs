use crate::domain::models::Languages;
use crate::{
    domain::models::{Challenge, GitRepository},
    presentation::ui::Colors,
};
use ratatui::{
    style::Style,
    text::{Line, Span},
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
        colors: &Colors,
    ) {
        let header_text = if let Some(challenge) = challenge {
            let difficulty_text = match &challenge.difficulty_level {
                Some(difficulty) => format!("{:?}", difficulty),
                None => "Unknown".to_string(),
            };

            let base_title = challenge.get_display_title_with_repo(&git_repository.cloned());

            // Create spans for colored language display before difficulty
            let mut spans = vec![Span::styled(
                base_title,
                Style::default().fg(colors.text_secondary()),
            )];

            // Add language with color if available
            if let Some(ref language) = challenge.language {
                let display_name = Languages::get_display_name(Some(language));
                spans.push(Span::styled(
                    " ",
                    Style::default().fg(colors.text_secondary()),
                ));
                spans.push(Span::styled(
                    format!("[{}]", display_name),
                    Style::default().fg(colors.info()),
                ));
            }

            // Add difficulty at the end
            spans.push(Span::styled(
                format!(" [{}]", difficulty_text),
                Style::default().fg(colors.text_secondary()),
            ));

            Line::from(spans)
        } else {
            Line::from(vec![Span::styled(
                "[Challenge]",
                Style::default().fg(colors.text_secondary()),
            )])
        };

        let header = Paragraph::new(vec![header_text]).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border()))
                .title("Challenge")
                .title_style(Style::default().fg(colors.border()))
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
        frame.render_widget(header, area);
    }
}
