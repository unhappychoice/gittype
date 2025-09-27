use crate::domain::models::color_mode::ColorMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: ThemeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_id")]
    pub current_theme_id: String,
    pub current_color_mode: ColorMode,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            current_theme_id: "default".to_string(),
            current_color_mode: ColorMode::default(),
        }
    }
}

fn default_theme_id() -> String {
    "default".to_string()
}
