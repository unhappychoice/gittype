use crate::game::challenge::Challenge;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct CountdownScreen;

impl CountdownScreen {
    pub fn show() -> Result<()> {
        Self::show_with_challenge(None)
    }

    pub fn show_with_challenge(challenge: Option<&Challenge>) -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Show source info if available
        if let Some(challenge) = challenge {
            if let Some(ref path) = challenge.source_file_path {
                let source_msg =
                    if let (Some(start), Some(end)) = (challenge.start_line, challenge.end_line) {
                        format!("Source: {}:{}-{}", path, start, end)
                    } else {
                        format!("Source: {}", path)
                    };
                let source_col = center_col.saturating_sub(source_msg.len() as u16 / 2);
                execute!(stdout, MoveTo(source_col, center_row - 4))?;
                execute!(stdout, SetForegroundColor(Color::Cyan))?;
                execute!(stdout, Print(&source_msg))?;
                execute!(stdout, ResetColor)?;
            }
        }

        // Show "Get Ready!" message
        let ready_msg = "Get Ready!";
        let ready_col = center_col.saturating_sub(ready_msg.len() as u16 / 2);
        execute!(stdout, terminal::Clear(ClearType::All))?;

        // Show source again after clear
        if let Some(challenge) = challenge {
            if let Some(ref path) = challenge.source_file_path {
                let source_msg =
                    if let (Some(start), Some(end)) = (challenge.start_line, challenge.end_line) {
                        format!("Source: {}:{}-{}", path, start, end)
                    } else {
                        format!("Source: {}", path)
                    };
                let source_col = center_col.saturating_sub(source_msg.len() as u16 / 2);
                execute!(stdout, MoveTo(source_col, center_row - 4))?;
                execute!(stdout, SetForegroundColor(Color::Cyan))?;
                execute!(stdout, Print(&source_msg))?;
                execute!(stdout, ResetColor)?;
            }
        }

        execute!(stdout, MoveTo(ready_col, center_row - 2))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Yellow)
        )?;
        execute!(stdout, Print(ready_msg))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        Self::clear_input_buffer_and_wait(600)?;

        // Countdown from 3 to 1
        for count in (1..=3).rev() {
            execute!(stdout, terminal::Clear(ClearType::All))?;

            // Show source info if available
            if let Some(challenge) = challenge {
                if let Some(ref path) = challenge.source_file_path {
                    let source_msg = if let (Some(start), Some(end)) =
                        (challenge.start_line, challenge.end_line)
                    {
                        format!("Source: {}:{}-{}", path, start, end)
                    } else {
                        format!("Source: {}", path)
                    };
                    let source_col = center_col.saturating_sub(source_msg.len() as u16 / 2);
                    execute!(stdout, MoveTo(source_col, center_row - 4))?;
                    execute!(stdout, SetForegroundColor(Color::Cyan))?;
                    execute!(stdout, Print(&source_msg))?;
                    execute!(stdout, ResetColor)?;
                }
            }

            // Show "Get Ready!" message
            execute!(stdout, MoveTo(ready_col, center_row - 2))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Yellow)
            )?;
            execute!(stdout, Print(ready_msg))?;
            execute!(stdout, ResetColor)?;

            // Show countdown number
            let count_str = count.to_string();
            let count_col = center_col.saturating_sub(count_str.len() as u16 / 2);
            execute!(stdout, MoveTo(count_col, center_row))?;
            execute!(stdout, SetAttribute(Attribute::Bold))?;

            // Different colors for each number
            match count {
                3 => execute!(stdout, SetForegroundColor(Color::Red))?,
                2 => execute!(stdout, SetForegroundColor(Color::Yellow))?,
                1 => execute!(stdout, SetForegroundColor(Color::Green))?,
                _ => execute!(stdout, SetForegroundColor(Color::White))?,
            }

            execute!(stdout, Print(&count_str))?;
            execute!(stdout, ResetColor)?;
            stdout.flush()?;

            Self::clear_input_buffer_and_wait(600)?;
        }

        // Show "GO!" message
        execute!(stdout, terminal::Clear(ClearType::All))?;
        let go_msg = "GO!";
        let go_col = center_col.saturating_sub(go_msg.len() as u16 / 2);
        execute!(stdout, MoveTo(go_col, center_row))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Green)
        )?;
        execute!(stdout, Print(go_msg))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        Self::clear_input_buffer_and_wait(400)?;

        Ok(())
    }

    pub fn show_stage_transition(stage_number: usize, total_stages: usize) -> Result<()> {
        Self::show_stage_transition_with_challenge(stage_number, total_stages, None)
    }

    pub fn show_stage_transition_with_challenge(
        stage_number: usize,
        total_stages: usize,
        challenge: Option<&Challenge>,
    ) -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Show source info if available
        if let Some(challenge) = challenge {
            if let Some(ref path) = challenge.source_file_path {
                let source_msg =
                    if let (Some(start), Some(end)) = (challenge.start_line, challenge.end_line) {
                        format!("Source: {}:{}-{}", path, start, end)
                    } else {
                        format!("Source: {}", path)
                    };
                let source_col = center_col.saturating_sub(source_msg.len() as u16 / 2);
                execute!(stdout, MoveTo(source_col, center_row - 4))?;
                execute!(stdout, SetForegroundColor(Color::Cyan))?;
                execute!(stdout, Print(&source_msg))?;
                execute!(stdout, ResetColor)?;
            }
        }

        // Show "Next Stage" message
        let stage_text = format!("Stage {} / {}", stage_number, total_stages);
        let stage_col = center_col.saturating_sub(stage_text.len() as u16 / 2);
        execute!(stdout, terminal::Clear(ClearType::All))?;

        // Show source again after clear
        if let Some(challenge) = challenge {
            if let Some(ref path) = challenge.source_file_path {
                let source_msg =
                    if let (Some(start), Some(end)) = (challenge.start_line, challenge.end_line) {
                        format!("Source: {}:{}-{}", path, start, end)
                    } else {
                        format!("Source: {}", path)
                    };
                let source_col = center_col.saturating_sub(source_msg.len() as u16 / 2);
                execute!(stdout, MoveTo(source_col, center_row - 4))?;
                execute!(stdout, SetForegroundColor(Color::Cyan))?;
                execute!(stdout, Print(&source_msg))?;
                execute!(stdout, ResetColor)?;
            }
        }

        execute!(stdout, MoveTo(stage_col, center_row - 2))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(&stage_text))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        Self::clear_input_buffer_and_wait(600)?;

        // Countdown from 3 to 1
        for count in (1..=3).rev() {
            execute!(stdout, terminal::Clear(ClearType::All))?;

            // Show source info if available
            if let Some(challenge) = challenge {
                if let Some(ref path) = challenge.source_file_path {
                    let source_msg = if let (Some(start), Some(end)) =
                        (challenge.start_line, challenge.end_line)
                    {
                        format!("Source: {}:{}-{}", path, start, end)
                    } else {
                        format!("Source: {}", path)
                    };
                    let source_col = center_col.saturating_sub(source_msg.len() as u16 / 2);
                    execute!(stdout, MoveTo(source_col, center_row - 4))?;
                    execute!(stdout, SetForegroundColor(Color::Cyan))?;
                    execute!(stdout, Print(&source_msg))?;
                    execute!(stdout, ResetColor)?;
                }
            }

            // Show stage number
            execute!(stdout, MoveTo(stage_col, center_row - 2))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Cyan)
            )?;
            execute!(stdout, Print(&stage_text))?;
            execute!(stdout, ResetColor)?;

            // Show countdown number
            let count_str = count.to_string();
            let count_col = center_col.saturating_sub(count_str.len() as u16 / 2);
            execute!(stdout, MoveTo(count_col, center_row))?;
            execute!(stdout, SetAttribute(Attribute::Bold))?;

            // Different colors for each number
            match count {
                3 => execute!(stdout, SetForegroundColor(Color::Red))?,
                2 => execute!(stdout, SetForegroundColor(Color::Yellow))?,
                1 => execute!(stdout, SetForegroundColor(Color::Green))?,
                _ => execute!(stdout, SetForegroundColor(Color::White))?,
            }

            execute!(stdout, Print(&count_str))?;
            execute!(stdout, ResetColor)?;
            stdout.flush()?;

            Self::clear_input_buffer_and_wait(600)?;
        }

        // Show "START!" message
        execute!(stdout, terminal::Clear(ClearType::All))?;
        let start_msg = "START!";
        let start_col = center_col.saturating_sub(start_msg.len() as u16 / 2);
        execute!(stdout, MoveTo(start_col, center_row))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Green)
        )?;
        execute!(stdout, Print(start_msg))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        Self::clear_input_buffer_and_wait(400)?;

        Ok(())
    }

    fn clear_input_buffer_and_wait(duration_ms: u64) -> Result<()> {
        let end_time = std::time::Instant::now() + std::time::Duration::from_millis(duration_ms);

        while std::time::Instant::now() < end_time {
            if event::poll(std::time::Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        // Use global session tracker to show summary
                        crate::game::stage_manager::show_session_summary_on_interrupt();
                        std::process::exit(0);
                    }
                }
            }
        }
        Ok(())
    }
}
