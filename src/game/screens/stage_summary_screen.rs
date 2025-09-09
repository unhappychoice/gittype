use crate::game::ascii_digits::get_digit_patterns;
use crate::game::screen_manager::{Screen, ScreenTransition, UpdateStrategy};
use crate::game::screens::ResultAction;
use crate::scoring::StageResult;
use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct StageSummaryScreen;

impl StageSummaryScreen {
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
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::FAILED))
            )?;
        } else if metrics.was_skipped {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::SKIPPED))
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::COMPLETED))
            )?;
        }
        execute!(stdout, Print(&stage_title))?;
        execute!(stdout, ResetColor)?;

        // Position score label below title
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
                SetForegroundColor(Colors::to_crossterm(Colors::FAILED))
            )?;
        } else if metrics.was_skipped {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::SKIPPED))
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::COMPLETED))
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
                execute!(stdout, SetForegroundColor(Color::Red))?;
            } else if metrics.was_skipped {
                execute!(stdout, SetForegroundColor(Color::Magenta))?;
            } else {
                // Use tier-based color like SessionSummaryScreen
                let best_rank = crate::scoring::Rank::for_score(metrics.challenge_score);
                execute!(stdout, SetForegroundColor(best_rank.terminal_color()))?;
            }
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display metrics only for completed (non-failed, non-skipped) challenges
        if !metrics.was_failed && !metrics.was_skipped {
            // Position metrics after ASCII score
            let ascii_height = ascii_numbers.len() as u16;
            let metrics_row = score_start_row + ascii_height + 1;

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
                SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
            )?;
            execute!(stdout, Print("CPM: "))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
            execute!(stdout, Print(format!("{:.0}", metrics.cpm)))?;
            execute!(stdout, Print(" | "))?;

            // WPM label and value
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::CPM_WPM))
            )?;
            execute!(stdout, Print("WPM: "))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
            execute!(stdout, Print(format!("{:.0}", metrics.wpm)))?;
            execute!(stdout, Print(" | "))?;

            // Time label and value
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::DURATION))
            )?;
            execute!(stdout, Print("Time: "))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
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
                SetForegroundColor(Colors::to_crossterm(Colors::STAGE_INFO))
            )?;
            execute!(stdout, Print("Keystrokes: "))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
            execute!(stdout, Print(format!("{}", keystrokes)))?;
            execute!(stdout, Print(" | "))?;

            // Mistakes label and value
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::ERROR))
            )?;
            execute!(stdout, Print("Mistakes: "))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
            execute!(stdout, Print(format!("{}", metrics.mistakes)))?;
            execute!(stdout, Print(" | "))?;

            // Accuracy label and value
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::ACCURACY))
            )?;
            execute!(stdout, Print("Accuracy: "))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
            )?;
            execute!(stdout, Print(format!("{:.1}%", metrics.accuracy)))?;
            execute!(stdout, ResetColor)?;
        }

        // Show progress indicator after metrics or ASCII score
        let ascii_height = ascii_numbers.len() as u16;
        let progress_start_row = if !metrics.was_failed && !metrics.was_skipped {
            score_start_row + ascii_height + 3 // ASCII + gap + metrics lines + gap
        } else {
            score_start_row + ascii_height + 1 // ASCII + gap
        };

        // Progress bar
        let progress_text = format!("Stage {} of {}", current_stage, total_stages);
        let progress_col = center_col.saturating_sub(progress_text.len() as u16 / 2);

        execute!(stdout, MoveTo(progress_col, progress_start_row))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(&progress_text))?;
        execute!(stdout, ResetColor)?;

        // Show next stage message if there are more stages
        if has_next_stage {
            execute!(stdout, MoveTo(0, progress_start_row + 1))?;
            execute!(stdout, Print(""))?;

            let next_text = "Next stage starting...";
            let next_col = center_col.saturating_sub(next_text.len() as u16 / 2);
            execute!(stdout, MoveTo(next_col, progress_start_row + 2))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::WARNING))
            )?;
            execute!(stdout, Print(next_text))?;
            execute!(stdout, ResetColor)?;
        }

        stdout.flush()?;

        // Show stage completion options with color coding
        let options_row = if has_next_stage {
            progress_start_row + 4
        } else {
            progress_start_row + 1
        };

        // Calculate position for centered text
        let total_text_length = "[SPACE] Continue  [ESC] Quit".len();
        let start_col = center_col.saturating_sub(total_text_length as u16 / 2);

        execute!(stdout, MoveTo(start_col, options_row))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::SUCCESS))
        )?;
        execute!(stdout, Print("[SPACE]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
        execute!(stdout, Print(" Continue  "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::ERROR))
        )?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::TEXT))
        )?;
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
}

pub struct StageSummaryScreenState {
    stage_result: Option<StageResult>,
    action_result: Option<ResultAction>,
    should_exit: bool,
}

impl StageSummaryScreenState {
    fn create_ascii_numbers(score: &str) -> Vec<String> {
        StageSummaryScreen::create_ascii_numbers(score)
    }

    pub fn new() -> Self {
        Self {
            stage_result: None,
            action_result: None,
            should_exit: false,
        }
    }

    pub fn with_result(mut self, result: StageResult) -> Self {
        self.stage_result = Some(result);
        self
    }

    pub fn get_action_result(&self) -> Option<ResultAction> {
        self.action_result.clone()
    }

}

impl Screen for StageSummaryScreenState {
    fn init(&mut self) -> Result<()> {
        self.action_result = None;
        self.should_exit = false;
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        
        match key_event.code {
            KeyCode::Esc => {
                self.action_result = Some(ResultAction::BackToTitle);
                self.should_exit = true;
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.action_result = Some(ResultAction::Quit);
                self.should_exit = true;
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        // Call existing high-quality implementation directly
        if let Some(ref stage_result) = self.stage_result {
            let fake_metrics = crate::scoring::StageResult {
                cpm: stage_result.cpm,
                wpm: stage_result.wpm,
                accuracy: stage_result.accuracy,
                keystrokes: stage_result.keystrokes,
                mistakes: stage_result.mistakes,
                consistency_streaks: stage_result.consistency_streaks.clone(),
                completion_time: stage_result.completion_time,
                challenge_score: stage_result.challenge_score,
                rank_name: stage_result.rank_name.clone(),
                tier_name: stage_result.tier_name.clone(),
                tier_position: stage_result.tier_position,
                tier_total: stage_result.tier_total,
                overall_position: stage_result.overall_position,
                overall_total: stage_result.overall_total,
                was_skipped: stage_result.was_skipped,
                was_failed: stage_result.was_failed,
                challenge_path: stage_result.challenge_path.clone(),
            };
            let _ = StageSummaryScreen::show_stage_completion(&fake_metrics, 1, 1, false, stage_result.keystrokes);
            Ok(())
        } else {
            Ok(())
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> Result<bool> {
        Ok(false)
    }
}
