use crate::game::ascii_digits::get_digit_patterns;
use crate::domain::models::{Rank, SessionResult};
use crate::storage::repositories::SessionRepository;
use crate::storage::session_repository::BestStatus;
use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::{stdout, Write};

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
        session_result: &SessionResult,
        best_rank: Rank,
        center_col: u16,
        score_label_row: u16,
        best_status: Option<&BestStatus>,
    ) -> Result<u16> {
        let mut stdout = stdout();

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

        let score_label = "SESSION SCORE";
        let label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(label_col, score_label_row))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::score()))
        )?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        if let Some(best_type) = updated_best_type {
            let best_label = format!("*** {} BEST ***", best_type);
            let best_label_col = center_col.saturating_sub(best_label.len() as u16 / 2);
            execute!(stdout, MoveTo(best_label_col, score_label_row + 1))?;
            execute!(stdout, SetAttribute(Attribute::Bold))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::warning()))
            )?;
            execute!(stdout, Print(&best_label))?;
            execute!(stdout, ResetColor)?;
        }

        let score_value = format!("{:.0}", session_result.session_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = if updated_best_type.is_some() {
            score_label_row + 2
        } else {
            score_label_row + 1
        };

        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(best_rank.terminal_color())
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        let score_diff = session_result.session_score - comparison_score;
        let diff_text = if score_diff > 0.0 {
            format!("(+{:.0})", score_diff)
        } else if score_diff < 0.0 {
            format!("({:.0})", score_diff)
        } else {
            "(±0)".to_string()
        };

        let diff_col = center_col.saturating_sub(diff_text.len() as u16 / 2);
        execute!(
            stdout,
            MoveTo(diff_col, score_start_row + ascii_numbers.len() as u16 + 1)
        )?;
        execute!(stdout, SetAttribute(Attribute::Bold))?;
        if score_diff > 0.0 {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::success()))
            )?;
        } else if score_diff < 0.0 {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::error()))
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
        }
        execute!(stdout, Print(&diff_text))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        let ascii_height = ascii_numbers.len() as u16;
        Ok(score_start_row + ascii_height + 3)
    }
}
