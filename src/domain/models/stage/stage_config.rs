use super::GameMode;

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
