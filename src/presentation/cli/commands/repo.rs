use crate::presentation::cli::commands::run_game_session;
use crate::presentation::cli::screen_runner::run_screen;
use crate::presentation::cli::Cli;
use crate::presentation::tui::screens::{RepoListScreen, RepoPlayScreen};
use crate::presentation::tui::ScreenType;
use crate::{GitTypeError, Result};
use std::io::{self, Write};

pub fn run_repo_list() -> Result<()> {
    run_screen(
        ScreenType::RepoList,
        RepoListScreen::new,
        None::<()>,
        None::<fn(&RepoListScreen) -> Option<()>>,
    )?;

    Ok(())
}

pub fn run_repo_clear(force: bool) -> Result<()> {
    use std::fs;

    // Get the repos directory path
    let home_dir = dirs::home_dir().ok_or_else(|| {
        GitTypeError::InvalidRepositoryFormat("Could not determine home directory".to_string())
    })?;
    let repos_dir = home_dir.join(".gittype").join("repos");

    if !repos_dir.exists() {
        println!("No cached repositories directory found.");
        return Ok(());
    }

    // Count actual repositories (look for directories with .git subdirectories)
    fn count_git_repositories(path: &std::path::Path) -> usize {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // Check if this directory contains a .git subdirectory (actual repo)
                    if entry_path.join(".git").exists() {
                        count += 1;
                    } else {
                        // Recursively check subdirectories
                        count += count_git_repositories(&entry_path);
                    }
                }
            }
        }
        count
    }

    let cached_count = count_git_repositories(&repos_dir);

    if cached_count == 0 {
        println!("No cached repositories found.");
        return Ok(());
    }

    if !force {
        println!("This will delete all locally cached repositories in:");
        println!("  {}", repos_dir.display());
        println!("({} cached repositories will be removed)", cached_count);

        print!("Are you sure you want to continue? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    // Delete the entire repos directory
    match fs::remove_dir_all(&repos_dir) {
        Ok(_) => {
            println!("Successfully deleted all cached repositories.");
            println!("Cache directory {} has been removed.", repos_dir.display());
        }
        Err(e) => {
            return Err(GitTypeError::InvalidRepositoryFormat(format!(
                "Failed to delete cache directory: {}",
                e
            )));
        }
    }

    Ok(())
}

pub fn run_repo_play() -> Result<()> {
    // Run screen and get selected repository
    let selected_repo = run_screen(
        ScreenType::RepoPlay,
        RepoPlayScreen::new,
        None::<()>,
        Some(|screen: &RepoPlayScreen| {
            screen
                .get_selected_repository()
                .map(|(repo, _)| (repo.user_name.clone(), repo.repository_name.clone()))
        }),
    )?;

    // If a repository was selected, start the game
    if let Some((user_name, repo_name)) = selected_repo {
        let repo_spec = format!("{}/{}", user_name, repo_name);

        println!("Starting gittype with repository: {}", repo_spec);

        // Create a Cli struct to pass to run_game_session
        let cli = Cli {
            repo_path: None,
            repo: Some(repo_spec),
            langs: None,
            command: None,
        };

        // Start the game session
        run_game_session(cli)
    } else {
        println!("Repository selection cancelled.");
        Ok(())
    }
}
