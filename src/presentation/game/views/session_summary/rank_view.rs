use crate::domain::services::scoring::RankCalculator;
use crate::presentation::game::ascii_rank_titles_generated::get_rank_display;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct RankView;

impl RankView {
    fn calculate_display_width(text: &str) -> u16 {
        let mut width = 0;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Skip ANSI escape sequence
                if chars.peek() == Some(&'[') {
                    chars.next(); // consume '['
                    for seq_ch in chars.by_ref() {
                        if seq_ch.is_ascii_alphabetic() {
                            break; // End of escape sequence
                        }
                    }
                }
            } else if !ch.is_control() {
                width += 1;
            }
        }

        width as u16
    }

    pub fn render(
        best_rank: crate::domain::services::scoring::Rank,
        session_score: f64,
        center_col: u16,
        rank_start_row: u16,
    ) -> Result<u16> {
        let mut stdout = stdout();

        let rank_lines = get_rank_display(best_rank.name());
        let rank_height = rank_lines.len() as u16;

        let tier_info_values = RankCalculator::calculate_tier_info(session_score);

        for (row_index, line) in rank_lines.iter().enumerate() {
            let display_width = Self::calculate_display_width(line);
            let line_col = center_col.saturating_sub(display_width / 2);
            execute!(stdout, MoveTo(line_col, rank_start_row + row_index as u16))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        let tier_info_row = rank_start_row + rank_height + 1;
        let tier_info = format!(
            "{} tier - {}/{} (overall {}/{})",
            tier_info_values.0,
            tier_info_values.1,
            tier_info_values.2,
            tier_info_values.3,
            tier_info_values.4
        );
        let tier_info_col = center_col.saturating_sub(tier_info.len() as u16 / 2);
        execute!(stdout, MoveTo(tier_info_col, tier_info_row))?;
        execute!(stdout, SetAttribute(Attribute::Bold))?;

        let tier_color = match best_rank.tier() {
            crate::domain::models::RankTier::Beginner => Colors::to_crossterm(Colors::border()),
            crate::domain::models::RankTier::Intermediate => {
                Colors::to_crossterm(Colors::success())
            }
            crate::domain::models::RankTier::Advanced => Colors::to_crossterm(Colors::info()),
            crate::domain::models::RankTier::Expert => Colors::to_crossterm(Colors::warning()),
            crate::domain::models::RankTier::Legendary => Colors::to_crossterm(Colors::error()),
        };
        execute!(stdout, SetForegroundColor(tier_color))?;
        execute!(stdout, Print(&tier_info))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(rank_height)
    }
}
