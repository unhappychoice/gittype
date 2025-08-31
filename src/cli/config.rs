#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub extraction: ExtractionConfig,
    pub game: GameConfig,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ExtractionConfig {
    pub languages: Vec<String>,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GameConfig {
    pub show_live_metrics: bool,
    pub syntax_highlighting: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            extraction: ExtractionConfig {
                languages: vec![
                    "rust".to_string(),
                    "typescript".to_string(),
                    "python".to_string(),
                    "go".to_string(),
                    "ruby".to_string(),
                ],
                include: vec!["src/**".to_string()],
                exclude: vec!["target/**".to_string(), "node_modules/**".to_string()],
            },
            game: GameConfig {
                show_live_metrics: true,
                syntax_highlighting: true,
            },
        }
    }
}
