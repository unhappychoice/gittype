use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub struct CountdownScreen;

impl CountdownScreen {
    pub fn show() -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Show "Get Ready!" message
        let ready_msg = "Get Ready!";
        let ready_col = center_col.saturating_sub(ready_msg.len() as u16 / 2);
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(ready_col, center_row - 2))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(ready_msg))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(1000));

        // Countdown from 3 to 1
        for count in (1..=3).rev() {
            execute!(stdout, terminal::Clear(ClearType::All))?;
            
            // Show "Get Ready!" message
            execute!(stdout, MoveTo(ready_col, center_row - 2))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Yellow))?;
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

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        // Show "GO!" message
        execute!(stdout, terminal::Clear(ClearType::All))?;
        let go_msg = "GO!";
        let go_col = center_col.saturating_sub(go_msg.len() as u16 / 2);
        execute!(stdout, MoveTo(go_col, center_row))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(go_msg))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(500));

        Ok(())
    }

    pub fn show_stage_transition(stage_number: usize, total_stages: usize) -> Result<()> {
        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Show "Next Stage" message
        let stage_text = format!("Stage {} / {}", stage_number, total_stages);
        let stage_col = center_col.saturating_sub(stage_text.len() as u16 / 2);
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, MoveTo(stage_col, center_row - 2))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print(&stage_text))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(1000));

        // Countdown from 3 to 1
        for count in (1..=3).rev() {
            execute!(stdout, terminal::Clear(ClearType::All))?;
            
            // Show stage number
            execute!(stdout, MoveTo(stage_col, center_row - 2))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
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

            std::thread::sleep(std::time::Duration::from_millis(800));
        }

        // Show "START!" message
        execute!(stdout, terminal::Clear(ClearType::All))?;
        let start_msg = "START!";
        let start_col = center_col.saturating_sub(start_msg.len() as u16 / 2);
        execute!(stdout, MoveTo(start_col, center_row))?;
        execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(start_msg))?;
        execute!(stdout, ResetColor)?;
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(500));

        Ok(())
    }
}