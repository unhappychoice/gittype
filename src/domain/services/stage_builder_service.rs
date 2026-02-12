use crate::domain::models::{Challenge, DifficultyLevel, GameMode, GitRepository, StageConfig};
use crate::domain::stores::{
    ChallengeStoreInterface, RepositoryStoreInterface, SessionStoreInterface,
};
use crate::presentation::tui::screens::TitleScreen;
use crate::presentation::tui::{ScreenManagerImpl, ScreenType};
use crate::Result;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngExt, SeedableRng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Repository for managing challenges and stage building
#[derive(shaku::Component)]
#[shaku(interface = StageRepositoryInterface)]
pub struct StageRepository {
    #[shaku(default)]
    git_repository: Mutex<Option<GitRepository>>,
    #[shaku(default)]
    config: Mutex<StageConfig>,
    #[shaku(default)]
    built_stages: Mutex<Vec<Challenge>>,
    #[shaku(default)]
    #[allow(dead_code)]
    current_index: Mutex<usize>,
    #[shaku(default)]
    difficulty_indices: Mutex<HashMap<DifficultyLevel, Vec<usize>>>,
    #[shaku(default)]
    indices_cached: Mutex<bool>,
    #[shaku(default)]
    cached_challenges: Mutex<Option<Vec<Challenge>>>,
    #[shaku(inject)]
    challenge_store: Arc<dyn ChallengeStoreInterface>,
    #[shaku(inject)]
    #[allow(dead_code)]
    repository_store: Arc<dyn RepositoryStoreInterface>,
    #[shaku(inject)]
    #[allow(dead_code)]
    session_store: Arc<dyn SessionStoreInterface>,
}

pub trait StageRepositoryInterface: shaku::Interface {
    // Note: update_title_screen_data() is not included here because it's generic
    // over the Backend type, making the trait not object-safe. Instead, callers
    // should downcast to the concrete StageRepository type when needed.
    fn as_any(&self) -> &dyn std::any::Any;
}
impl StageRepositoryInterface for StageRepository {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl StageRepository {
    pub fn new(
        git_repository: Option<GitRepository>,
        challenge_store: Arc<dyn ChallengeStoreInterface>,
        repository_store: Arc<dyn RepositoryStoreInterface>,
        session_store: Arc<dyn SessionStoreInterface>,
    ) -> Self {
        Self {
            git_repository: Mutex::new(git_repository),
            config: Mutex::new(StageConfig::default()),
            built_stages: Mutex::new(Vec::new()),
            current_index: Mutex::new(0),
            difficulty_indices: Mutex::new(HashMap::new()),
            indices_cached: Mutex::new(false),
            cached_challenges: Mutex::new(None),
            challenge_store,
            repository_store,
            session_store,
        }
    }

    pub fn with_config(
        git_repository: Option<GitRepository>,
        config: StageConfig,
        challenge_store: Arc<dyn ChallengeStoreInterface>,
        repository_store: Arc<dyn RepositoryStoreInterface>,
        session_store: Arc<dyn SessionStoreInterface>,
    ) -> Self {
        Self {
            git_repository: Mutex::new(git_repository),
            config: Mutex::new(config),
            built_stages: Mutex::new(Vec::new()),
            current_index: Mutex::new(0),
            difficulty_indices: Mutex::new(HashMap::new()),
            indices_cached: Mutex::new(false),
            cached_challenges: Mutex::new(None),
            challenge_store,
            repository_store,
            session_store,
        }
    }

    pub fn with_mode(self, mode: GameMode) -> Self {
        self.config.lock().unwrap().game_mode = mode;
        self
    }

    pub fn with_max_stages(self, max_stages: usize) -> Self {
        self.config.lock().unwrap().max_stages = max_stages;
        self
    }

    pub fn with_seed(self, seed: u64) -> Self {
        self.config.lock().unwrap().seed = Some(seed);
        self
    }

    pub fn with_challenges<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Vec<Challenge>) -> R,
    {
        self.challenge_store.get_challenges().as_ref().map(f)
    }

    /// Build stages based on configuration
    pub fn build_stages(&self) -> Vec<Challenge> {
        self.with_challenges(|available_challenges| {
            if available_challenges.is_empty() {
                return vec![];
            }

            let config = self.config.lock().unwrap();
            match &config.game_mode {
                GameMode::Normal => self.build_normal_stages(available_challenges),
                GameMode::TimeAttack => self.build_time_attack_stages(available_challenges),
                GameMode::Custom {
                    max_stages,
                    difficulty,
                    ..
                } => self.build_custom_stages(
                    available_challenges,
                    max_stages.unwrap_or(config.max_stages),
                    difficulty,
                ),
            }
        })
        .unwrap_or_default()
    }

    fn build_normal_stages(&self, available_challenges: &[Challenge]) -> Vec<Challenge> {
        let mut challenges = available_challenges.to_vec();
        let target_count = self.config.lock().unwrap().max_stages.min(challenges.len());

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
        match self.config.lock().unwrap().seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => rand::make_rng(),
        }
    }

    pub fn get_mode_description(&self) -> String {
        let config = self.config.lock().unwrap();
        match &config.game_mode {
            GameMode::Normal => {
                format!("Normal Mode - {} random challenges", config.max_stages)
            }
            GameMode::TimeAttack => "Time Attack Mode - All challenges".to_string(),
            GameMode::Custom {
                max_stages,
                time_limit,
                difficulty,
            } => {
                let stages = max_stages.unwrap_or(config.max_stages);
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

    pub fn update_title_screen_data<B: ratatui::backend::Backend + Send + 'static>(
        &self,
        manager: &mut ScreenManagerImpl<B>,
    ) -> Result<()> {
        // Only update if indices are cached to avoid GameData access during screen transitions
        if !*self.indices_cached.lock().unwrap() {
            return Ok(());
        }

        let challenge_counts = self.count_challenges_by_difficulty();

        // Get the title screen and update its data
        if let Some(screen) = manager.get_screen_mut(&ScreenType::Title) {
            if let Some(title_screen) = screen.as_any().downcast_ref::<TitleScreen>() {
                title_screen.set_challenge_counts(challenge_counts);
                title_screen.set_git_repository(self.git_repository.lock().unwrap().clone());
            }
        }
        Ok(())
    }

    pub fn count_challenges_by_difficulty(&self) -> [usize; 5] {
        // Use cached indices for O(1) counting
        if *self.indices_cached.lock().unwrap() {
            let mut counts = [0; 5];
            let difficulty_indices = self.difficulty_indices.lock().unwrap();
            counts[0] = difficulty_indices
                .get(&DifficultyLevel::Easy)
                .map_or(0, |v| v.len());
            counts[1] = difficulty_indices
                .get(&DifficultyLevel::Normal)
                .map_or(0, |v| v.len());
            counts[2] = difficulty_indices
                .get(&DifficultyLevel::Hard)
                .map_or(0, |v| v.len());
            counts[3] = difficulty_indices
                .get(&DifficultyLevel::Wild)
                .map_or(0, |v| v.len());
            counts[4] = difficulty_indices
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

// Default implementation removed - use new() with stores instead

impl StageRepository {
    /// Set cached challenges (for testing)
    pub fn set_cached_challenges(&self, challenges: Vec<Challenge>) {
        *self.built_stages.lock().unwrap() = challenges.clone();
        *self.cached_challenges.lock().unwrap() = Some(challenges);
        *self.indices_cached.lock().unwrap() = false;
    }

    /// Get a single challenge for specific difficulty (optimized with cached data)
    pub fn get_challenge_for_difficulty(&self, difficulty: DifficultyLevel) -> Option<Challenge> {
        // Ensure indices are built
        self.build_difficulty_indices();

        let difficulty_indices = self.difficulty_indices.lock().unwrap();
        if let Some(indices) = difficulty_indices.get(&difficulty) {
            if indices.is_empty() {
                None
            } else {
                let cached_challenges = self.cached_challenges.lock().unwrap();
                if let Some(ref challenges) = *cached_challenges {
                    // O(1) lookup using cached challenges (no GameData access!)
                    let mut rng = self.create_rng();
                    let random_index_pos = rng.random_range(0..indices.len());
                    let challenge_index = indices[random_index_pos];

                    if challenge_index < challenges.len() {
                        Some(challenges[challenge_index].clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    /// Build difficulty indices for O(1) challenge lookup
    pub fn build_difficulty_indices(&self) {
        if *self.indices_cached.lock().unwrap() {
            return;
        }

        // Create temporary indices map
        let mut temp_indices: HashMap<DifficultyLevel, Vec<usize>> = HashMap::new();

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
            *self.difficulty_indices.lock().unwrap() = temp_indices;
            *self.cached_challenges.lock().unwrap() = Some(cached_challenges);
            *self.indices_cached.lock().unwrap() = true;
        }
    }
}
