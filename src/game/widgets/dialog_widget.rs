use crate::game::utils::{MenuSelector, TerminalUtils};
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{Stdout, Write};

/// Centered dialog widget with menu options
pub struct DialogWidget<'a, T> {
    title: &'a str,
    options: &'a [(&'a str, T)],
    width: u16,
    height: u16,
    border_char: char,
    title_color: Color,
    selected_color: Color,
    normal_color: Color,
}

impl<'a, T: Clone> DialogWidget<'a, T> {
    pub fn new(title: &'a str, options: &'a [(&'a str, T)]) -> Self {
        Self {
            title,
            options,
            width: 50,
            height: 10,
            border_char: '─',
            title_color: Color::Cyan,
            selected_color: Color::Yellow,
            normal_color: Color::White,
        }
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn border_char(mut self, ch: char) -> Self {
        self.border_char = ch;
        self
    }

    pub fn colors(mut self, title: Color, selected: Color, normal: Color) -> Self {
        self.title_color = title;
        self.selected_color = selected;
        self.normal_color = normal;
        self
    }

    /// Draw the dialog box border
    pub fn draw_border(&self, stdout: &mut Stdout) -> Result<()> {
        let (center_col, center_row) = TerminalUtils::center_position(self.width, self.height)?;
        let start_col = center_col;
        let start_row = center_row;

        // Draw top border
        execute!(stdout, MoveTo(start_col, start_row))?;
        execute!(stdout, SetForegroundColor(self.title_color))?;
        for _ in 0..self.width {
            execute!(stdout, Print(self.border_char))?;
        }

        // Draw sides
        for i in 1..self.height - 1 {
            execute!(stdout, MoveTo(start_col, start_row + i))?;
            execute!(stdout, Print('│'))?;
            execute!(stdout, MoveTo(start_col + self.width - 1, start_row + i))?;
            execute!(stdout, Print('│'))?;
        }

        // Draw bottom border
        execute!(stdout, MoveTo(start_col, start_row + self.height - 1))?;
        for _ in 0..self.width {
            execute!(stdout, Print(self.border_char))?;
        }

        execute!(stdout, ResetColor)?;
        Ok(())
    }

    /// Draw the dialog content with menu options
    pub fn draw_content(&self, stdout: &mut Stdout, selector: &MenuSelector) -> Result<()> {
        let (center_col, center_row) = TerminalUtils::center_position(self.width, self.height)?;
        let start_col = center_col;
        let start_row = center_row;

        // Draw title
        execute!(stdout, MoveTo(start_col + 2, start_row + 1))?;
        execute!(stdout, SetForegroundColor(self.title_color))?;
        execute!(stdout, Print(self.title))?;

        // Draw options
        for (i, (option_text, _)) in self.options.iter().enumerate() {
            let y_pos = start_row + 3 + i as u16;
            execute!(stdout, MoveTo(start_col + 4, y_pos))?;

            if i == selector.selected() {
                execute!(stdout, SetForegroundColor(self.selected_color))?;
                execute!(stdout, Print(format!("> {}", option_text)))?;
            } else {
                execute!(stdout, SetForegroundColor(self.normal_color))?;
                execute!(stdout, Print(format!("  {}", option_text)))?;
            }
        }

        execute!(stdout, ResetColor)?;
        stdout.flush()?;
        Ok(())
    }

    /// Draw control instructions at the bottom of the dialog
    pub fn draw_controls(&self, stdout: &mut Stdout) -> Result<()> {
        let (center_col, center_row) = TerminalUtils::center_position(self.width, self.height)?;
        let start_row = center_row;

        // Instructions with color coding
        let instructions_row = start_row + self.height - 2;
        let instructions_text = "[↑↓/JK] Navigate [SPACE] Select [ESC] Close";
        let instructions_col =
            center_col + (self.width / 2).saturating_sub(instructions_text.len() as u16 / 2);

        execute!(stdout, MoveTo(instructions_col, instructions_row))?;
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

        stdout.flush()?;
        Ok(())
    }

    /// Get the action for the currently selected option
    pub fn get_selected_action(&self, selector: &MenuSelector) -> Option<T> {
        self.options
            .get(selector.selected())
            .map(|(_, action)| action.clone())
    }
}
