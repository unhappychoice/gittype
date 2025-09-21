use crate::config::ConfigManager;
use crate::ui::color_mode::ColorMode;
use crate::ui::color_scheme::ColorScheme;
use crate::ui::theme::Theme;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static THEME_MANAGER: Lazy<Mutex<Option<ThemeManager>>> = Lazy::new(|| Mutex::new(None));

/// Theme manager with current theme and color mode
pub struct ThemeManager {
    current_theme: Theme,
    current_color_mode: ColorMode,
}

impl ThemeManager {
    /// Initialize the theme manager with optional config path
    pub fn init(config_path: Option<std::path::PathBuf>) -> anyhow::Result<()> {
        let config_manager = if let Some(path) = config_path {
            ConfigManager::with_config_path(path)?
        } else {
            ConfigManager::new()?
        };

        let config = config_manager.get_config();
        let current_theme = config.theme.current_theme.clone();
        let current_color_mode = config.theme.current_color_mode.clone();

        let mut manager = THEME_MANAGER.lock().unwrap();
        *manager = Some(ThemeManager {
            current_theme,
            current_color_mode,
        });
        Ok(())
    }

    /// Get the current color scheme
    pub(crate) fn get_current_color_scheme() -> ColorScheme {
        THEME_MANAGER
            .lock()
            .unwrap()
            .as_ref()
            .map(|tm| Self::get_color_scheme(&tm.current_theme, &tm.current_color_mode))
            .unwrap_or_else(|| {
                let default_theme = Theme::default();
                Self::get_color_scheme(&default_theme, &ColorMode::Dark)
            })
    }

    /// Get color scheme for the specified theme and color mode
    fn get_color_scheme(theme: &Theme, color_mode: &ColorMode) -> ColorScheme {
        match color_mode {
            ColorMode::Light => theme.light.clone(),
            ColorMode::Dark => theme.dark.clone(),
        }
    }
}
