use crate::game::ascii_rank_titles_generated::get_rank_display;
use crate::game::utils::{AsciiNumbersWidget, TerminalUtils};
use crate::storage::repositories::SessionRepository;
use crate::{models::GitRepository, Result};
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

#[derive(Debug)]
pub enum ResultAction {
    Restart,
    BackToTitle,
    Quit,
    Retry,
    Share,
}

pub struct SessionSummaryScreen;

impl SessionSummaryScreen {
    // Helper function to calculate actual display width without ANSI escape sequences
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

    // Removed: Now using AsciiNumbersWidget::create_ascii_numbers

    pub fn show_session_summary(
        session_result: &crate::models::SessionResult,
        _repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        let mut stdout = stdout();

        // Comprehensive screen reset
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, Hide)?; // Hide cursor
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Short delay to ensure terminal state is reset
        std::thread::sleep(std::time::Duration::from_millis(10));

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Use SessionResult data directly
        let best_rank = crate::scoring::Rank::for_score(session_result.session_score);

        // Calculate tier info for display
        let tier_info_values =
            crate::scoring::RankCalculator::calculate_tier_info(session_result.session_score);

        // Display rank as large ASCII art at the top
        let rank_lines = get_rank_display(best_rank.name());
        let rank_height = rank_lines.len() as u16;

        // Calculate total content height and center vertically
        let total_content_height = 4 + rank_height + 1 + 3 + 1 + 4 + 2 + 2; // session_title_space + rank + tier + gap_after_tier + label + score + gap + summary
        let rank_start_row = if total_content_height < terminal_height {
            center_row.saturating_sub(total_content_height / 2)
        } else {
            3
        };

        // Display session complete title at the top
        TerminalUtils::display_header(
            &mut stdout,
            "=== SESSION COMPLETE ===",
            Color::Cyan,
            rank_start_row.saturating_sub(4),
        )?;

        // Display "you're:" label before rank (1 line gap from rank)
        let youre_label = "YOU'RE:";
        let youre_col = center_col.saturating_sub(youre_label.len() as u16 / 2);
        execute!(stdout, MoveTo(youre_col, rank_start_row.saturating_sub(1)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(youre_label))?;
        execute!(stdout, ResetColor)?;

        for (row_index, line) in rank_lines.iter().enumerate() {
            // Calculate actual display width without ANSI codes for centering
            let display_width = Self::calculate_display_width(line);
            let line_col = center_col.saturating_sub(display_width / 2);
            execute!(stdout, MoveTo(line_col, rank_start_row + row_index as u16))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display tier information right after rank (small gap after rank)
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

        // Set color based on tier
        let tier_color = match best_rank.tier() {
            crate::models::RankTier::Beginner => Color::Blue,
            crate::models::RankTier::Intermediate => Color::Green,
            crate::models::RankTier::Advanced => Color::Cyan,
            crate::models::RankTier::Expert => Color::Yellow,
            crate::models::RankTier::Legendary => Color::Red,
        };
        execute!(stdout, SetForegroundColor(tier_color))?;
        execute!(stdout, Print(&tier_info))?;
        execute!(stdout, ResetColor)?;

        // Calculate score position based on rank height and tier info (add extra gap after tier info)
        let score_label_row = rank_start_row + rank_height + 4;

        // Check if this session updated any personal best and get today's score for diff
        let best_records = SessionRepository::get_best_records_global().ok().flatten();
        let mut updated_best_type = None;
        let todays_best_score = if let Some(records) = &best_records {
            // Check if we updated ALL TIME best
            if let Some(ref all_time) = records.all_time_best {
                if session_result.session_score >= all_time.score {
                    updated_best_type = Some("ALL TIME");
                }
            }
            // Check if we updated WEEKLY best (only if ALL TIME wasn't updated)
            else if let Some(ref weekly) = records.weekly_best {
                if session_result.session_score >= weekly.score {
                    updated_best_type = Some("WEEKLY");
                }
            }
            // Check if we updated TODAY'S best (only if neither ALL TIME nor WEEKLY was updated)
            else if let Some(ref today) = records.todays_best {
                if session_result.session_score >= today.score {
                    updated_best_type = Some("TODAY'S");
                }
            }

            // Get today's best score for diff calculation
            records.todays_best.as_ref().map(|t| t.score).unwrap_or(0.0)
        } else {
            // No records exist, so this is automatically a TODAY'S best
            updated_best_type = Some("TODAY'S");
            0.0
        };

        // Display "SESSION SCORE" or best achievement label
        let score_label = if let Some(_best_type) = updated_best_type {
            "SESSION SCORE"
        } else {
            "SESSION SCORE"
        };
        let label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(label_col, score_label_row))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        // Display best achievement if updated
        if let Some(best_type) = updated_best_type {
            let best_label = format!("*** {} BEST ***", best_type);
            let best_label_col = center_col.saturating_sub(best_label.len() as u16 / 2);
            execute!(stdout, MoveTo(best_label_col, score_label_row + 1))?;
            execute!(stdout, SetAttribute(Attribute::Bold))?;
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            execute!(stdout, Print(&best_label))?;
            execute!(stdout, ResetColor)?;
        }

        // Display large ASCII art session score with single bold color
        let score_value = format!("{:.0}", session_result.session_score);
        let ascii_numbers = AsciiNumbersWidget::create_ascii_numbers(&score_value);
        let score_start_row = if updated_best_type.is_some() {
            score_label_row + 2 // SESSION SCORE + *** BEST *** + ASCII
        } else {
            score_label_row + 1 // SESSION SCORE + ASCII
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

        // Always display score difference from today's best
        let score_diff = session_result.session_score - todays_best_score;
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
        )?; // +1 for line spacing
        execute!(stdout, SetAttribute(Attribute::Bold))?;
        if score_diff > 0.0 {
            execute!(stdout, SetForegroundColor(Color::Green))?;
        } else if score_diff < 0.0 {
            execute!(stdout, SetForegroundColor(Color::Red))?;
        } else {
            execute!(stdout, SetForegroundColor(Color::White))?;
        }
        execute!(stdout, Print(&diff_text))?;
        execute!(stdout, ResetColor)?;

        // Calculate summary position based on score height (add gap after score and diff)
        let ascii_height = ascii_numbers.len() as u16;
        let summary_start_row = score_start_row + ascii_height + 3; // ASCII + line spacing + diff + gap

        // Display session summary with compact format like other screens
        let summary_lines = [
            format!(
                "CPM: {:.0} | WPM: {:.0} | Time: {:.1}s",
                session_result.overall_cpm,
                session_result.overall_wpm,
                session_result.session_duration.as_secs_f64()
            ),
            format!(
                "Keystrokes: {} | Mistakes: {} | Accuracy: {:.1}%",
                session_result.valid_keystrokes + session_result.invalid_keystrokes,
                session_result.valid_mistakes + session_result.invalid_mistakes,
                session_result.overall_accuracy
            ),
        ];

        for (i, line) in summary_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, summary_start_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Calculate options start position based on summary display
        let options_start = summary_start_row + summary_lines.len() as u16 + 2;

        // Display options in two rows with color coding
        let row1_options = [
            ("[D]", " Show Detail", Color::Cyan),
            ("[S]", " Share Result", Color::Cyan),
        ];

        let row2_options = [
            ("[R]", " Retry", Color::Green),
            ("[T]", " Back to Title", Color::Green),
            ("[ESC]", " Quit", Color::Red),
        ];

        // First row
        let mut row1_text = String::new();
        for (i, (key, label, _)) in row1_options.iter().enumerate() {
            if i > 0 {
                row1_text.push_str("  ");
            }
            row1_text.push_str(key);
            row1_text.push_str(label);
        }
        let row1_col = center_col.saturating_sub(row1_text.len() as u16 / 2);
        execute!(stdout, MoveTo(row1_col, options_start))?;

        let mut _pos = 0;
        for (i, (key, label, key_color)) in row1_options.iter().enumerate() {
            if i > 0 {
                execute!(stdout, Print("  "))?;
                _pos += 2;
            }
            execute!(stdout, SetForegroundColor(*key_color))?;
            execute!(stdout, Print(key))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(label))?;
            _pos += key.len() + label.len();
        }
        execute!(stdout, ResetColor)?;

        // Second row
        let mut row2_text = String::new();
        for (i, (key, label, _)) in row2_options.iter().enumerate() {
            if i > 0 {
                row2_text.push_str("  ");
            }
            row2_text.push_str(key);
            row2_text.push_str(label);
        }
        let row2_col = center_col.saturating_sub(row2_text.len() as u16 / 2);
        execute!(stdout, MoveTo(row2_col, options_start + 1))?;

        for (i, (key, label, key_color)) in row2_options.iter().enumerate() {
            if i > 0 {
                execute!(stdout, Print("  "))?;
            }
            execute!(stdout, SetForegroundColor(*key_color))?;
            execute!(stdout, Print(key))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(label))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        Ok(())
    }

    pub fn show_session_summary_with_input(
        session_result: &crate::models::SessionResult,
        repo_info: &Option<GitRepository>,
        show_animation: bool,
    ) -> Result<ResultAction> {
        use crate::game::screens::AnimationScreen;

        // Show animation only if requested
        if show_animation {
            AnimationScreen::show_session_animation(session_result)?;
        }

        // Show the result screen
        Self::show_session_summary(session_result, repo_info)?;

        // Wait for user input and return action
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('d') | KeyCode::Char('D') => {
                            // Show details dialog using SessionResult directly
                            let _ = crate::game::screens::DetailsDialog::show_details(
                                session_result,
                                repo_info,
                            );
                            // Redraw the screen after dialog closes
                            Self::show_session_summary(session_result, repo_info)?;
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            return Ok(ResultAction::Retry);
                        }
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            return Ok(ResultAction::Share);
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            return Ok(ResultAction::BackToTitle);
                        }
                        KeyCode::Esc => {
                            return Ok(ResultAction::Quit);
                        }
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            crate::game::stage_manager::cleanup_terminal();
                            std::process::exit(0);
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
}
