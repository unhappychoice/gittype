use crate::game::ascii_digits::get_digit_patterns;
use crate::game::ascii_rank_titles_generated::get_rank_title_display;
use crate::scoring::{RankingTitle, ScoringEngine, StageResult};
use crate::{models::GitRepository, Result};
use crossterm::{
    cursor::MoveTo,
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

    pub fn show_stage_completion(
        metrics: &StageResult,
        current_stage: usize,
        total_stages: usize,
        has_next_stage: bool,
        keystrokes: usize,
    ) -> Result<Option<ResultAction>> {
        let mut stdout = stdout();

        // Comprehensive screen reset
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Short delay to ensure terminal state is reset
        std::thread::sleep(std::time::Duration::from_millis(10));

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Display stage title at the center
        let stage_title = if metrics.was_failed {
            format!("=== STAGE {} FAILED ===", current_stage)
        } else if metrics.was_skipped {
            format!("=== STAGE {} SKIPPED ===", current_stage)
        } else {
            format!("=== STAGE {} COMPLETE ===", current_stage)
        };

        // Use simple character count for more reliable centering
        let title_col = center_col.saturating_sub(stage_title.len() as u16 / 2);

        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(6)))?;
        execute!(stdout, SetAttribute(Attribute::Bold))?;
        if metrics.was_failed {
            execute!(stdout, SetForegroundColor(Color::Red))?;
        } else if metrics.was_skipped {
            execute!(stdout, SetForegroundColor(Color::Magenta))?;
        } else {
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
        }
        execute!(stdout, Print(&stage_title))?;
        execute!(stdout, ResetColor)?;

        // Position score label below title
        let score_label_row = center_row.saturating_sub(3);

        // Display different label and score for skipped/failed challenges
        if metrics.was_failed {
            let fail_message = "Challenge failed - no score";
            let fail_col = center_col.saturating_sub(fail_message.len() as u16 / 2);
            execute!(stdout, MoveTo(fail_col, score_label_row))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Red)
            )?;
            execute!(stdout, Print(fail_message))?;
            execute!(stdout, ResetColor)?;
        } else if metrics.was_skipped {
            let skip_message = "Challenge skipped - no score";
            let skip_col = center_col.saturating_sub(skip_message.len() as u16 / 2);
            execute!(stdout, MoveTo(skip_col, score_label_row))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::DarkGrey)
            )?;
            execute!(stdout, Print(skip_message))?;
            execute!(stdout, ResetColor)?;
        } else {
            // Display "SCORE" label
            let score_label = "SCORE";
            let score_label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
            execute!(stdout, MoveTo(score_label_col, score_label_row))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Cyan)
            )?;
            execute!(stdout, Print(score_label))?;
            execute!(stdout, ResetColor)?;
        }

        // Display large ASCII art numbers
        let score_value = if metrics.was_failed || metrics.was_skipped {
            "---".to_string()
        } else {
            format!("{:.0}", metrics.challenge_score)
        };
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = score_label_row + 1;

        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(stdout, SetAttribute(Attribute::Bold))?;
            if metrics.was_failed {
                execute!(stdout, SetForegroundColor(Color::Red))?;
            } else if metrics.was_skipped {
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(
                        RankingTitle::for_score(metrics.challenge_score).terminal_color()
                    )
                )?;
            }
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Calculate dynamic positioning for metrics in stage completion (add gap after score)
        let ascii_height = ascii_numbers.len() as u16;
        let stage_metrics_row = score_start_row + ascii_height + 2;

        // Display compact metrics below the score
        let metrics_lines = [
            format!(
                "CPM: {:.0} | WPM: {:.0} | Time: {:.1}s",
                metrics.cpm,
                metrics.wpm,
                metrics.completion_time.as_secs_f64()
            ),
            format!(
                "Keystrokes: {} | Accuracy: {:.1}% | Mistakes: {}",
                keystrokes, metrics.accuracy, metrics.mistakes
            ),
        ];

        for (i, line) in metrics_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, stage_metrics_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Calculate dynamic positioning for progress and continue text
        let progress_start_row = stage_metrics_row + metrics_lines.len() as u16 + 2;

        // Show progress and next action
        if has_next_stage {
            let progress_text = format!("Progress: {} / {}", current_stage, total_stages);
            let progress_col = center_col.saturating_sub(progress_text.len() as u16 / 2);
            execute!(stdout, MoveTo(progress_col, progress_start_row))?;
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print(&progress_text))?;
            execute!(stdout, ResetColor)?;

            let next_text = "Next stage starting...";
            let next_col = center_col.saturating_sub(next_text.len() as u16 / 2);
            execute!(stdout, MoveTo(next_col, progress_start_row + 1))?;
            execute!(stdout, SetForegroundColor(Color::Yellow))?;
            execute!(stdout, Print(next_text))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Show stage completion options with color coding
        let options_row = if has_next_stage {
            progress_start_row + 3
        } else {
            progress_start_row
        };

        // Calculate position for centered text
        let total_text_length = "[SPACE] Continue  [ESC] Quit".len();
        let start_col = center_col.saturating_sub(total_text_length as u16 / 2);

        execute!(stdout, MoveTo(start_col, options_row))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print("[SPACE]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Continue  "))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Quit"))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char(' ') => break, // Continue
                        KeyCode::Esc => {
                            return Ok(Some(ResultAction::Quit));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn show_session_summary(
        total_stages: usize,
        completed_stages: usize,
        stage_engines: &[(String, ScoringEngine)],
    ) -> Result<()> {
        use crate::game::screens::AnimationScreen;

        // First show the animation
        AnimationScreen::show_session_animation(total_stages, completed_stages, stage_engines)?;

        // Then show the original result screen
        Self::show_session_summary_original(total_stages, completed_stages, stage_engines, &None)
    }

    pub fn show_session_summary_original(
        _total_stages: usize,
        _completed_stages: usize,
        stage_engines: &[(String, ScoringEngine)],
        repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        let mut stdout = stdout();

        // Comprehensive screen reset
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Short delay to ensure terminal state is reset
        std::thread::sleep(std::time::Duration::from_millis(10));

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Calculate aggregated session metrics by combining ScoringEngines with + operator
        if stage_engines.is_empty() {
            return Ok(());
        }

        let combined_engine = stage_engines
            .iter()
            .map(|(_, engine)| engine.clone())
            .reduce(|acc, engine| acc + engine)
            .unwrap(); // Safe because we checked is_empty() above

        let session_metrics = match combined_engine.calculate_result() {
            Ok(metrics) => metrics,
            Err(_) => {
                // Fallback if calculation fails
                return Ok(());
            }
        };

        // Display ranking title as large ASCII art at the top
        let rank_title_lines = get_rank_title_display(&session_metrics.ranking_title);
        let rank_title_height = rank_title_lines.len() as u16;

        // Calculate total content height and center vertically
        let total_content_height = 4 + rank_title_height + 1 + 3 + 1 + 4 + 2 + 2; // session_title_space + rank + tier + gap_after_tier + label + score + gap + summary
        let rank_start_row = if total_content_height < terminal_height {
            center_row.saturating_sub(total_content_height / 2)
        } else {
            3
        };

        // Display session complete title at the top
        let session_title = "=== SESSION COMPLETE ===";
        let title_col = center_col.saturating_sub(session_title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, rank_start_row.saturating_sub(4)))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(session_title))?;
        execute!(stdout, ResetColor)?;

        // Display "you're:" label before rank title (1 line gap from rank title)
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

        for (row_index, line) in rank_title_lines.iter().enumerate() {
            // Calculate actual display width without ANSI codes for centering
            let display_width = Self::calculate_display_width(line);
            let line_col = center_col.saturating_sub(display_width / 2);
            execute!(stdout, MoveTo(line_col, rank_start_row + row_index as u16))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display tier information right after rank title (small gap after rank title)
        let tier_info_row = rank_start_row + rank_title_height + 1;
        let tier_info = format!(
            "{} rank - {}/{} (overall {}/{})",
            session_metrics.rank,
            session_metrics.tier_position,
            session_metrics.tier_total,
            session_metrics.overall_position,
            session_metrics.overall_total
        );
        let tier_info_col = center_col.saturating_sub(tier_info.len() as u16 / 2);
        execute!(stdout, MoveTo(tier_info_col, tier_info_row))?;
        execute!(stdout, SetAttribute(Attribute::Bold))?;

        // Set color based on tier
        let tier_color = match session_metrics.rank.as_str() {
            "Beginner" => Color::Blue,
            "Intermediate" => Color::Green,
            "Advanced" => Color::Cyan,
            "Expert" => Color::Yellow,
            "Legendary" => Color::Red,
            _ => Color::White,
        };
        execute!(stdout, SetForegroundColor(tier_color))?;
        execute!(stdout, Print(&tier_info))?;
        execute!(stdout, ResetColor)?;

        // Calculate score position based on rank title height and tier info (add extra gap after tier info)
        let score_label_row = rank_start_row + rank_title_height + 4;

        // Display "SCORE" label in normal text with color
        let score_label = "SESSION SCORE";
        let label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(label_col, score_label_row))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        // Display large ASCII art session score with single bold color
        let score_value = format!("{:.0}", session_metrics.challenge_score);
        let ascii_numbers = Self::create_ascii_numbers(&score_value);
        let score_start_row = score_label_row + 1;

        for (row_index, line) in ascii_numbers.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, score_start_row + row_index as u16))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(
                    RankingTitle::for_score(session_metrics.challenge_score).terminal_color()
                )
            )?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Calculate summary position based on score height (add gap after score)
        let ascii_height = ascii_numbers.len() as u16;
        let summary_start_row = score_start_row + ascii_height + 2;

        // Display session summary with compact format like other screens
        let summary_lines = [
            format!(
                "CPM: {:.0} | WPM: {:.0} | Time: {:.1}s",
                session_metrics.cpm,
                session_metrics.wpm,
                session_metrics.completion_time.as_secs_f64()
            ),
            format!(
                "Keystrokes: {} | Accuracy: {:.1}% | Mistakes: {}",
                stage_engines
                    .iter()
                    .map(|(_, e)| e.total_chars())
                    .sum::<usize>(),
                session_metrics.accuracy,
                session_metrics.mistakes
            ),
        ];

        for (i, line) in summary_lines.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, summary_start_row + i as u16))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display individual stage results
        if !stage_engines.is_empty() {
            let stage_results_start_row = summary_start_row + summary_lines.len() as u16 + 2;

            let stage_label = if let Some(repo) = repo_info {
                format!(
                    "Stage Results: [{}/{}]",
                    repo.user_name, repo.repository_name
                )
            } else {
                "Stage Results:".to_string()
            };
            let stage_label_col = center_col.saturating_sub(stage_label.len() as u16 / 2);
            execute!(stdout, MoveTo(stage_label_col, stage_results_start_row))?;
            execute!(stdout, SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print(&stage_label))?;
            execute!(stdout, ResetColor)?;

            // Calculate maximum stage name width for alignment
            let max_stage_name_width = stage_engines
                .iter()
                .take(5)
                .map(|(stage_name, _)| stage_name.len())
                .max()
                .unwrap_or(0);

            for (i, (stage_name, engine)) in stage_engines.iter().enumerate().take(5) {
                let result_line = format!(
                    "{:>width$}: CPM {:.0}, WPM {:.0}, Acc {:.1}%",
                    stage_name,
                    engine.cpm(),
                    engine.wpm(),
                    engine.accuracy(),
                    width = max_stage_name_width
                );
                let result_col = center_col.saturating_sub(result_line.len() as u16 / 2);
                execute!(
                    stdout,
                    MoveTo(result_col, stage_results_start_row + 1 + i as u16)
                )?;
                execute!(stdout, SetForegroundColor(Color::Grey))?;
                execute!(stdout, Print(&result_line))?;
                execute!(stdout, ResetColor)?;
            }
        }

        // Display options with color coding
        let options_data = [
            ("[R]", " Retry", Color::Green),
            ("[S]", " Share Result", Color::Cyan),
            ("[T]", " Back to Title", Color::Green),
            ("[ESC]", " Quit", Color::Red),
        ];

        let options_start = if !stage_engines.is_empty() {
            let stage_results_start_row = summary_start_row + summary_lines.len() as u16 + 2;
            stage_results_start_row + 7 // stage label + 5 stages + gap
        } else {
            summary_start_row + summary_lines.len() as u16 + 2
        };

        for (i, (key, label, key_color)) in options_data.iter().enumerate() {
            let full_text_len = key.len() + label.len();
            let option_col = center_col.saturating_sub(full_text_len as u16 / 2);
            execute!(stdout, MoveTo(option_col, options_start + i as u16))?;
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
        total_stages: usize,
        completed_stages: usize,
        stage_engines: &[(String, ScoringEngine)],
        repo_info: &Option<GitRepository>,
    ) -> Result<ResultAction> {
        Self::show_session_summary_with_input_internal(
            total_stages,
            completed_stages,
            stage_engines,
            repo_info,
            true,
        )
    }

    pub fn show_session_summary_with_input_no_animation(
        total_stages: usize,
        completed_stages: usize,
        stage_engines: &[(String, ScoringEngine)],
        repo_info: &Option<GitRepository>,
    ) -> Result<ResultAction> {
        Self::show_session_summary_with_input_internal(
            total_stages,
            completed_stages,
            stage_engines,
            repo_info,
            false,
        )
    }

    fn show_session_summary_with_input_internal(
        total_stages: usize,
        completed_stages: usize,
        stage_engines: &[(String, ScoringEngine)],
        repo_info: &Option<GitRepository>,
        show_animation: bool,
    ) -> Result<ResultAction> {
        use crate::game::screens::AnimationScreen;

        // Show animation only if requested
        if show_animation {
            AnimationScreen::show_session_animation(total_stages, completed_stages, stage_engines)?;
        }

        // Show the result screen
        Self::show_session_summary_original(
            total_stages,
            completed_stages,
            stage_engines,
            repo_info,
        )?;

        // Wait for user input and return action
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
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
