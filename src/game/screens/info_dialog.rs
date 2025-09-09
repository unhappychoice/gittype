use crate::game::screen_manager::{Screen, ScreenTransition, UpdateStrategy};
use std::io::Stdout;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};

pub enum InfoAction {
    OpenGithub,
    OpenX,
    Close,
}

pub struct InfoDialog;

impl InfoDialog {
    pub fn show() -> Result<InfoAction> {
        let mut selected_option = 0;
        let options = [
            ("GitHub Repository", InfoAction::OpenGithub),
            ("X #gittype", InfoAction::OpenX),
            ("Close", InfoAction::Close),
        ];

        let mut stdout = stdout();
        let (terminal_width, terminal_height) = terminal::size()?;
        let center_row = terminal_height / 2;
        let center_col = terminal_width / 2;

        // Draw dialog box
        Self::draw_dialog_box(&mut stdout, center_row, center_col)?;
        Self::draw_dialog_content(
            &mut stdout,
            center_row,
            center_col,
            &options,
            selected_option,
        )?;
        stdout.flush()?;

        loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(50)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    match key_event.code {
                        KeyCode::Char(' ') => {
                            return Ok(match selected_option {
                                0 => InfoAction::OpenGithub,
                                1 => InfoAction::OpenX,
                                _ => InfoAction::Close,
                            });
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            selected_option = if selected_option == 0 {
                                options.len() - 1
                            } else {
                                selected_option - 1
                            };
                            Self::draw_dialog_content(
                                &mut stdout,
                                center_row,
                                center_col,
                                &options,
                                selected_option,
                            )?;
                            stdout.flush()?;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            selected_option = (selected_option + 1) % options.len();
                            Self::draw_dialog_content(
                                &mut stdout,
                                center_row,
                                center_col,
                                &options,
                                selected_option,
                            )?;
                            stdout.flush()?;
                        }
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(InfoAction::Close),
                        KeyCode::Char('g') => return Ok(InfoAction::OpenGithub),
                        KeyCode::Char('x') => return Ok(InfoAction::OpenX),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw_dialog_box(
        stdout: &mut std::io::Stdout,
        center_row: u16,
        center_col: u16,
    ) -> Result<()> {
        let dialog_width = 50;
        let dialog_height = 10;
        let start_col = center_col.saturating_sub(dialog_width / 2);
        let start_row = center_row.saturating_sub(dialog_height / 2);

        // Draw dialog background
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

        Ok(())
    }

    fn draw_dialog_content(
        stdout: &mut std::io::Stdout,
        center_row: u16,
        center_col: u16,
        options: &[(&str, InfoAction); 3],
        selected_option: usize,
    ) -> Result<()> {
        let start_row = center_row.saturating_sub(4);

        // Title
        let title = "Information & Links";
        let title_col = center_col.saturating_sub(title.len() as u16 / 2);
        execute!(stdout, MoveTo(title_col, start_row + 1))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Cyan)
        )?;
        execute!(stdout, Print(title))?;
        execute!(stdout, ResetColor)?;

        // Options
        for (i, (label, _)) in options.iter().enumerate() {
            let option_col = center_col.saturating_sub(label.len() as u16 / 2 + 2);
            execute!(stdout, MoveTo(option_col, start_row + 3 + i as u16))?;

            if i == selected_option {
                execute!(
                    stdout,
                    SetAttribute(Attribute::Bold),
                    SetForegroundColor(Color::Yellow)
                )?;
                execute!(stdout, Print("> "))?;
                execute!(stdout, SetForegroundColor(Color::White))?;
                execute!(stdout, Print(label))?;
            } else {
                execute!(stdout, SetForegroundColor(Color::Grey))?;
                execute!(stdout, Print("  "))?;
                execute!(stdout, Print(label))?;
            }
            execute!(stdout, ResetColor)?;
        }

        // Instructions with color coding
        let total_instructions_len = "[↑↓/JK] Navigate [SPACE] Select [ESC] Close".len();
        let instructions_col = center_col.saturating_sub(total_instructions_len as u16 / 2);
        execute!(stdout, MoveTo(instructions_col, start_row + 7))?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        execute!(stdout, Print("[↑↓/JK]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Navigate "))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        execute!(stdout, Print("[SPACE]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Select "))?;
        execute!(stdout, SetForegroundColor(Color::Red))?;
        execute!(stdout, Print("[ESC]"))?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        execute!(stdout, Print(" Close"))?;
        execute!(stdout, ResetColor)?;

        Ok(())
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

// Basic Screen trait implementation for ScreenManager compatibility
pub struct ScreenState {
    should_exit: bool,
}

impl ScreenState {
    pub fn new() -> Self {
        Self { should_exit: false }
    }
}

impl Screen for ScreenState {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> crate::Result<ScreenTransition> {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key_event.code {
            KeyCode::Esc => {
                self.should_exit = true;
                Ok(ScreenTransition::None)
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_exit = true;
                Ok(ScreenTransition::Exit)
            }
            _ => Ok(ScreenTransition::None),
        }
    }

    fn render_crossterm(&self, _stdout: &mut Stdout) -> crate::Result<()> {
        let _ = InfoDialog::show();
        Ok(())
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        UpdateStrategy::InputOnly
    }

    fn update(&mut self) -> crate::Result<bool> {
        Ok(false)
    }
}
