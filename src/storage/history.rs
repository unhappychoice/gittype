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
        // Session history retrieval not yet implemented
        // Returns empty vec as placeholder until database integration is complete
        Ok(vec![])
    }
}
