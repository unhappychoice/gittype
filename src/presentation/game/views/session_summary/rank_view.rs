use crate::domain::models::Rank;
use crate::domain::services::scoring::RankCalculator;
use crate::presentation::game::ascii_rank_titles_generated::get_rank_display;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct RankView;

impl RankView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        best_rank: &Rank,
        session_score: f64,
    ) -> usize {
        let rank_lines = get_rank_display(best_rank.name());
        let rank_height = rank_lines.len();

        let tier_info_values = RankCalculator::calculate_tier_info(session_score);
        let tier_info = format!(
            "{} tier - {}/{} (overall {}/{})",
            tier_info_values.0,
            tier_info_values.1,
            tier_info_values.2,
            tier_info_values.3,
            tier_info_values.4
        );

        let mut constraints = vec![];
        for _ in &rank_lines {
            constraints.push(Constraint::Length(1));
        }
        constraints.push(Constraint::Length(1)); // Spacing
        constraints.push(Constraint::Length(1)); // Tier info

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        // Render rank ASCII art lines
        for (i, line) in rank_lines.iter().enumerate() {
            // Ratatui can render ANSI escape sequences directly
            let paragraph = Paragraph::new(line.as_str()).alignment(Alignment::Center);
            frame.render_widget(paragraph, chunks[i]);
        }

        // Render tier info
        let tier_info_span = Span::styled(
            tier_info,
            Style::default()
                .fg(best_rank.color())
                .add_modifier(Modifier::BOLD),
        );
        let tier_info_widget =
            Paragraph::new(Line::from(vec![tier_info_span])).alignment(Alignment::Center);
        frame.render_widget(tier_info_widget, chunks[rank_height + 1]);

        rank_height
    }
}
