use super::super::models::color_mode::ColorMode;
use super::super::models::color_scheme::{ColorScheme, CustomThemeFile, ThemeFile};
use super::super::models::theme::Theme;
use crate::domain::services::config_manager::ConfigService;
use crate::infrastructure::storage::file_storage::{FileStorage, FileStorageInterface};
use once_cell::sync::Lazy;

pub static THEME_MANAGER: Lazy<std::sync::RwLock<ThemeManager>> = Lazy::new(|| {
    std::sync::RwLock::new(ThemeManager {
        current_theme: Theme::default(),
        current_color_mode: ColorMode::Dark,
        file_storage: FileStorage::new(),
    })
});

/// Theme manager with current theme and color mode
pub struct ThemeManager {
    pub current_theme: Theme,
    pub current_color_mode: ColorMode,
    file_storage: FileStorage,
}

impl ThemeManager {
    #[cfg(feature = "test-mocks")]
    pub fn new_for_test(current_theme: Theme, current_color_mode: ColorMode) -> Self {
        Self {
            current_theme,
            current_color_mode,
            file_storage: FileStorage::new(),
        }
    }

    /// Initialize the theme manager
    pub fn init() -> anyhow::Result<()> {
        let config_manager = ConfigService::new()?;

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
                self.file_storage
                    .file_exists(&Self::get_custom_theme_path())
                    .then(|| {
                        self.file_storage
                            .read_to_string(&Self::get_custom_theme_path())
                            .ok()
                    })
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
        let file_storage = FileStorage::new();
        file_storage
            .get_app_data_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("custom-theme.json")
    }

    /// Create default custom theme file if it doesn't exist
    fn create_default_custom_theme_file() -> anyhow::Result<()> {
        let file_storage = FileStorage::new();
        let custom_theme_path = Self::get_custom_theme_path();

        if file_storage.file_exists(&custom_theme_path) {
            return Ok(());
        }

        // Create directory if it doesn't exist
        if let Some(parent) = custom_theme_path.parent() {
            file_storage.create_dir_all(parent)?;
        }

        // Create default custom theme based on the default theme
        let default_theme_json = include_str!("../../../assets/themes/default.json");
        let default_theme_file: ThemeFile = serde_json::from_str(default_theme_json)?;

        let custom_theme = CustomThemeFile {
            dark: default_theme_file.dark,
            light: default_theme_file.light,
        };

        let custom_theme_json = serde_json::to_string_pretty(&custom_theme)?;
        file_storage.write(&custom_theme_path, custom_theme_json.as_bytes())?;

        Ok(())
    }
}
