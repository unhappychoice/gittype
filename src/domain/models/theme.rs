use crate::domain::models::color_mode::ColorMode;
use crate::domain::models::color_scheme::ColorScheme;
use serde::{Deserialize, Serialize};

const THEME_FILES: &[&str] = &[
    include_str!("../../../assets/themes/default.json"),
    include_str!("../../../assets/themes/original.json"),
    include_str!("../../../assets/themes/ascii.json"),
    include_str!("../../../assets/themes/aurora.json"),
    include_str!("../../../assets/themes/blood_oath.json"),
    include_str!("../../../assets/themes/cyber_void.json"),
    include_str!("../../../assets/themes/eclipse.json"),
    include_str!("../../../assets/themes/glacier.json"),
    include_str!("../../../assets/themes/inferno.json"),
    include_str!("../../../assets/themes/neon_abyss.json"),
    include_str!("../../../assets/themes/oblivion.json"),
    include_str!("../../../assets/themes/runic.json"),
    include_str!("../../../assets/themes/spectral.json"),
    include_str!("../../../assets/themes/starforge.json"),
    include_str!("../../../assets/themes/venom.json"),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub light: ColorScheme,
    pub dark: ColorScheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self::all_themes()
            .into_iter()
            .find(|theme| theme.id == "default")
            .expect("Default theme not found")
    }
}

impl Theme {
    /// Get all builtin themes
    pub fn all_themes() -> Vec<Self> {
        THEME_FILES
            .iter()
            .map(|json| {
                let theme_file: crate::domain::models::color_scheme::ThemeFile =
                    serde_json::from_str(json).expect("Failed to parse theme JSON");

                let light = ColorScheme::from_theme_file(&theme_file, &ColorMode::Light);
                let dark = ColorScheme::from_theme_file(&theme_file, &ColorMode::Dark);

                Self {
                    id: theme_file.id,
                    name: theme_file.name,
                    description: theme_file.description,
                    light,
                    dark,
                }
            })
            .collect()
    }
}
