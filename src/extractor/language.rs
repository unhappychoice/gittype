#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    TypeScript,
    Python,
    Ruby,
    Go,
}

impl Language {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Language::Rust),
            "ts" | "tsx" => Some(Language::TypeScript),
            "py" => Some(Language::Python),
            "rb" => Some(Language::Ruby),
            "go" => Some(Language::Go),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::TypeScript => "ts",
            Language::Python => "py",
            Language::Ruby => "rb",
            Language::Go => "go",
        }
    }
}
