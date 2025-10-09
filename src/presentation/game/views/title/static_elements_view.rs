use crate::domain::models::GitRepository;
use crate::presentation::game::views::title::{logo, GitRepositoryView};
use crate::presentation::ui::{Colors, GradationText};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct StaticElementsView;

impl StaticElementsView {
    pub fn render(
        frame: &mut Frame,
        logo_area: ratatui::layout::Rect,
        subtitle_area: ratatui::layout::Rect,
        instructions_area: ratatui::layout::Rect,
        git_repository: Option<&GitRepository>,
    ) {
        // Render logo
        let logo_lines = logo::get_logo_lines();
        let logo_colors = logo::get_logo_colors();
        let logo_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); 6])
            .split(logo_area);

        for (i, line) in logo_lines.iter().enumerate() {
            let widget = GradationText::new(line, logo_colors).alignment(Alignment::Center);
            frame.render_widget(widget, logo_chunks[i]);
        }

        // Render subtitle
        let subtitle = Paragraph::new(Line::from(vec![Span::styled(
            "Code Typing Challenge",
            Style::default().fg(Colors::text_secondary()),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(subtitle, subtitle_area);

        // Render instructions
        let instructions_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Tier 1: Change Difficulty
                Constraint::Length(1), // Tier 2: Secondary actions
                Constraint::Length(1), // Tier 3: Primary actions
            ])
            .split(instructions_area);

        // Tier 1: Change Difficulty
        let tier1 = Line::from(vec![
            Span::styled("[←→/HL]", Style::default().fg(Colors::key_navigation())),
            Span::styled(" Change Difficulty", Style::default().fg(Colors::text())),
        ]);
        frame.render_widget(
            Paragraph::new(tier1).alignment(Alignment::Center),
            instructions_chunks[0],
        );

        // Tier 2: Secondary actions
        let tier2 = Line::from(vec![
            Span::styled("[R]", Style::default().fg(Colors::info())),
            Span::styled(" Records  ", Style::default().fg(Colors::text())),
            Span::styled("[A]", Style::default().fg(Colors::info())),
            Span::styled(" Analytics  ", Style::default().fg(Colors::text())),
            Span::styled("[S]", Style::default().fg(Colors::info())),
            Span::styled(" Settings  ", Style::default().fg(Colors::text())),
            Span::styled("[I/?]", Style::default().fg(Colors::info())),
            Span::styled(" Help", Style::default().fg(Colors::text())),
        ]);
        frame.render_widget(
            Paragraph::new(tier2).alignment(Alignment::Center),
            instructions_chunks[1],
        );

        // Tier 3: Primary actions
        let tier3 = Line::from(vec![
            Span::styled("[SPACE]", Style::default().fg(Colors::success())),
            Span::styled(" Start  ", Style::default().fg(Colors::text())),
            Span::styled("[ESC]", Style::default().fg(Colors::error())),
            Span::styled(" Quit", Style::default().fg(Colors::text())),
        ]);
        frame.render_widget(
            Paragraph::new(tier3).alignment(Alignment::Center),
            instructions_chunks[2],
        );

        // Render git repository info
        GitRepositoryView::render(frame, git_repository);
    }
}
