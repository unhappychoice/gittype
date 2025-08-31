use crate::Result;

pub struct SessionHistory;

impl Default for SessionHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionHistory {
    pub fn new() -> Self {
        Self
    }

    pub fn get_history(&self) -> Result<Vec<String>> {
        // TODO: Implement session history retrieval
        Ok(vec![])
    }
}
