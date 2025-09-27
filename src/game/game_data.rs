use crate::domain::models::ExtractionOptions;
use crate::domain::models::{Challenge, GitRepository};
use crate::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

/// Global game data that persists across screens
#[derive(Debug, Default)]
pub struct GameData {
    pub challenges: Option<Vec<Challenge>>,
    pub git_repository: Option<GitRepository>,
    pub loading_completed: bool,
    pub loading_failed: bool,
    pub error_message: Option<String>,
    // Processing parameters
    pub repo_spec: Option<String>,
    pub repo_path: Option<PathBuf>,
    pub extraction_options: Option<ExtractionOptions>,
}

static GLOBAL_GAME_DATA: OnceLock<Arc<Mutex<GameData>>> = OnceLock::new();

impl GameData {
    /// Initialize the global game data instance
    pub fn initialize() -> Result<()> {
        let game_data = Arc::new(Mutex::new(GameData::default()));
        GLOBAL_GAME_DATA.set(game_data).map_err(|_| {
            crate::GitTypeError::TerminalError("GameData already initialized".to_string())
        })?;
        Ok(())
    }

    /// Get a reference to the global game data instance
    pub fn instance() -> Arc<Mutex<GameData>> {
        GLOBAL_GAME_DATA
            .get()
            .expect("GameData not initialized")
            .clone()
    }

    /// Reset the global game data
    pub fn reset() -> Result<()> {
        if let Some(game_data) = GLOBAL_GAME_DATA.get() {
            let mut data = game_data.lock().unwrap();
            *data = GameData::default();
        }
        Ok(())
    }

    /// Set processing parameters
    pub fn set_processing_parameters(
        repo_spec: Option<&str>,
        repo_path: Option<&PathBuf>,
        extraction_options: &ExtractionOptions,
    ) -> Result<()> {
        let game_data = Self::instance();
        let mut data = game_data.lock().unwrap();
        data.repo_spec = repo_spec.map(|s| s.to_string());
        data.repo_path = repo_path.cloned();
        data.extraction_options = Some(extraction_options.clone());
        Ok(())
    }

    /// Get processing parameters
    pub fn get_processing_parameters(
    ) -> Option<(Option<String>, Option<PathBuf>, ExtractionOptions)> {
        if let Some(game_data) = GLOBAL_GAME_DATA.get() {
            let data = game_data.lock().unwrap();
            data.extraction_options.as_ref().map(|options| {
                (
                    data.repo_spec.clone(),
                    data.repo_path.clone(),
                    options.clone(),
                )
            })
        } else {
            None
        }
    }

    /// Set the processing results
    pub fn set_results(
        challenges: Vec<Challenge>,
        git_repository: Option<GitRepository>,
    ) -> Result<()> {
        let game_data = Self::instance();
        let mut data = game_data.lock().unwrap();
        data.challenges = Some(challenges);
        data.git_repository = git_repository;
        data.loading_completed = true;

        Ok(())
    }

    /// Set loading failure
    pub fn set_loading_failed(error_message: String) -> Result<()> {
        let game_data = Self::instance();
        let mut data = game_data.lock().unwrap();
        data.loading_failed = true;
        data.error_message = Some(error_message);
        Ok(())
    }

    /// Check if loading is completed
    pub fn is_loading_completed() -> bool {
        if let Some(game_data) = GLOBAL_GAME_DATA.get() {
            let data = game_data.lock().unwrap();
            data.loading_completed
        } else {
            false
        }
    }

    /// Check if loading failed
    pub fn is_loading_failed() -> bool {
        if let Some(game_data) = GLOBAL_GAME_DATA.get() {
            let data = game_data.lock().unwrap();
            data.loading_failed
        } else {
            false
        }
    }

    /// Get reference to challenges with callback (to avoid lifetime issues)
    pub fn with_challenges<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&Vec<Challenge>) -> R,
    {
        GLOBAL_GAME_DATA
            .get()
            .and_then(|game_data| game_data.lock().ok())
            .and_then(|data| data.challenges.as_ref().map(f))
    }

    /// Take the challenges (move out of GameData)
    pub fn take_challenges() -> Option<Vec<Challenge>> {
        if let Some(game_data) = GLOBAL_GAME_DATA.get() {
            let mut data = game_data.lock().unwrap();
            data.challenges.take()
        } else {
            None
        }
    }

    /// Get the git repository if available
    pub fn get_git_repository() -> Option<GitRepository> {
        if let Some(game_data) = GLOBAL_GAME_DATA.get() {
            let data = game_data.lock().unwrap();
            data.git_repository.clone()
        } else {
            None
        }
    }

    /// Set git repository information directly
    pub fn set_git_repository(git_repository: Option<GitRepository>) -> Result<()> {
        if let Some(game_data) = GLOBAL_GAME_DATA.get() {
            let mut data = game_data.lock().unwrap();
            data.git_repository = git_repository;
            Ok(())
        } else {
            Err(crate::GitTypeError::TerminalError(
                "GameData not initialized".to_string(),
            ))
        }
    }
}
