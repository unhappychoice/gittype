#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Rust,
    TypeScript,
    Python,
}

impl Language {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Language::Rust),
            "ts" | "tsx" => Some(Language::TypeScript),
            "py" => Some(Language::Python),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::TypeScript => "ts",
            Language::Python => "py",
        }
    }
}