use crate::domain::models::GitRepository;
use crate::presentation::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};

pub struct PreviewView;

impl PreviewView {
    pub fn render(
        metrics: &crate::domain::models::SessionResult,
        repo_info: &Option<GitRepository>,
        center_col: u16,
        center_row: u16,
    ) -> Result<()> {
        let mut stdout = stdout();

        let best_rank = crate::domain::services::scoring::Rank::for_score(metrics.session_score);
        let preview_text = if let Some(repo) = repo_info {
            format!(
                "\"{}\" with {:.0}pts on [{}/{}] - CPM: {:.0}, Mistakes: {}",
                best_rank.name(),
                metrics.session_score,
                repo.user_name,
                repo.repository_name,
                metrics.overall_cpm,
                metrics.valid_mistakes + metrics.invalid_mistakes
            )
        } else {
            format!(
                "\"{}\" with {:.0}pts - CPM: {:.0}, Mistakes: {}",
                best_rank.name(),
                metrics.session_score,
                metrics.overall_cpm,
                metrics.valid_mistakes + metrics.invalid_mistakes
            )
        };
        let preview_col = center_col.saturating_sub(preview_text.len() as u16 / 2);
        execute!(stdout, MoveTo(preview_col, center_row.saturating_sub(5)))?;

        // Display with individual colors
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print("\""))?;
        execute!(stdout, SetForegroundColor(best_rank.terminal_color()))?;
        execute!(stdout, Print(best_rank.name()))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print("\" with "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::score()))
        )?;
        execute!(stdout, Print(format!("{:.0}pts", metrics.session_score)))?;

        if let Some(repo) = repo_info {
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
            execute!(stdout, Print(" on ["))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::info()))
            )?;
            execute!(
                stdout,
                Print(format!("{}/{}", repo.user_name, repo.repository_name))
            )?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text()))
            )?;
            execute!(stdout, Print("]"))?;
        }

        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(" - "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::cpm_wpm()))
        )?;
        execute!(stdout, Print("CPM: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(stdout, Print(format!("{:.0}", metrics.overall_cpm)))?;
        execute!(stdout, Print(", "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::error()))
        )?;
        execute!(stdout, Print("Mistakes: "))?;
        execute!(
            stdout,
            SetForegroundColor(Colors::to_crossterm(Colors::text()))
        )?;
        execute!(
            stdout,
            Print(format!(
                "{}",
                metrics.valid_mistakes + metrics.invalid_mistakes
            ))
        )?;
        execute!(stdout, ResetColor)?;

        stdout.flush()?;
        Ok(())
    }
}
