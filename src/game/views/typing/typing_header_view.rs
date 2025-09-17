use crate::{
    models::{Challenge, GitRepository},
    ui::Colors,
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
    ) {
        let header_text = if let Some(challenge) = challenge {
            let difficulty_text = match &challenge.difficulty_level {
                Some(difficulty) => format!("{:?}", difficulty),
                None => "Unknown".to_string(),
            };

            let base_title = challenge.get_display_title_with_repo(&git_repository.cloned());

            // Create spans for colored language display before difficulty
            let mut spans = vec![Span::from(base_title)];

            // Add language with color if available
            if let Some(ref language) = challenge.language {
                use crate::extractor::models::language::LanguageRegistry;
                let language_color = LanguageRegistry::get_color(Some(language));
                let display_name = LanguageRegistry::get_display_name(Some(language));
                spans.push(Span::from(" "));
                spans.push(Span::styled(
                    format!("[{}]", display_name),
                    Style::default().fg(language_color),
                ));
            }

            // Add difficulty at the end
            spans.push(Span::from(format!(" [{}]", difficulty_text)));

            Line::from(spans)
        } else {
            Line::from("[Challenge]")
        };

        let header = Paragraph::new(vec![header_text]).block(
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
