use shaku::Interface;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::domain::models::color_mode::ColorMode;
use crate::domain::models::color_scheme::{
    ColorScheme, CustomThemeFile, SerializableColor, ThemeFile,
};
use crate::domain::models::theme::Theme;
use crate::domain::services::config_service::ConfigServiceInterface;
use crate::infrastructure::storage::app_data_provider::AppDataProvider;
use crate::infrastructure::storage::file_storage::{FileStorage, FileStorageInterface};
use crate::presentation::ui::Colors;

pub struct ThemeServiceState {
    current_theme: Theme,
    current_color_mode: ColorMode,
    // Map of (theme_id, color_mode) -> (lang_name -> Color)
    language_colors: HashMap<(String, ColorMode), HashMap<String, ratatui::style::Color>>,
}

impl Default for ThemeServiceState {
    fn default() -> Self {
        Self {
            current_theme: Theme::default(),
            current_color_mode: ColorMode::Dark,
            language_colors: HashMap::new(),
        }
    }
}

pub trait ThemeServiceInterface: Interface {
    fn get_available_themes(&self) -> Vec<Theme>;
    fn get_current_theme(&self) -> Theme;
    fn get_current_color_mode(&self) -> ColorMode;
    fn set_current_theme(&self, theme: Theme);
    fn set_current_color_mode(&self, color_mode: ColorMode);
    fn get_current_color_scheme(&self) -> ColorScheme;
    fn get_colors(&self) -> Colors;
    fn get_color_for_language(&self, language_name: &str) -> ratatui::style::Color;
    fn init(&self) -> anyhow::Result<()>;
}

#[derive(shaku::Component)]
#[shaku(interface = ThemeServiceInterface)]
pub struct ThemeService {
    #[shaku(default)]
    state: RwLock<ThemeServiceState>,
    #[shaku(inject)]
    file_storage: Arc<dyn FileStorageInterface>,
    #[shaku(inject)]
    config_service: Arc<dyn ConfigServiceInterface>,
}

impl AppDataProvider for ThemeService {}

impl ThemeService {
    #[cfg(feature = "test-mocks")]
    pub fn new_for_test(current_theme: Theme, current_color_mode: ColorMode) -> Self {
        use crate::domain::services::config_service::ConfigService;
        let file_storage = Arc::new(FileStorage::new());
        let config_service = Arc::new(ConfigService::new(file_storage.clone()).unwrap());
        Self {
            state: RwLock::new(ThemeServiceState {
                current_theme,
                current_color_mode,
                language_colors: HashMap::new(),
            }),
            file_storage,
            config_service,
        }
    }

    /// Get color scheme for the specified theme and color mode
    fn get_color_scheme(theme: &Theme, color_mode: &ColorMode) -> ColorScheme {
        match color_mode {
            ColorMode::Light => theme.light.clone(),
            ColorMode::Dark => theme.dark.clone(),
        }
    }

    /// Get the custom theme file path
    fn get_custom_theme_path(&self) -> PathBuf {
        Self::get_app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("custom-theme.json")
    }

    /// Create default custom theme file if it doesn't exist
    fn create_default_custom_theme_file() -> anyhow::Result<()> {
        let file_storage = FileStorage::new();
        let custom_theme_path = Self::get_app_data_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("custom-theme.json");

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

    /// Load all language colors from JSON files
    fn load_all_language_colors(
    ) -> HashMap<(String, ColorMode), HashMap<String, ratatui::style::Color>> {
        let mut result = HashMap::new();

        let dark_colors = Self::parse_language_colors_json(include_str!(
            "../../../assets/languages/lang_dark.json"
        ));
        result.insert(("default".to_string(), ColorMode::Dark), dark_colors);

        let light_colors = Self::parse_language_colors_json(include_str!(
            "../../../assets/languages/lang_light.json"
        ));
        result.insert(("default".to_string(), ColorMode::Light), light_colors);

        let ascii_colors = Self::parse_language_colors_json(include_str!(
            "../../../assets/languages/lang_ascii.json"
        ));
        result.insert(("ascii".to_string(), ColorMode::Dark), ascii_colors.clone());
        result.insert(("ascii".to_string(), ColorMode::Light), ascii_colors);

        result
    }

    /// Parse language colors from JSON string
    fn parse_language_colors_json(json: &str) -> HashMap<String, ratatui::style::Color> {
        serde_json::from_str::<HashMap<String, SerializableColor>>(json)
            .unwrap_or_default()
            .into_iter()
            .map(|(key, color)| {
                let lang_name = key.strip_prefix("lang_").unwrap_or(&key);
                (lang_name.to_string(), color.into())
            })
            .collect()
    }
}

impl ThemeServiceInterface for ThemeService {
    fn init(&self) -> anyhow::Result<()> {
        // Create default custom theme file if it doesn't exist
        let _ = Self::create_default_custom_theme_file();

        let config = self.config_service.get_config();
        let current_theme_id = config.theme.current_theme_id.clone();
        let current_color_mode = config.theme.current_color_mode.clone();

        // Find theme by ID
        let available_themes = self.get_available_themes();
        let current_theme = available_themes
            .into_iter()
            .find(|t| t.id == current_theme_id)
            .unwrap_or_else(Theme::default);

        // Load all language colors
        let language_colors = Self::load_all_language_colors();

        let mut state = self.state.write().unwrap();
        state.current_theme = current_theme;
        state.current_color_mode = current_color_mode;
        state.language_colors = language_colors;
        Ok(())
    }

    fn get_available_themes(&self) -> Vec<Theme> {
        let custom_theme_path = self.get_custom_theme_path();
        Theme::all_themes()
            .into_iter()
            .chain(
                self.file_storage
                    .file_exists(&custom_theme_path)
                    .then(|| self.file_storage.read_to_string(&custom_theme_path).ok())
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

    fn get_current_theme(&self) -> Theme {
        self.state.read().unwrap().current_theme.clone()
    }

    fn get_current_color_mode(&self) -> ColorMode {
        self.state.read().unwrap().current_color_mode.clone()
    }

    fn set_current_theme(&self, theme: Theme) {
        self.state.write().unwrap().current_theme = theme;
    }

    fn set_current_color_mode(&self, color_mode: ColorMode) {
        self.state.write().unwrap().current_color_mode = color_mode;
    }

    fn get_current_color_scheme(&self) -> ColorScheme {
        let state = self.state.read().unwrap();
        Self::get_color_scheme(&state.current_theme, &state.current_color_mode)
    }

    fn get_colors(&self) -> Colors {
        Colors::new(self.get_current_color_scheme())
    }

    fn get_color_for_language(&self, language_name: &str) -> ratatui::style::Color {
        let state = self.state.read().unwrap();
        let key = if state.current_theme.id == "ascii" {
            ("ascii".to_string(), state.current_color_mode.clone())
        } else {
            ("default".to_string(), state.current_color_mode.clone())
        };

        state
            .language_colors
            .get(&key)
            .and_then(|lang_map| lang_map.get(language_name))
            .copied()
            .unwrap_or_else(|| {
                // Fallback: get default color from map or use White
                state
                    .language_colors
                    .get(&key)
                    .and_then(|lang_map| lang_map.get("default"))
                    .copied()
                    .unwrap_or(ratatui::style::Color::White)
            })
    }
}
