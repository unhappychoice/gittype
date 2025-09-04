use crate::game::ascii_digits;
use crate::models::Challenge;
use crate::{models::GitRepository, Result};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
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
        Self::show_with_challenge_and_repo(challenge, &None)
    }

    pub fn show_with_challenge_and_repo(
        challenge: Option<&Challenge>,
        repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Hide cursor during countdown
        execute!(stdout, Hide)?;

        // Show source and repository info if available
        Self::draw_source_and_repo_info(&mut stdout, center_row, center_col, challenge, repo_info)?;

        // Show "Get Ready!" message
        let ready_msg = "Get Ready!";
        let ready_col = center_col.saturating_sub(ready_msg.len() as u16 / 2);
        execute!(stdout, terminal::Clear(ClearType::All))?;

        // Show source and repository info again after clear
        Self::draw_source_and_repo_info(&mut stdout, center_row, center_col, challenge, repo_info)?;

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

            // Show source and repository info if available
            Self::draw_source_and_repo_info(
                &mut stdout,
                center_row,
                center_col,
                challenge,
                repo_info,
            )?;

            // Show "Get Ready!" message
            execute!(stdout, MoveTo(ready_col, center_row - 2))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Yellow)
            )?;
            execute!(stdout, Print(ready_msg))?;
            execute!(stdout, ResetColor)?;

            // Show countdown number as ASCII art
            let digit_patterns = ascii_digits::get_digit_patterns();
            let pattern = &digit_patterns[count];

            // Different colors for each number - warm to cool progression
            let color = match count {
                3 => Color::Magenta,
                2 => Color::Cyan,
                1 => Color::Yellow,
                _ => Color::White,
            };

            execute!(stdout, SetAttribute(Attribute::Bold))?;
            execute!(stdout, SetForegroundColor(color))?;

            // Display each line of the ASCII art, positioned below stage text
            let ascii_start_row = center_row + 1;
            for (i, line) in pattern.iter().enumerate() {
                let line_col = center_col.saturating_sub(line.len() as u16 / 2);
                execute!(stdout, MoveTo(line_col, ascii_start_row + i as u16))?;
                execute!(stdout, Print(line))?;
            }

            execute!(stdout, ResetColor)?;
            stdout.flush()?;

            Self::clear_input_buffer_and_wait(600)?;
        }

        // Show "GO!" message as ASCII art
        execute!(stdout, terminal::Clear(ClearType::All))?;

        let go_art = vec![
            "   ____  ___  ",
            "  / ___|/ _ \\ ",
            " | |  _| | | |",
            " | |_| | |_| |",
            "  \\____|\\___/ ",
        ];

        execute!(stdout, SetAttribute(Attribute::Bold))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;

        // Display each line of the GO ASCII art
        for (i, line) in go_art.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row - 2 + i as u16))?;
            execute!(stdout, Print(line))?;
        }

        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        Self::clear_input_buffer_and_wait(400)?;

        // Show cursor again
        execute!(stdout, Show)?;

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
        Self::show_stage_transition_with_challenge_and_repo(
            stage_number,
            total_stages,
            challenge,
            &None,
        )
    }

    pub fn show_stage_transition_with_challenge_and_repo(
        stage_number: usize,
        total_stages: usize,
        challenge: Option<&Challenge>,
        repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Hide cursor during countdown
        execute!(stdout, Hide)?;

        // Show source and repository info if available
        Self::draw_source_and_repo_info(&mut stdout, center_row, center_col, challenge, repo_info)?;

        // Show "Next Stage" message
        let stage_text = format!("Stage {} / {}", stage_number, total_stages);
        let stage_col = center_col.saturating_sub(stage_text.len() as u16 / 2);
        execute!(stdout, terminal::Clear(ClearType::All))?;

        // Show source and repository info again after clear
        Self::draw_source_and_repo_info(&mut stdout, center_row, center_col, challenge, repo_info)?;

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

            // Show source and repository info if available
            Self::draw_source_and_repo_info(
                &mut stdout,
                center_row,
                center_col,
                challenge,
                repo_info,
            )?;

            // Show stage number
            execute!(stdout, MoveTo(stage_col, center_row - 2))?;
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                SetForegroundColor(Color::Cyan)
            )?;
            execute!(stdout, Print(&stage_text))?;
            execute!(stdout, ResetColor)?;

            // Show countdown number as ASCII art
            let digit_patterns = ascii_digits::get_digit_patterns();
            let pattern = &digit_patterns[count];

            // Different colors for each number - warm to cool progression
            let color = match count {
                3 => Color::Magenta,
                2 => Color::Cyan,
                1 => Color::Yellow,
                _ => Color::White,
            };

            execute!(stdout, SetAttribute(Attribute::Bold))?;
            execute!(stdout, SetForegroundColor(color))?;

            // Display each line of the ASCII art, positioned below stage text
            let ascii_start_row = center_row + 1;
            for (i, line) in pattern.iter().enumerate() {
                let line_col = center_col.saturating_sub(line.len() as u16 / 2);
                execute!(stdout, MoveTo(line_col, ascii_start_row + i as u16))?;
                execute!(stdout, Print(line))?;
            }

            execute!(stdout, ResetColor)?;
            stdout.flush()?;

            Self::clear_input_buffer_and_wait(600)?;
        }

        // Show "GO!" message as ASCII art
        execute!(stdout, terminal::Clear(ClearType::All))?;

        let go_art = vec![
            "   ____  ___  ",
            "  / ___|/ _ \\ ",
            " | |  _| | | |",
            " | |_| | |_| |",
            "  \\____|\\___/ ",
        ];

        execute!(stdout, SetAttribute(Attribute::Bold))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;

        // Display each line of the GO ASCII art
        for (i, line) in go_art.iter().enumerate() {
            let line_col = center_col.saturating_sub(line.len() as u16 / 2);
            execute!(stdout, MoveTo(line_col, center_row - 2 + i as u16))?;
            execute!(stdout, Print(line))?;
        }

        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        Self::clear_input_buffer_and_wait(400)?;

        // Show cursor again
        execute!(stdout, Show)?;

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

    fn draw_source_and_repo_info(
        stdout: &mut std::io::Stdout,
        center_row: u16,
        center_col: u16,
        challenge: Option<&Challenge>,
        repo_info: &Option<GitRepository>,
    ) -> Result<()> {
        if let Some(challenge) = challenge {
            if challenge.source_file_path.is_some() {
                // Show "Source:" label in Cyan
                let source_label = "Source:";
                let source_label_col = center_col.saturating_sub(source_label.len() as u16 / 2);
                execute!(stdout, MoveTo(source_label_col, center_row - 6))?;
                execute!(stdout, SetForegroundColor(Color::Cyan))?;
                execute!(stdout, Print(source_label))?;
                execute!(stdout, ResetColor)?;

                // Show repository info in brackets if available, in DarkGrey
                if let Some(repo) = repo_info {
                    let repo_msg = format!("[{}/{}]", repo.user_name, repo.repository_name);
                    let repo_col = center_col.saturating_sub(repo_msg.len() as u16 / 2);
                    execute!(stdout, MoveTo(repo_col, center_row - 5))?;
                    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                    execute!(stdout, Print(&repo_msg))?;
                    execute!(stdout, ResetColor)?;
                }

                // Show source file:line info in DarkGrey
                let source_file = challenge.get_display_title();
                let source_col = center_col.saturating_sub(source_file.len() as u16 / 2);
                execute!(stdout, MoveTo(source_col, center_row - 4))?;
                execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
                execute!(stdout, Print(&source_file))?;
                execute!(stdout, ResetColor)?;

                // Add blank line after source info (center_row - 3 is now blank)
            }
        }
        Ok(())
    }
}
