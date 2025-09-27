use crate::domain::models::GitRepository;
use crate::ui::Colors;
use crate::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal,
};
use std::io::Stdout;

pub struct GitRepositoryView;

impl GitRepositoryView {
    pub fn draw(stdout: &mut Stdout, git_repository: Option<&GitRepository>) -> Result<()> {
        if let Some(info) = git_repository {
            let (terminal_width, terminal_height) = terminal::size()?;
            let bottom_row = terminal_height - 1;

            // Build git info string
            let mut parts = vec![format!("📁 {}/{}", info.user_name, info.repository_name)];

            if let Some(ref branch) = info.branch {
                parts.push(format!("🌿 {}", branch));
            }

            if let Some(ref commit) = info.commit_hash {
                parts.push(format!("📝 {}", &commit[..8]));
            }

            let status_symbol = if info.is_dirty { "⚠️" } else { "✓" };
            parts.push(status_symbol.to_string());

            let git_text = parts.join(" • ");

            // Calculate approximate display width considering emoji width
            // Each emoji takes about 2 characters worth of width
            let emoji_count = git_text.chars().filter(|c| *c as u32 > 127).count();
            let approximate_width = git_text.chars().count() + emoji_count;

            // Center the text using approximate width
            let git_col = terminal_width.saturating_sub(approximate_width as u16) / 2;

            execute!(stdout, MoveTo(git_col, bottom_row))?;
            execute!(
                stdout,
                SetForegroundColor(Colors::to_crossterm(Colors::text_secondary()))
            )?;
            execute!(stdout, Print(&git_text))?;
            execute!(stdout, ResetColor)?;
        }
        Ok(())
    }
}
