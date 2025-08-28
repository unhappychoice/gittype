use super::Challenge;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Debug, Clone)]
pub enum GameMode {
    Normal,      // Random selection of few challenges
    TimeAttack,  // Time limit with all challenges
    Custom {     // Custom configuration
        max_stages: Option<usize>,
        time_limit: Option<u64>, // seconds
        difficulty: DifficultyLevel,
    },
}

#[derive(Debug, Clone)]
pub enum DifficultyLevel {
    Easy,    // Prefer shorter chunks
    Medium,  // Balanced selection
    Hard,    // Prefer longer chunks
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

pub struct StageBuilder {
    config: StageConfig,
}

impl StageBuilder {
    pub fn new(config: StageConfig) -> Self {
        Self { config }
    }

    pub fn with_mode(mode: GameMode) -> Self {
        let config = StageConfig {
            game_mode: mode,
            ..Default::default()
        };
        Self::new(config)
    }

    pub fn with_max_stages(mut self, max_stages: usize) -> Self {
        self.config.max_stages = max_stages;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }

    pub fn build_stages(&self, available_challenges: Vec<Challenge>) -> Vec<Challenge> {
        if available_challenges.is_empty() {
            return vec![];
        }

        match &self.config.game_mode {
            GameMode::Normal => self.build_normal_stages(available_challenges),
            GameMode::TimeAttack => self.build_time_attack_stages(available_challenges),
            GameMode::Custom { max_stages, difficulty, .. } => {
                self.build_custom_stages(available_challenges, max_stages.unwrap_or(self.config.max_stages), difficulty)
            }
        }
    }

    fn build_normal_stages(&self, mut available_challenges: Vec<Challenge>) -> Vec<Challenge> {
        let target_count = self.config.max_stages.min(available_challenges.len());
        
        // Random selection
        let mut rng = self.create_rng();
        available_challenges.shuffle(&mut rng);
        
        // Prefer moderate length challenges (not too short, not too long)
        available_challenges.sort_by_key(|challenge| {
            let line_count = challenge.code_content.lines().count();
            // Consider 5-15 lines as ideal length
            if line_count < 5 {
                line_count + 100 // Penalty for too short
            } else if line_count > 20 {
                line_count + 50  // Penalty for too long
            } else {
                line_count       // Ideal range
            }
        });
        
        available_challenges.into_iter().take(target_count).collect()
    }

    fn build_time_attack_stages(&self, available_challenges: Vec<Challenge>) -> Vec<Challenge> {
        // Time attack mode uses all challenges
        // Sort by difficulty (short to long)
        let mut challenges = available_challenges;
        challenges.sort_by_key(|challenge| {
            challenge.code_content.lines().count()
        });
        
        challenges
    }

    fn build_custom_stages(&self, mut available_challenges: Vec<Challenge>, max_stages: usize, difficulty: &DifficultyLevel) -> Vec<Challenge> {
        let target_count = max_stages.min(available_challenges.len());
        
        // Filter and sort based on difficulty level with randomness
        let mut rng = self.create_rng();
        match difficulty {
            DifficultyLevel::Easy => {
                // Prefer shorter chunks but add randomness within size groups
                available_challenges.sort_by_key(|challenge| {
                    challenge.code_content.lines().count()
                });
                // Add randomness by shuffling challenges within same line count groups
                Self::shuffle_within_groups(&mut available_challenges, &mut rng);
            }
            DifficultyLevel::Medium => {
                // Random mix
                available_challenges.shuffle(&mut rng);
            }
            DifficultyLevel::Hard => {
                // Prefer longer chunks but add randomness within size groups
                available_challenges.sort_by_key(|challenge| {
                    std::cmp::Reverse(challenge.code_content.lines().count())
                });
                // Add randomness by shuffling challenges within same line count groups
                Self::shuffle_within_groups(&mut available_challenges, &mut rng);
            }
        }
        
        available_challenges.into_iter().take(target_count).collect()
    }

    fn create_rng(&self) -> StdRng {
        match self.config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        }
    }

    pub fn get_mode_description(&self) -> String {
        match &self.config.game_mode {
            GameMode::Normal => format!("Normal Mode - {} random challenges", self.config.max_stages),
            GameMode::TimeAttack => "Time Attack Mode - All challenges".to_string(),
            GameMode::Custom { max_stages, time_limit, difficulty } => {
                let stages = max_stages.unwrap_or(self.config.max_stages);
                let time_desc = match time_limit {
                    Some(t) => format!(" ({}s limit)", t),
                    None => "".to_string(),
                };
                format!("Custom Mode - {} challenges{} ({:?} difficulty)", stages, time_desc, difficulty)
            }
        }
    }

    fn shuffle_within_groups(challenges: &mut Vec<Challenge>, rng: &mut StdRng) {
        // Group challenges by line count and shuffle within each group
        let mut i = 0;
        while i < challenges.len() {
            let current_line_count = challenges[i].code_content.lines().count();
            let mut j = i;
            
            // Find the end of the current group (same line count)
            while j < challenges.len() && challenges[j].code_content.lines().count() == current_line_count {
                j += 1;
            }
            
            // Shuffle the group [i..j)
            if j - i > 1 {
                challenges[i..j].shuffle(rng);
            }
            
            i = j;
        }
    }
}

