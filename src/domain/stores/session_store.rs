use shaku::Interface;

use std::sync::RwLock;

pub trait SessionStoreInterface: Interface {
    fn is_loading_completed(&self) -> bool;
    fn set_loading_completed(&self, completed: bool);

    fn is_loading_failed(&self) -> bool;
    fn set_loading_failed(&self, failed: bool);

    fn get_error_message(&self) -> Option<String>;
    fn set_error_message(&self, message: String);
    fn clear_error_message(&self);

    fn clear(&self);
}

#[derive(shaku::Component)]
#[shaku(interface = SessionStoreInterface)]
pub struct SessionStore {
    #[shaku(default)]
    loading_completed: RwLock<bool>,
    #[shaku(default)]
    loading_failed: RwLock<bool>,
    #[shaku(default)]
    error_message: RwLock<Option<String>>,
}

impl SessionStore {
    #[cfg(feature = "test-mocks")]
    pub fn new_for_test() -> Self {
        Self {
            loading_completed: RwLock::new(false),
            loading_failed: RwLock::new(false),
            error_message: RwLock::new(None),
        }
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self {
            loading_completed: RwLock::new(false),
            loading_failed: RwLock::new(false),
            error_message: RwLock::new(None),
        }
    }
}

impl SessionStoreInterface for SessionStore {
    fn is_loading_completed(&self) -> bool {
        *self.loading_completed.read().unwrap()
    }

    fn set_loading_completed(&self, completed: bool) {
        *self.loading_completed.write().unwrap() = completed;
    }

    fn is_loading_failed(&self) -> bool {
        *self.loading_failed.read().unwrap()
    }

    fn set_loading_failed(&self, failed: bool) {
        *self.loading_failed.write().unwrap() = failed;
    }

    fn get_error_message(&self) -> Option<String> {
        self.error_message.read().unwrap().clone()
    }

    fn set_error_message(&self, message: String) {
        *self.error_message.write().unwrap() = Some(message);
    }

    fn clear_error_message(&self) {
        *self.error_message.write().unwrap() = None;
    }

    fn clear(&self) {
        *self.loading_completed.write().unwrap() = false;
        *self.loading_failed.write().unwrap() = false;
        *self.error_message.write().unwrap() = None;
    }
}
