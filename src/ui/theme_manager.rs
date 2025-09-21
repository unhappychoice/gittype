use crate::config::ConfigManager;
use crate::ui::color_mode::ColorMode;
use crate::ui::color_scheme::{ColorScheme, CustomThemeFile, ThemeFile};
use crate::ui::theme::Theme;
use once_cell::sync::Lazy;

pub static THEME_MANAGER: Lazy<std::sync::RwLock<ThemeManager>> = Lazy::new(|| {
    std::sync::RwLock::new(ThemeManager {
        current_theme: Theme::default(),
        current_color_mode: ColorMode::Dark,
    })
});

/// Theme manager with current theme and color mode
pub struct ThemeManager {
    pub current_theme: Theme,
    pub current_color_mode: ColorMode,
}

impl ThemeManager {
    /// Initialize the theme manager with optional config path
    pub fn init(config_path: Option<std::path::PathBuf>) -> anyhow::Result<()> {
        let config_manager = if let Some(path) = config_path {
            ConfigManager::with_config_path(path)?
        } else {
            ConfigManager::new()?
        };

        // Create default custom theme file if it doesn't exist
        let _ = Self::create_default_custom_theme_file();

        let config = config_manager.get_config();
        let current_theme_id = config.theme.current_theme_id.clone();
        let current_color_mode = config.theme.current_color_mode.clone();

        let mut manager = THEME_MANAGER.write().unwrap();

        // Find theme by ID
        let available_themes = manager.get_available_themes();
        let current_theme = available_themes
            .into_iter()
            .find(|t| t.id == current_theme_id)
            .unwrap_or_else(Theme::default);

        manager.current_theme = current_theme;
        manager.current_color_mode = current_color_mode;
        Ok(())
    }

    /// Get the current color scheme
    pub(crate) fn get_current_color_scheme() -> ColorScheme {
        let manager = THEME_MANAGER.read().unwrap();
        Self::get_color_scheme(&manager.current_theme, &manager.current_color_mode)
    }

    /// Get color scheme for the specified theme and color mode
    fn get_color_scheme(theme: &Theme, color_mode: &ColorMode) -> ColorScheme {
        match color_mode {
            ColorMode::Light => theme.light.clone(),
            ColorMode::Dark => theme.dark.clone(),
        }
    }

    /// Get all available themes
    pub fn get_available_themes(&self) -> Vec<Theme> {
        Theme::all_themes()
            .into_iter()
            .chain(
                Self::get_custom_theme_path()
                    .exists()
                    .then(|| std::fs::read_to_string(Self::get_custom_theme_path()).ok())
                    .flatten()
                    .and_then(|json| serde_json::from_str::<CustomThemeFile>(&json).ok())
                    .map(|custom_theme_file| {
                        let theme_file = custom_theme_file.to_theme_file();
                        let dark = ColorScheme::from_theme_file(&theme_file, &ColorMode::Dark);
                        let light = ColorScheme::from_theme_file(&theme_file, &ColorMode::Light);

                        Theme {
                            id: theme_file.id,
                            name: theme_file.name,
                            description: theme_file.description,
                            dark,
                            light,
                        }
                    }),
            )
            .collect()
    }

    /// Get the custom theme file path
    fn get_custom_theme_path() -> std::path::PathBuf {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home_dir)
            .join(".gittype")
            .join("custom-theme.json")
    }

    /// Create default custom theme file if it doesn't exist
    fn create_default_custom_theme_file() -> anyhow::Result<()> {
        let custom_theme_path = Self::get_custom_theme_path();

        if custom_theme_path.exists() {
            return Ok(());
        }

        // Create directory if it doesn't exist
        if let Some(parent) = custom_theme_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create default custom theme based on the default theme
        let default_theme_json = include_str!("../../assets/themes/default.json");
        let default_theme_file: ThemeFile = serde_json::from_str(default_theme_json)?;

        let custom_theme = CustomThemeFile {
            dark: default_theme_file.dark,
            light: default_theme_file.light,
        };

        let custom_theme_json = serde_json::to_string_pretty(&custom_theme)?;
        std::fs::write(&custom_theme_path, custom_theme_json)?;

        Ok(())
    }
}
