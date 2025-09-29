use crate::domain::models::DifficultyLevel;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::Stdout;

pub struct DifficultySelectionView;

impl DifficultySelectionView {
    pub fn draw(
        stdout: &mut Stdout,
        center_row: u16,
        center_col: u16,
        difficulties: &[(&str, DifficultyLevel); 5],
        selected_difficulty: usize,
        challenge_counts: &[usize; 5],
    ) -> Result<()> {
        let start_row = center_row + 1;
        let (name, difficulty_level) = &difficulties[selected_difficulty];
        let count = challenge_counts[selected_difficulty];

        // Clear previous difficulty display (multiple lines)
        for i in 0..4 {
            execute!(stdout, MoveTo(0, start_row + i))?;
            execute!(stdout, Print(" ".repeat(120)))?;
        }

        // Line 1: Difficulty selection
        let difficulty_text = format!("Difficulty: ← {} →", name);
        let difficulty_col = center_col.saturating_sub(difficulty_text.chars().count() as u16 / 2);

        execute!(stdout, MoveTo(difficulty_col, start_row))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print("Difficulty: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::accuracy()))
        )?;
        execute!(stdout, Print("← "))?;
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(name))?;
        execute!(
            stdout,
            ResetColor,
            SetForegroundColor(Colors::to_crossterm(Colors::accuracy()))
        )?;
        execute!(stdout, Print(" →"))?;
        execute!(stdout, ResetColor)?;

        // Line 2: Challenge count
        let count_text = if count > 0 {
            format!("{} challenges available", count)
        } else {
            "Challenge count will be displayed after loading".to_string()
        };
        let count_col = center_col.saturating_sub(count_text.chars().count() as u16 / 2);

        execute!(stdout, MoveTo(count_col, start_row + 1))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::info())),
            SetAttribute(Attribute::Dim)
        )?;
        execute!(stdout, Print(count_text))?;
        execute!(stdout, ResetColor)?;

        // Line 3 & 4: Description lines
        let descriptions = [difficulty_level.description(), difficulty_level.subtitle()];
        for (i, description) in descriptions.iter().enumerate() {
            let desc_col = center_col.saturating_sub(description.chars().count() as u16 / 2);
            execute!(stdout, MoveTo(desc_col, start_row + 2 + i as u16))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text())),
                SetAttribute(Attribute::Dim)
            )?;
            execute!(stdout, Print(description))?;
            execute!(stdout, ResetColor)?;
        }

        Ok(())
    }
}
