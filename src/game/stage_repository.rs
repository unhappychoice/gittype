use super::{screen_manager::ScreenManager, screens::title_screen::TitleScreen};
use crate::game::models::ScreenType;

use crate::{models::Challenge, models::GitRepository, Result};
use once_cell::sync::Lazy;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum GameMode {
    Normal,     // Random selection of few challenges
    TimeAttack, // Time limit with all challenges
    Custom {
        // Custom configuration
        max_stages: Option<usize>,
        time_limit: Option<u64>, // seconds
        difficulty: DifficultyLevel,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DifficultyLevel {
    Easy,   // ~100 characters
    Normal, // ~200 characters
    Hard,   // ~500 characters
    Wild,   // Entire chunks, unpredictable length
    Zen,    // Entire file
}

impl DifficultyLevel {
    pub fn char_limits(&self) -> (usize, usize) {
        match self {
            DifficultyLevel::Easy => (20, 100),
            DifficultyLevel::Normal => (80, 200),
            DifficultyLevel::Hard => (180, 500),
            DifficultyLevel::Wild => (0, usize::MAX), // No limits - full chunks
            DifficultyLevel::Zen => (0, usize::MAX),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DifficultyLevel::Easy => "~100 characters",
            DifficultyLevel::Normal => "~200 characters",
            DifficultyLevel::Hard => "~500 characters",
            DifficultyLevel::Wild => "Full chunks",
            DifficultyLevel::Zen => "Entire files",
        }
    }

    pub fn subtitle(&self) -> &'static str {
        match self {
            DifficultyLevel::Easy => "Short code snippets",
            DifficultyLevel::Normal => "Medium functions",
            DifficultyLevel::Hard => "Long functions or classes",
            DifficultyLevel::Wild => "Unpredictable length chunks",
            DifficultyLevel::Zen => "Complete files as challenges",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StageConfig {
    pub game_mode: GameMode,
    pub max_stages: usize,
    pub seed: Option<u64>, // 再現可能なランダム生成用
}

impl Default for StageConfig {
    fn default() -> Self {
        Self {
            game_mode: GameMode::Normal,
            max_stages: 3,
            seed: None,
        }
    }
}

/// Repository for managing challenges and stage building
pub struct StageRepository {
    git_repository: Option<GitRepository>,
    config: StageConfig,
    built_stages: Vec<Challenge>,
    current_index: usize,
}

/// Global StageRepository instance
static GLOBAL_STAGE_REPOSITORY: Lazy<Arc<Mutex<StageRepository>>> =
    Lazy::new(|| Arc::new(Mutex::new(StageRepository::empty())));

impl StageRepository {
    /// Create a new StageRepository with the provided git_repository
    pub fn new(git_repository: Option<GitRepository>) -> Self {
        Self {
            git_repository,
            config: StageConfig::default(),
            built_stages: Vec::new(),
            current_index: 0,
        }
    }

    /// Create an empty StageRepository
    pub fn empty() -> Self {
        Self {
            git_repository: None,
            config: StageConfig::default(),
            built_stages: Vec::new(),
            current_index: 0,
        }
    }

    pub fn with_config(git_repository: Option<GitRepository>, config: StageConfig) -> Self {
        Self {
            git_repository,
            config,
            built_stages: Vec::new(),
            current_index: 0,
        }
    }

    pub fn with_mode(mut self, mode: GameMode) -> Self {
        self.config.game_mode = mode;
        self
    }

    pub fn with_max_stages(mut self, max_stages: usize) -> Self {
        self.config.max_stages = max_stages;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }

    /// Execute callback with challenges from GameData
    pub fn with_challenges<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Vec<Challenge>) -> R,
    {
        use crate::game::GameData;
        GameData::with_challenges(f)
    }

    /// Build stages based on configuration
    pub fn build_stages(&self) -> Vec<Challenge> {
        self.with_challenges(|available_challenges| {
            if available_challenges.is_empty() {
                return vec![];
            }

            match &self.config.game_mode {
                GameMode::Normal => self.build_normal_stages(available_challenges),
                GameMode::TimeAttack => self.build_time_attack_stages(available_challenges),
                GameMode::Custom {
                    max_stages,
                    difficulty,
                    ..
                } => self.build_custom_stages(
                    available_challenges,
                    max_stages.unwrap_or(self.config.max_stages),
                    difficulty,
                ),
            }
        })
        .unwrap_or_default()
    }

    fn build_normal_stages(&self, available_challenges: &[Challenge]) -> Vec<Challenge> {
        let mut challenges = available_challenges.to_vec();
        let target_count = self.config.max_stages.min(challenges.len());

        // Random selection
        let mut rng = self.create_rng();
        challenges.shuffle(&mut rng);

        // Prefer moderate length challenges (not too short, not too long)
        challenges.sort_by_key(|challenge| {
            let line_count = challenge.code_content.lines().count();
            // Consider 5-15 lines as ideal length
            if line_count < 5 {
                line_count + 100 // Penalty for too short
            } else if line_count > 20 {
                line_count + 50 // Penalty for too long
            } else {
                line_count // Ideal range
            }
        });

        challenges.into_iter().take(target_count).collect()
    }

    fn build_time_attack_stages(&self, available_challenges: &[Challenge]) -> Vec<Challenge> {
        // Time attack mode uses all challenges
        // Sort by difficulty (short to long)
        let mut challenges = available_challenges.to_vec();
        challenges.sort_by_key(|challenge| challenge.code_content.lines().count());

        challenges
    }

    fn build_custom_stages(
        &self,
        available_challenges: &[Challenge],
        max_stages: usize,
        difficulty: &DifficultyLevel,
    ) -> Vec<Challenge> {
        // Filter challenges by difficulty level
        let filtered_challenges: Vec<Challenge> = available_challenges
            .iter()
            .filter(|challenge| {
                match &challenge.difficulty_level {
                    Some(challenge_difficulty) => challenge_difficulty == difficulty,
                    None => false, // Skip challenges without difficulty level
                }
            })
            .cloned()
            .collect();

        let target_count = max_stages.min(filtered_challenges.len());

        // Random selection from filtered challenges
        let mut rng = self.create_rng();
        let mut selected_challenges = filtered_challenges;
        selected_challenges.shuffle(&mut rng);

        selected_challenges.into_iter().take(target_count).collect()
    }

    fn create_rng(&self) -> StdRng {
        match self.config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_os_rng(),
        }
    }

    pub fn get_mode_description(&self) -> String {
        match &self.config.game_mode {
            GameMode::Normal => {
                format!("Normal Mode - {} random challenges", self.config.max_stages)
            }
            GameMode::TimeAttack => "Time Attack Mode - All challenges".to_string(),
            GameMode::Custom {
                max_stages,
                time_limit,
                difficulty,
            } => {
                let stages = max_stages.unwrap_or(self.config.max_stages);
                let time_desc = match time_limit {
                    Some(t) => format!(" ({}s limit)", t),
                    None => "".to_string(),
                };
                format!(
                    "Custom Mode - {} challenges{} ({:?} difficulty)",
                    stages, time_desc, difficulty
                )
            }
        }
    }

    pub fn update_title_screen_data(&self, manager: &mut ScreenManager) -> Result<()> {
        let challenge_counts = self.count_challenges_by_difficulty();

        // Get the title screen and update its data
        if let Some(screen) = manager.get_screen_mut(&ScreenType::Title) {
            if let Some(title_screen) = screen.as_any_mut().downcast_mut::<TitleScreen>() {
                title_screen.set_challenge_counts(challenge_counts);
                title_screen.set_git_repository(self.git_repository.clone());
            }
        }
        Ok(())
    }

    pub fn count_challenges_by_difficulty(&self) -> [usize; 5] {
        self.with_challenges(|available_challenges| {
            let mut counts = [0; 5];
            for challenge in available_challenges {
                let difficulty_index = match challenge.difficulty_level {
                    Some(ref diff) => match diff {
                        DifficultyLevel::Easy => 0,
                        DifficultyLevel::Normal => 1,
                        DifficultyLevel::Hard => 2,
                        DifficultyLevel::Wild => 3,
                        DifficultyLevel::Zen => 4,
                    },
                    None => 0, // Default to easy
                };
                if difficulty_index < 5 {
                    counts[difficulty_index] += 1;
                }
            }
            counts
        })
        .unwrap_or([0; 5])
    }
}

impl Default for StageRepository {
    fn default() -> Self {
        Self::new(None)
    }
}

impl StageRepository {
    /// Get the global StageRepository instance
    pub fn instance() -> Arc<Mutex<StageRepository>> {
        GLOBAL_STAGE_REPOSITORY.clone()
    }

    /// Initialize the global StageRepository
    pub fn initialize_global(git_repository: Option<GitRepository>) -> Result<()> {
        let mut repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        *repo = Self::new(git_repository);
        // Don't build stages immediately - defer until needed
        Ok(())
    }

    /// Initialize the global StageRepository and build stages
    pub fn initialize_global_with_stages(git_repository: Option<GitRepository>) -> Result<()> {
        let mut repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        *repo = Self::new(git_repository);
        repo.build_and_store_stages();
        Ok(())
    }

    /// Set difficulty for the global repository and rebuild stages
    pub fn set_global_difficulty(difficulty: DifficultyLevel) -> Result<()> {
        let mut repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        // Create new config with the difficulty
        let config = StageConfig {
            game_mode: GameMode::Custom {
                max_stages: Some(3),
                time_limit: None,
                difficulty,
            },
            max_stages: 3,
            seed: None,
        };

        repo.config = config;
        repo.build_and_store_stages();
        repo.current_index = 0;
        Ok(())
    }

    /// Get the next challenge from the global repository
    pub fn get_next_global_challenge() -> Result<Option<Challenge>> {
        let mut repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        if repo.current_index < repo.built_stages.len() {
            let challenge = repo.built_stages[repo.current_index].clone();
            repo.current_index += 1;
            Ok(Some(challenge))
        } else {
            Ok(None)
        }
    }

    /// Check if there are more challenges available without consuming them
    pub fn has_next_global_challenge() -> Result<bool> {
        let repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        Ok(repo.current_index < repo.built_stages.len())
    }

    /// Get current stage info (current stage number, total stages)
    pub fn get_global_stage_info() -> Result<(usize, usize)> {
        let repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        Ok((repo.current_index + 1, repo.built_stages.len()))
    }

    /// Build stages and store them internally
    fn build_and_store_stages(&mut self) {
        self.built_stages = self.build_stages();
        self.current_index = 0;
    }

    /// Get a single challenge for specific difficulty
    pub fn get_challenge_for_difficulty(&self, difficulty: DifficultyLevel) -> Option<Challenge> {
        self.with_challenges(|available_challenges| {
            // Filter challenges by difficulty level
            let filtered_challenges: Vec<&Challenge> = available_challenges
                .iter()
                .filter(|challenge| {
                    match &challenge.difficulty_level {
                        Some(challenge_difficulty) => challenge_difficulty == &difficulty,
                        None => difficulty == DifficultyLevel::Normal, // Default to normal if no difficulty set
                    }
                })
                .collect();

            if filtered_challenges.is_empty() {
                return None;
            }

            // Random selection from filtered challenges
            let mut rng = self.create_rng();
            let selected_index = rng.random_range(0..filtered_challenges.len());
            Some(filtered_challenges[selected_index].clone())
        })
        .flatten()
    }

    /// Get challenge for specific difficulty (static version for global instance)
    pub fn get_global_challenge_for_difficulty(
        difficulty: DifficultyLevel,
    ) -> Result<Option<Challenge>> {
        let repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        Ok(repo.get_challenge_for_difficulty(difficulty))
    }
}
