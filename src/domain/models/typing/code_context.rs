#[derive(Debug, Clone, Default)]
pub struct CodeContext {
    pub pre_context: Vec<String>,
    pub post_context: Vec<String>,
}

impl CodeContext {
    pub fn empty() -> Self {
        Self::default()
    }
}
