use gittype::domain::models::color_mode::ColorMode;
use gittype::domain::models::theme::Theme;
use gittype::presentation::game::screens::settings_screen::SettingsScreenData;
use gittype::presentation::game::ScreenDataProvider;
use gittype::Result;

pub struct MockSettingsScreenDataProvider;

impl ScreenDataProvider for MockSettingsScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let themes = Theme::all_themes();
        let data = SettingsScreenData {
            color_modes: vec![ColorMode::Dark, ColorMode::Light],
            themes: themes.clone(),
            current_theme: themes.first().cloned().unwrap_or_default(),
            current_color_mode: ColorMode::Dark,
        };
        Ok(Box::new(data))
    }
}
