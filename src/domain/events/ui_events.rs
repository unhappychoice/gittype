use super::Event;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct RenderRequested {
    pub screen_name: String,
}

impl Event for RenderRequested {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct ScreenChanged {
    pub from: String,
    pub to: String,
}

impl Event for ScreenChanged {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct UserInputReceived {
    pub input: String,
}

impl Event for UserInputReceived {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct ConfigUpdated {
    pub key: String,
    pub value: String,
}

impl Event for ConfigUpdated {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
