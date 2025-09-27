use crate::game::ascii_digits::get_digit_patterns;
use crate::scoring::StageResult;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct StageCompletionView;

impl StageCompletionView {
    pub fn render_complete(
        metrics: &StageResult,
        current_stage: usize,
        total_stages: usize,
        has_next_stage: bool,
        keystrokes: usize,
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

        // Render stage title
        Self::render_stage_title(&mut stdout, metrics, current_stage, center_col, center_row)?;

        // Render score section
        let ascii_height =
            Self::render_score_section(&mut stdout, metrics, center_col, center_row)?;
        let score_start_row = center_row.saturating_sub(3) + 1;

        // Display metrics only for completed (non-failed, non-skipped) challenges
        if !metrics.was_failed && !metrics.was_skipped {
            let metrics_row = score_start_row + ascii_height + 1;
            Self::render_metrics(&mut stdout, metrics, keystrokes, center_col, metrics_row)?;
        }

        // Show progress indicator after metrics or ASCII score
        let progress_start_row = if !metrics.was_failed && !metrics.was_skipped {
            score_start_row + ascii_height + 3 // ASCII + gap + metrics lines + gap
        } else {
            score_start_row + ascii_height + 1 // ASCII + gap
        };

        let options_row = Self::render_progress_indicator(
            &mut stdout,
            current_stage,
            total_stages,
            has_next_stage,
            center_col,
            progress_start_row,
        )?;

        // Show stage completion options with color coding
        Self::render_options(&mut stdout, center_col, options_row)?;

        stdout.flush()?;

        Ok(())
    }

    fn create_ascii_numbers(score: &str) -> Vec<String> {
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

    fn render_stage_title(
        stdout: &mut std::io::Stdout,
        metrics: &StageResult,
        current_stage: usize,
        center_col: u16,
        center_row: u16,
    ) -> Result<()> {
        let stage_title = if metrics.was_failed {
            format!("=== STAGE {} FAILED ===", current_stage)
        } else if metrics.was_skipped {
            format!("=== STAGE {} SKIPPED ===", current_stage)
        } else {
            format!("=== STAGE {} COMPLETE ===", current_stage)
        };

        let title_col = center_col.saturating_sub(stage_title.len() as u16 / 2);

        execute!(stdout, MoveTo(title_col, center_row.saturating_sub(6)))?;
        execute!(stdout, SetAttribute(Attribute::Bold))?;
        if metrics.was_failed {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::error()))
            )?;
        } else if metrics.was_skipped {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::warning()))
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::success()))
            )?;
        }
        execute!(stdout, Print(&stage_title))?;
        execute!(stdout, ResetColor)?;

        Ok(())
    }

    fn render_score_section(
        stdout: &mut std::io::Stdout,
        metrics: &StageResult,
        center_col: u16,
        center_row: u16,
    ) -> Result<u16> {
        let score_label_row = center_row.saturating_sub(3);

        // Display score label
        let score_label = if metrics.was_failed {
            "FAILED AFTER"
        } else if metrics.was_skipped {
            "SKIPPED"
        } else {
            "SCORE"
        };

        let score_label_col = center_col.saturating_sub(score_label.len() as u16 / 2);
        execute!(stdout, MoveTo(score_label_col, score_label_row))?;
        execute!(stdout, SetAttribute(Attribute::Bold))?;
        if metrics.was_failed {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::error()))
            )?;
        } else if metrics.was_skipped {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::error()))
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::success()))
            )?;
        }
        execute!(stdout, Print(score_label))?;
        execute!(stdout, ResetColor)?;

        // Display large ASCII art score
        let score_value = if metrics.was_failed {
            format!("{:.1}", metrics.completion_time.as_secs_f64())
        } else if metrics.was_skipped {
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
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::error()))
                )?;
            } else if metrics.was_skipped {
                execute!(
                    stdout,
                    SetForegroundColor(Colors::to_crossterm(Colors::warning()))
                )?;
            } else {
                let best_rank = crate::scoring::Rank::for_score(metrics.challenge_score);
                execute!(stdout, SetForegroundColor(best_rank.terminal_color()))?;
            }
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        Ok(ascii_numbers.len() as u16)
    }

    fn render_metrics(
        stdout: &mut std::io::Stdout,
        metrics: &StageResult,
        keystrokes: usize,
        center_col: u16,
        metrics_row: u16,
    ) -> Result<()> {
        // Line 1: CPM, WPM, Time with colors
        let time_secs = metrics.completion_time.as_secs_f64();
        let line1_text = format!(
            "CPM: {:.0} | WPM: {:.0} | Time: {:.1}s",
            metrics.cpm, metrics.wpm, time_secs
        );
        let line1_col = center_col.saturating_sub(line1_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line1_col, metrics_row))?;

        // CPM label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print("CPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.0}", metrics.cpm)))?;
        execute!(stdout, Print(" | "))?;

        // WPM label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print("WPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.0}", metrics.wpm)))?;
        execute!(stdout, Print(" | "))?;

        // Time label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::duration()))
        )?;
        execute!(stdout, Print("Time: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.1}s", time_secs)))?;
        execute!(stdout, ResetColor)?;

        // Line 2: Keystrokes, Mistakes, Accuracy with colors
        let line2_text = format!(
            "Keystrokes: {} | Mistakes: {} | Accuracy: {:.1}%",
            keystrokes, metrics.mistakes, metrics.accuracy
        );
        let line2_col = center_col.saturating_sub(line2_text.len() as u16 / 2);
        execute!(stdout, MoveTo(line2_col, metrics_row + 1))?;

        // Keystrokes label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::stage_info()))
        )?;
        execute!(stdout, Print("Keystrokes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{}", keystrokes)))?;
        execute!(stdout, Print(" | "))?;

        // Mistakes label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::error()))
        )?;
        execute!(stdout, Print("Mistakes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{}", metrics.mistakes)))?;
        execute!(stdout, Print(" | "))?;

        // Accuracy label and value
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::accuracy()))
        )?;
        execute!(stdout, Print("Accuracy: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.1}%", metrics.accuracy)))?;
        execute!(stdout, ResetColor)?;

        Ok(())
    }

    fn render_progress_indicator(
        stdout: &mut std::io::Stdout,
        current_stage: usize,
        total_stages: usize,
        has_next_stage: bool,
        center_col: u16,
        progress_start_row: u16,
    ) -> Result<u16> {
        // Progress bar
        let progress_text = format!("Stage {} of {}", current_stage, total_stages);
        let progress_col = center_col.saturating_sub(progress_text.len() as u16 / 2);

        execute!(stdout, MoveTo(progress_col, progress_start_row))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(&progress_text))?;
        execute!(stdout, ResetColor)?;

        let mut next_row = progress_start_row + 1;

        // Show next stage message if there are more stages
        if has_next_stage {
            execute!(stdout, MoveTo(0, progress_start_row + 1))?;
            execute!(stdout, Print(""))?;

            let next_text = "Next stage starting...";
            let next_col = center_col.saturating_sub(next_text.len() as u16 / 2);
            execute!(stdout, MoveTo(next_col, progress_start_row + 2))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::warning()))
            )?;
            execute!(stdout, Print(next_text))?;
            execute!(stdout, ResetColor)?;
            next_row = progress_start_row + 4;
        }

        Ok(next_row)
    }

    fn render_options(
        stdout: &mut std::io::Stdout,
        center_col: u16,
        options_row: u16,
    ) -> Result<()> {
        // Calculate position for centered text
        let total_text_length = "[SPACE] Continue  [ESC] Quit".len();
        let start_col = center_col.saturating_sub(total_text_length as u16 / 2);

        execute!(stdout, MoveTo(start_col, options_row))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::success()))
        )?;
        execute!(stdout, Print("[SPACE]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Continue  "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::error()))
        )?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" Quit"))?;
        execute!(stdout, ResetColor)?;

        Ok(())
    }
}
