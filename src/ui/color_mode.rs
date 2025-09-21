use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorMode {
    Dark,
    Light,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Dark
    }
}
