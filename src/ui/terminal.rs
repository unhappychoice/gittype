use crate::Result;

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&self) -> Result<()> {
        // TODO: Initialize terminal
        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        // TODO: Cleanup terminal
        Ok(())
    }
}