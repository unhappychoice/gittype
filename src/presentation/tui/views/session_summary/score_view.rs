use crate::domain::models::{Rank, SessionResult};
use crate::domain::repositories::session_repository::BestStatus;
use crate::domain::repositories::SessionRepository;
use crate::domain::models::ui::ascii_digits::get_digit_patterns;
use crate::domain::models::ui::rank_colors;
use crate::presentation::ui::{Colors, GradationText};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct ScoreView;

impl ScoreView {
    pub fn create_ascii_numbers(score: &str) -> Vec<String> {
        let digit_patterns = get_digit_patterns();
        let max_height = 4;
        let mut result = vec![String::new(); max_height];

        for ch in score.chars() {
            if let Some(digit) = ch.to_digit(10) {
                let pattern = &digit_patterns[digit as usize];
                for (i, line) in pattern.iter().enumerate() {
                    result[i].push_str(line);
                    result[i].push(' ');
                }
            }
        }

        result
    }

    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        session_result: &SessionResult,
        best_rank: &Rank,
        best_status: Option<&BestStatus>,
        colors: &Colors,
    ) -> usize {
        let (updated_best_type, comparison_score) = if let Some(status) = best_status {
            // For comparison, always use the most relevant previous score
            let comparison_score = if status.best_type.as_deref() == Some("ALL TIME") {
                status.all_time_best_score
            } else if status.best_type.as_deref() == Some("WEEKLY") {
                status.weekly_best_score
            } else if status.best_type.as_deref() == Some("TODAY'S") {
                status.todays_best_score
            } else {
                // No new best, but still compare against today's best if available
                status.todays_best_score
            };

            log::debug!(
                "ScoreView: best_type={:?}, comparison_score={}, session_score={}",
                status.best_type,
                comparison_score,
                session_result.session_score
            );
            log::debug!(
                "ScoreView: todays_best_score={}, weekly_best_score={}, all_time_best_score={}",
                status.todays_best_score,
                status.weekly_best_score,
                status.all_time_best_score
            );

            (status.best_type.as_deref(), comparison_score)
        } else {
            // Fallback to old behavior if no best_status provided
            let best_records = SessionRepository::get_best_records_global().ok().flatten();
            let mut updated_best_type = None;
            let comparison_score = if let Some(records) = &best_records {
                if let Some(ref all_time) = records.all_time_best {
                    if session_result.session_score >= all_time.score {
                        updated_best_type = Some("ALL TIME");
                    }
                    all_time.score
                } else if let Some(ref weekly) = records.weekly_best {
                    if session_result.session_score >= weekly.score {
                        updated_best_type = Some("WEEKLY");
                    }
                    weekly.score
                } else if let Some(ref today) = records.todays_best {
                    if session_result.session_score >= today.score {
                        updated_best_type = Some("TODAY'S");
                    }
                    today.score
                } else {
                    updated_best_type = Some("TODAY'S");
                    0.0
                }
            } else {
                updated_best_type = Some("TODAY'S");
                0.0
            };
            (updated_best_type, comparison_score)
        };

        let score_value = format!("{:.0}", session_result.session_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let ascii_height = ascii_numbers.len();

        let score_diff = session_result.session_score - comparison_score;
        let diff_text = if score_diff > 0.0 {
            format!("(+{:.0})", score_diff)
        } else if score_diff < 0.0 {
            format!("({:.0})", score_diff)
        } else {
            "(±0)".to_string()
        };

        let diff_color = if score_diff > 0.0 {
            colors.success()
        } else if score_diff < 0.0 {
            colors.error()
        } else {
            colors.text()
        };

        // Build layout
        let mut constraints = vec![
            Constraint::Length(1), // Score label
        ];
        if updated_best_type.is_some() {
            constraints.push(Constraint::Length(1)); // Best label
        }
        for _ in 0..ascii_height {
            constraints.push(Constraint::Length(1)); // ASCII number lines
        }
        constraints.push(Constraint::Length(1)); // Spacing
        constraints.push(Constraint::Length(1)); // Diff text

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        // Render score label
        let score_label = Paragraph::new(Line::from(vec![Span::styled(
            "SESSION SCORE",
            Style::default()
                .fg(colors.score())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(score_label, chunks[0]);

        let mut chunk_index = 1;

        // Render best label if present
        if let Some(best_type) = updated_best_type {
            let best_label = format!("*** {} BEST ***", best_type);
            let best_widget = Paragraph::new(Line::from(vec![Span::styled(
                best_label,
                Style::default()
                    .fg(colors.warning())
                    .add_modifier(Modifier::BOLD),
            )]))
            .alignment(Alignment::Center);
            frame.render_widget(best_widget, chunks[chunk_index]);
            chunk_index += 1;
        }

        // Render ASCII numbers with gradation
        let tier_colors = rank_colors::get_tier_colors(&best_rank.tier);
        for (i, line) in ascii_numbers.iter().enumerate() {
            let widget =
                GradationText::new(line.as_str(), tier_colors).alignment(Alignment::Center);
            frame.render_widget(widget, chunks[chunk_index + i]);
        }
        chunk_index += ascii_height;

        // Skip spacing
        chunk_index += 1;

        // Render diff text
        let diff_widget = Paragraph::new(Line::from(vec![Span::styled(
            diff_text,
            Style::default().fg(diff_color).add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(diff_widget, chunks[chunk_index]);

        ascii_height + 3
    }
}
