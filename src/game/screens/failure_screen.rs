use crate::game::screens::session_summary_screen::ResultAction;
use crate::scoring::ScoringEngine;
use crate::{extractor::GitRepositoryInfo, Result};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct FailureScreen;

impl FailureScreen {
    pub fn show_session_summary_fail_mode(
        total_stages: usize,
        completed_stages: usize,
        stage_engines: &[(String, ScoringEngine)],
        _repo_info: &Option<GitRepositoryInfo>,
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

        // Header - show FAILED status (centered)
        let header_text = "=== SESSION FAILED ===";
        let header_x = (terminal_width - header_text.len() as u16) / 2;
        execute!(stdout, MoveTo(header_x, center_y.saturating_sub(6)))?;
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            SetAttribute(Attribute::Bold)
        )?;
        execute!(stdout, Print(header_text))?;
        execute!(stdout, ResetColor)?;

        // Show stage progress (centered, cyan)
        let stage_text = format!("Stages: {}/{}", completed_stages, total_stages);
        let stage_x = (terminal_width - stage_text.len() as u16) / 2;
        execute!(stdout, MoveTo(stage_x, center_y.saturating_sub(2)))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(stage_text))?;

        // Show basic metrics if available (centered, white)
        if !stage_engines.is_empty() {
            let last_engine = &stage_engines.last().unwrap().1;
            let metrics = last_engine
                .calculate_metrics_with_status(false, true)
                .unwrap();

            let metrics_text = format!(
                "CPM: {:.0} | WPM: {:.0} | Accuracy: {:.0}%",
                metrics.cpm, metrics.wpm, metrics.accuracy
            );
            let metrics_x = (terminal_width - metrics_text.len() as u16) / 2;
            execute!(stdout, MoveTo(metrics_x, center_y))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            execute!(stdout, Print(metrics_text))?;
        }

        // Failure message (centered)
        let fail_text = "Challenge failed. Better luck next time!";
        let fail_x = (terminal_width - fail_text.len() as u16) / 2;
        execute!(stdout, MoveTo(fail_x, center_y + 2))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print(fail_text))?;

        // Navigation instructions with color coding
        let full_text_len = "[R] Retry | [T] Back to Title | [ESC] Session Summary & Exit".len();
        let nav_x = (terminal_width - full_text_len as u16) / 2;
        execute!(stdout, MoveTo(nav_x, center_y + 4))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print("[R]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Retry | "))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print("[T]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Back to Title | "))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
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
