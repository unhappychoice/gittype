use crate::domain::models::ui::ascii_rank_titles;
use crate::domain::models::ui::rank_colors;
use crate::domain::models::Rank;
use crate::domain::services::scoring::RankCalculator;
use crate::presentation::ui::GradationText;
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
        let rank_patterns = ascii_rank_titles::get_all_rank_patterns();
        let rank_lines = match rank_patterns.get(best_rank.name()) {
            Some(lines) => lines,
            None => return 0,
        };
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

        // Get tier colors for gradation
        let tier_colors = rank_colors::get_tier_colors(&best_rank.tier);

        // Check if the last line is empty (all spaces)
        let last_line_is_empty = rank_lines
            .last()
            .map(|line| line.trim().is_empty())
            .unwrap_or(false);

        let mut constraints = vec![];
        for _ in rank_lines.iter() {
            constraints.push(Constraint::Length(1));
        }
        if !last_line_is_empty {
            constraints.push(Constraint::Length(1)); // Spacing line if needed
        }
        constraints.push(Constraint::Length(1)); // Tier info

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        // Render rank ASCII art lines with gradation
        for (i, line) in rank_lines.iter().enumerate() {
            let widget = GradationText::new(line, tier_colors).alignment(Alignment::Center);
            frame.render_widget(widget, chunks[i]);
        }

        // Render tier info at the appropriate chunk index
        let tier_info_index = if last_line_is_empty {
            rank_height
        } else {
            rank_height + 1
        };

        let tier_info_span = Span::styled(
            tier_info,
            Style::default()
                .fg(best_rank.color())
                .add_modifier(Modifier::BOLD),
        );
        let tier_info_widget =
            Paragraph::new(Line::from(vec![tier_info_span])).alignment(Alignment::Center);
        frame.render_widget(tier_info_widget, chunks[tier_info_index]);

        // Return total height (ASCII art + spacing if needed + tier info)
        if last_line_is_empty {
            rank_height + 1
        } else {
            rank_height + 2
        }
    }
}
