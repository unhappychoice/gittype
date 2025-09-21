use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ColorMode {
    #[default]
    Dark,
    Light,
}
