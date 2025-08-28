use crate::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub enum TitleAction {
    Start,
    Quit,
}

pub struct TitleScreen;

impl TitleScreen {
    pub fn show() -> Result<TitleAction> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(ClearType::All))?;
        
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // ASCII logo lines
        let logo_lines = vec![
            "─╔═══╗─╔══╗─╔════╗────╔════╗─╔╗──╔╗─╔═══╗─╔═══╗─",
            "─║╔═╗║─╚╣╠╝─║╔╗╔╗║────║╔╗╔╗║─║╚╗╔╝║─║╔═╗║─║╔══╝─",
            "─║║─╚╝──║║──╚╝║║╚╝────╚╝║║╚╝─╚╗╚╝╔╝─║╚═╝║─║╚══╗─",
            "─║║╔═╗──║║────║║────────║║────╚╗╔╝──║╔══╝─║╔══╝─",
            "─║╚╩═║─╔╣╠╗───║║────────║║─────║║───║║────║╚══╗─",
            "─╚═══╝─╚══╝───╚╝────────╚╝─────╚╝───╚╝────╚═══╝─",
        ];

        // Display ASCII logo
        let logo_width = logo_lines[0].chars().count() as u16;
        let logo_start_col = center_col.saturating_sub(logo_width / 2);
        let logo_start_row = center_row.saturating_sub(8);

        for (i, line) in logo_lines.iter().enumerate() {
            execute!(stdout, MoveTo(logo_start_col, logo_start_row + i as u16))?;
            execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Cyan))?;
            execute!(stdout, Print(line))?;
            execute!(stdout, ResetColor)?;
        }

        // Display subtitle
        let subtitle = "Code Typing Challenge";
        let subtitle_col = center_col.saturating_sub(subtitle.len() as u16 / 2);
        execute!(stdout, MoveTo(subtitle_col, center_row - 1))?;
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        execute!(stdout, Print(subtitle))?;
        execute!(stdout, ResetColor)?;

        // Display instructions
        let start_msg = "[ENTER] Start Challenge";
        let quit_msg = "[ESC] Quit";
        let start_col = center_col.saturating_sub(start_msg.len() as u16 / 2);
        let quit_col = center_col.saturating_sub(quit_msg.len() as u16 / 2);
        
        execute!(stdout, MoveTo(start_col, center_row + 3))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(start_msg))?;
        execute!(stdout, ResetColor)?;

        execute!(stdout, MoveTo(quit_col, center_row + 4))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print(quit_msg))?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;

        // Wait for user input
        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Enter => return Ok(TitleAction::Start),
                        KeyCode::Esc => return Ok(TitleAction::Quit),
                        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(TitleAction::Quit);
                        },
                        _ => continue,
                    }
                }
            }
        }
    }
}