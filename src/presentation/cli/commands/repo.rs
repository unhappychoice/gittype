use crate::infrastructure::console::{Console, ConsoleImpl};
use crate::infrastructure::storage::app_data_provider::AppDataProvider;
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::infrastructure::storage::file_storage::FileStorageInterface;
use crate::presentation::cli::commands::run_game_session;
use crate::presentation::cli::screen_runner::run_screen;
use crate::presentation::cli::Cli;
use crate::presentation::tui::screens::{RepoListScreen, RepoPlayScreen};
use crate::presentation::tui::ScreenType;
use crate::{GitTypeError, Result};
use std::sync::Arc;

pub fn run_repo_list() -> Result<()> {
    use crate::domain::services::theme_service::ThemeServiceInterface;
    use crate::presentation::di::AppModule;
    use shaku::HasComponent;

    let container = AppModule::builder().build();
    let theme_service: Arc<dyn ThemeServiceInterface> = container.resolve();

    run_screen(
        ScreenType::RepoList,
        |event_bus| RepoListScreen::new(event_bus, Arc::clone(&theme_service)),
        None::<()>,
        None::<fn(&RepoListScreen) -> Option<()>>,
    )?;

    Ok(())
}

struct RepoClearCommand;
impl AppDataProvider for RepoClearCommand {}

pub fn run_repo_clear(force: bool) -> Result<()> {
    let file_storage = FileStorage::new();
    let console = ConsoleImpl::new();

    // Get the repos directory path
    let repos_dir = RepoClearCommand::get_app_data_dir()
        .map_err(|_| {
            GitTypeError::InvalidRepositoryFormat(
                "Could not determine app data directory".to_string(),
            )
        })?
        .join("repos");

    if !file_storage.file_exists(&repos_dir) {
        console.println("No cached repositories directory found.")?;
        return Ok(());
    }

    // Count actual repositories (look for directories with .git subdirectories)
    fn count_git_repositories(file_storage: &FileStorage, path: &std::path::Path) -> Result<usize> {
        let mut count = 0;
        let entries = file_storage.read_dir(path)?;
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                // Check if this directory contains a .git subdirectory (actual repo)
                if file_storage.file_exists(&entry_path.join(".git")) {
                    count += 1;
                } else {
                    // Recursively check subdirectories
                    count += count_git_repositories(file_storage, &entry_path)?;
                }
            }
        }
        Ok(count)
    }

    let cached_count = count_git_repositories(&file_storage, &repos_dir)?;

    if cached_count == 0 {
        console.println("No cached repositories found.")?;
        return Ok(());
    }

    if !force {
        console.println("This will delete all locally cached repositories in:")?;
        console.println(&format!("  {}", repos_dir.display()))?;
        console.println(&format!(
            "({} cached repositories will be removed)",
            cached_count
        ))?;

        console.print("Are you sure you want to continue? [y/N]: ")?;
        console.flush()?;

        let mut input = String::new();
        console.read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            console.println("Operation cancelled.")?;
            return Ok(());
        }
    }

    // Delete the entire repos directory
    match file_storage.remove_dir_all(&repos_dir) {
        Ok(_) => {
            console.println("Successfully deleted all cached repositories.")?;
            console.println(&format!(
                "Cache directory {} has been removed.",
                repos_dir.display()
            ))?;
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
    use crate::domain::services::theme_service::ThemeServiceInterface;
    use crate::presentation::di::AppModule;
    use shaku::HasComponent;

    let console = ConsoleImpl::new();
    let container = AppModule::builder().build();
    let theme_service: Arc<dyn ThemeServiceInterface> = container.resolve();

    // Run screen and get selected repository
    let selected_repo = run_screen(
        ScreenType::RepoPlay,
        |event_bus| RepoPlayScreen::new(event_bus, Arc::clone(&theme_service)),
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

        console.println(&format!("Starting gittype with repository: {}", repo_spec))?;

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
        console.println("Repository selection cancelled.")?;
        Ok(())
    }
}
