use crate::presentation::game::screens::TitleScreen;
use crate::presentation::game::{ScreenManager, ScreenType};
use crate::{domain::models::Challenge, domain::models::GitRepository, Result};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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
    // Performance optimization: difficulty-based challenge indices
    difficulty_indices: std::collections::HashMap<DifficultyLevel, Vec<usize>>,
    indices_cached: bool,
    // Cache challenges for direct access (eliminates GameData dependency)
    cached_challenges: Option<Vec<Challenge>>,
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
            difficulty_indices: std::collections::HashMap::new(),
            indices_cached: false,
            cached_challenges: None,
        }
    }

    /// Create an empty StageRepository
    pub fn empty() -> Self {
        Self {
            git_repository: None,
            config: StageConfig::default(),
            built_stages: Vec::new(),
            current_index: 0,
            difficulty_indices: std::collections::HashMap::new(),
            indices_cached: false,
            cached_challenges: None,
        }
    }

    pub fn with_config(git_repository: Option<GitRepository>, config: StageConfig) -> Self {
        Self {
            git_repository,
            config,
            built_stages: Vec::new(),
            current_index: 0,
            difficulty_indices: std::collections::HashMap::new(),
            indices_cached: false,
            cached_challenges: None,
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
        use crate::presentation::game::GameData;
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
        // Only update if indices are cached to avoid GameData access during screen transitions
        if !self.indices_cached {
            return Ok(());
        }

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
        // Use cached indices for O(1) counting
        if self.indices_cached {
            let mut counts = [0; 5];
            counts[0] = self
                .difficulty_indices
                .get(&DifficultyLevel::Easy)
                .map_or(0, |v| v.len());
            counts[1] = self
                .difficulty_indices
                .get(&DifficultyLevel::Normal)
                .map_or(0, |v| v.len());
            counts[2] = self
                .difficulty_indices
                .get(&DifficultyLevel::Hard)
                .map_or(0, |v| v.len());
            counts[3] = self
                .difficulty_indices
                .get(&DifficultyLevel::Wild)
                .map_or(0, |v| v.len());
            counts[4] = self
                .difficulty_indices
                .get(&DifficultyLevel::Zen)
                .map_or(0, |v| v.len());
            counts
        } else {
            // Fallback to GameData access (should only happen during initialization)
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
        // Don't rebuild stages/indices - just update config
        // Indices are already cached and difficulty filtering happens at runtime
        repo.current_index = 0;
        log::info!(
            "✅ StageRepository: Difficulty set to {:?} (keeping cached indices)",
            difficulty
        );
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

    /// Get a single challenge for specific difficulty (optimized with cached data)
    pub fn get_challenge_for_difficulty(
        &mut self,
        difficulty: DifficultyLevel,
    ) -> Option<Challenge> {
        // Ensure indices are built
        self.build_difficulty_indices();

        if let Some(indices) = self.difficulty_indices.get(&difficulty) {
            if indices.is_empty() {
                None
            } else if let Some(ref cached_challenges) = self.cached_challenges {
                // O(1) lookup using cached challenges (no GameData access!)
                let mut rng = self.create_rng();
                let random_index_pos = rng.random_range(0..indices.len());
                let challenge_index = indices[random_index_pos];

                if challenge_index < cached_challenges.len() {
                    Some(cached_challenges[challenge_index].clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get challenge for specific difficulty (static version for global instance)
    pub fn get_global_challenge_for_difficulty(
        difficulty: DifficultyLevel,
    ) -> Result<Option<Challenge>> {
        let mut repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        Ok(repo.get_challenge_for_difficulty(difficulty))
    }

    /// Build difficulty indices for global repository
    pub fn build_global_difficulty_indices() -> Result<()> {
        let mut repo = GLOBAL_STAGE_REPOSITORY.lock().map_err(|e| {
            crate::GitTypeError::TerminalError(format!(
                "Failed to lock global StageRepository: {}",
                e
            ))
        })?;

        repo.build_difficulty_indices();
        Ok(())
    }

    /// Update title screen data globally (called once during initialization)
    pub fn update_global_title_screen_data() -> Result<()> {
        // This will be called from finalizing step with proper screen manager access
        // For now, just log that it's ready
        log::info!(
            "✅ StageRepository: Title screen data ready (will be updated by ScreenManager)"
        );
        Ok(())
    }

    /// Build difficulty indices for O(1) challenge lookup
    fn build_difficulty_indices(&mut self) {
        if self.indices_cached {
            return;
        }

        // Create temporary indices map
        let mut temp_indices: std::collections::HashMap<DifficultyLevel, Vec<usize>> =
            std::collections::HashMap::new();

        // Initialize all difficulty levels
        temp_indices.insert(DifficultyLevel::Easy, Vec::new());
        temp_indices.insert(DifficultyLevel::Normal, Vec::new());
        temp_indices.insert(DifficultyLevel::Hard, Vec::new());
        temp_indices.insert(DifficultyLevel::Wild, Vec::new());
        temp_indices.insert(DifficultyLevel::Zen, Vec::new());

        let result = self.with_challenges(|available_challenges| {
            // Store challenges for caching outside the closure
            let cached_challenges = available_challenges.clone();

            for (index, challenge) in available_challenges.iter().enumerate() {
                let difficulty = challenge
                    .difficulty_level
                    .unwrap_or(DifficultyLevel::Normal);
                if let Some(indices) = temp_indices.get_mut(&difficulty) {
                    indices.push(index);
                }
            }

            cached_challenges
        });

        if let Some(cached_challenges) = result {
            // Replace the actual indices with the temporary ones
            self.difficulty_indices = temp_indices;
            self.cached_challenges = Some(cached_challenges);
            self.indices_cached = true;
        }
    }
}
