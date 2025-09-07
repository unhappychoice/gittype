use crate::game::screens::session_summary_screen::ResultAction;
use crate::scoring::StageTracker;
use crate::ui::Colors;
use crate::{models::GitRepository, Result};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct CancelScreen;

impl CancelScreen {
    pub fn show_session_summary_cancel_mode(
        total_stages: usize,
        completed_stages: usize,
        stage_trackers: &[(String, StageTracker)],
        _repo_info: &Option<GitRepository>,
    ) -> Result<ResultAction> {
        let mut stdout = stdout();

        // Comprehensive screen reset
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Short delay to ensure terminal state is reset
        std::thread::sleep(std::time::Duration::from_millis(10));

        let (terminal_width, terminal_height) = terminal::size()?;
        let center_y = terminal_height / 2;

        // Header - show CANCELLED status (centered)
        let header_text = "=== SESSION CANCELLED ===";
        let header_x = (terminal_width - header_text.len() as u16) / 2;
        execute!(stdout, MoveTo(header_x, center_y.saturating_sub(6)))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::WARNING)),
            SetAttribute(Attribute::Bold)
        )?;
        execute!(stdout, Print(header_text))?;
        execute!(stdout, ResetColor)?;

        // Show stage progress (centered, cyan)
        let stage_text = format!("Stages: {}/{}", completed_stages, total_stages);
        let stage_x = (terminal_width - stage_text.len() as u16) / 2;
        execute!(stdout, MoveTo(stage_x, center_y.saturating_sub(2)))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::INFO))
        )?;
        execute!(stdout, Print(stage_text))?;

        // Show basic metrics if available (centered, white)
        if !stage_trackers.is_empty() {
            let (_last_stage_name, last_tracker) = stage_trackers.last().unwrap();
            let metrics = crate::scoring::StageCalculator::calculate(last_tracker);

            let metrics_text = format!(
                "CPM: {:.0} | WPM: {:.0} | Accuracy: {:.0}%",
                metrics.cpm, metrics.wpm, metrics.accuracy
            );
            let metrics_x = (terminal_width - metrics_text.len() as u16) / 2;
            execute!(stdout, MoveTo(metrics_x, center_y))?;

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
            execute!(stdout, Print(format!("{:.0}%", metrics.accuracy)))?;
        }

        // Cancellation message (centered)
        let cancel_text = "Challenge cancelled. You can retry or go back to title.";
        let cancel_x = (terminal_width - cancel_text.len() as u16) / 2;
        execute!(stdout, MoveTo(cancel_x, center_y + 2))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::WARNING))
        )?;
        execute!(stdout, Print(cancel_text))?;

        // Navigation instructions with color coding
        let full_text_len = "[R] Retry | [T] Back to Title | [ESC] Session Summary & Exit".len();
        let nav_x = (terminal_width - full_text_len as u16) / 2;
        execute!(stdout, MoveTo(nav_x, center_y + 4))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::SUCCESS))
        )?;
        execute!(stdout, Print("[R]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Retry | "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::SUCCESS))
        )?;
        execute!(stdout, Print("[T]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Back to Title | "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::ERROR))
        )?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Session Summary & Exit"))?;

        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        // Wait for user input and return action
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            return Ok(ResultAction::Retry);
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
                            return Ok(ResultAction::Quit);
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
}
