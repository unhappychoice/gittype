use crate::game::utils::{MenuSelector, TerminalUtils};
use crate::game::widgets::DialogWidget;
use crate::Result;
use crossterm::event::{self, Event, KeyCode};
use std::io::stdout;

#[derive(Debug, Clone)]
pub enum InfoAction {
    OpenGithub,
    OpenX,
    Close,
}

pub struct InfoDialog;

impl InfoDialog {
    pub fn show() -> Result<InfoAction> {
        let options = [
            ("GitHub Repository", InfoAction::OpenGithub),
            ("X #gittype", InfoAction::OpenX),
            ("Close", InfoAction::Close),
        ];

        let mut selector = MenuSelector::new(options.len());
        let mut stdout = stdout();

        // Clear screen
        TerminalUtils::clear_screen(&mut stdout)?;

        // Create dialog widget
        let dialog = DialogWidget::new("GitType Information", &options)
            .width(50)
            .height(10);

        // Draw initial dialog
        dialog.draw_border(&mut stdout)?;
        dialog.draw_content(&mut stdout, &selector)?;
        dialog.draw_controls(&mut stdout)?;

        // Main event loop
        loop {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(' ') => {
                        if let Some(action) = dialog.get_selected_action(&selector) {
                            return Ok(action);
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => return Ok(InfoAction::Close),
                    KeyCode::Char('g') => return Ok(InfoAction::OpenGithub),
                    KeyCode::Char('x') => return Ok(InfoAction::OpenX),
                    _ => {
                        // Handle navigation
                        if selector.handle_key(key.code) {
                            // Redraw content if selection changed
                            dialog.draw_content(&mut stdout, &selector)?;
                            dialog.draw_controls(&mut stdout)?;
                        }
                    }
                }
            }
        }
    }

    pub fn open_github() -> Result<()> {
        let url = "https://github.com/unhappychoice/gittype";
        if open::that(url).is_err() {
            Self::show_url_fallback("GitHub Repository", url)?;
        }
        Ok(())
    }

    pub fn open_x() -> Result<()> {
        let url = "https://x.com/search?q=%23gittype";
        if open::that(url).is_err() {
            Self::show_url_fallback("X Search", url)?;
        }
        Ok(())
    }

    fn show_url_fallback(title: &str, url: &str) -> Result<()> {
        use crossterm::{
            execute,
            terminal::{self, ClearType},
        };
        use std::io::{stdout, Write};

        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Clear screen
        execute!(stdout, terminal::Clear(ClearType::All))?;

        // Draw fallback dialog
        Self::draw_fallback_dialog(&mut stdout, center_row, center_col, title, url)?;
        stdout.flush()?;

        // Wait for ESC key
        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(50)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    if key_event.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn draw_fallback_dialog(
        stdout: &mut std::io::Stdout,
        center_row: u16,
        center_col: u16,
        title: &str,
        url: &str,
    ) -> Result<()> {
        use crossterm::{
            cursor::MoveTo,
            execute,
            style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
        };

        let dialog_width = std::cmp::max(60, url.len() + 4) as u16;
        let dialog_height = 8;
        let start_col = center_col.saturating_sub(dialog_width / 2);
        let start_row = center_row.saturating_sub(dialog_height / 2);

        // Draw dialog box
        for i in 0..dialog_height {
            execute!(stdout, MoveTo(start_col, start_row + i))?;
            execute!(stdout, SetForegroundColor(Color::White))?;
            if i == 0 {
                execute!(stdout, Print("┌"))?;
                execute!(stdout, Print("─".repeat((dialog_width - 2) as usize)))?;
                execute!(stdout, Print("┐"))?;
            } else if i == dialog_height - 1 {
                execute!(stdout, Print("└"))?;
                execute!(stdout, Print("─".repeat((dialog_width - 2) as usize)))?;
                execute!(stdout, Print("┘"))?;
            } else {
                execute!(stdout, Print("│"))?;
                execute!(stdout, Print(" ".repeat((dialog_width - 2) as usize)))?;
                execute!(stdout, Print("│"))?;
            }
            execute!(stdout, ResetColor)?;
        }

        // Title
        let fallback_title = format!("Cannot open {}", title);
        let title_col = center_col.saturating_sub(fallback_title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, start_row + 1))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red)
        )?;
        execute!(stdout, Print(&fallback_title))?;
        execute!(stdout, ResetColor)?;

        // Message
        let message = "Please copy and paste the URL below:";
        let message_col = center_col.saturating_sub(message.len() as u16 / 2);
        execute!(stdout, MoveTo(message_col, start_row + 3))?;
        execute!(stdout, SetForegroundColor(Color::Yellow))?;
        execute!(stdout, Print(message))?;
        execute!(stdout, ResetColor)?;

        // URL
        let url_col = center_col.saturating_sub(url.len() as u16 / 2);
        execute!(stdout, MoveTo(url_col, start_row + 4))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(url))?;
        execute!(stdout, ResetColor)?;

        // Instructions with color coding
        let back_key = "[ESC]";
        let back_label = " Back";
        let total_back_len = back_key.len() + back_label.len();
        let instructions_col = center_col.saturating_sub(total_back_len as u16 / 2);
        execute!(stdout, MoveTo(instructions_col, start_row + 6))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print(back_key))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(back_label))?;
        execute!(stdout, ResetColor)?;

        Ok(())
    }
}
