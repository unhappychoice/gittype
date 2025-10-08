use crate::domain::models::DifficultyLevel;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct DifficultySelectionView;

impl DifficultySelectionView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        difficulties: &[(&str, DifficultyLevel); 5],
        selected_difficulty: usize,
        challenge_counts: &[usize; 5],
        error_message: Option<&String>,
    ) {
        let (name, difficulty_level) = &difficulties[selected_difficulty];
        let count = challenge_counts[selected_difficulty];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Difficulty selection
                Constraint::Length(1), // Challenge count
                Constraint::Length(1), // Description line 1 / Error
                Constraint::Length(1), // Description line 2
            ])
            .split(area);

        // Line 1: Difficulty selection
        let difficulty_line = Line::from(vec![
            Span::styled("Difficulty: ", Style::default().fg(Colors::text())),
            Span::styled("← ", Style::default().fg(Colors::accuracy())),
            Span::styled(
                name.to_string(),
                Style::default()
                    .fg(Colors::text())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" →", Style::default().fg(Colors::accuracy())),
        ]);
        frame.render_widget(
            Paragraph::new(difficulty_line).alignment(Alignment::Center),
            chunks[0],
        );

        // Line 2: Challenge count
        let count_text = if count > 0 {
            format!("{} challenges available", count)
        } else {
            "Challenge count will be displayed after loading".to_string()
        };
        let count_line = Paragraph::new(Line::from(vec![Span::styled(
            count_text,
            Style::default()
                .fg(Colors::info())
                .add_modifier(Modifier::DIM),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(count_line, chunks[1]);

        // Line 3 & 4: Description lines or error message
        if let Some(error) = error_message {
            // Display error message in red
            let error_line = Paragraph::new(Line::from(vec![Span::styled(
                error.as_str(),
                Style::default()
                    .fg(Colors::error())
                    .add_modifier(Modifier::BOLD),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(error_line, chunks[2]);
        } else {
            let descriptions = [difficulty_level.description(), difficulty_level.subtitle()];
            for (i, description) in descriptions.iter().enumerate() {
                let desc_line = Paragraph::new(Line::from(vec![Span::styled(
                    *description,
                    Style::default()
                        .fg(Colors::text())
                        .add_modifier(Modifier::DIM),
                )]))
                .alignment(Alignment::Center);
                frame.render_widget(desc_line, chunks[2 + i]);
            }
        }
    }
}
