use crate::domain::models::Challenge;
use shaku::Interface;
use std::sync::RwLock;

pub trait ChallengeStoreInterface: Interface {
    fn get_challenges(&self) -> Option<Vec<Challenge>>;
    fn set_challenges(&self, challenges: Vec<Challenge>);
    fn clear(&self);
    fn take_challenges(&self) -> Option<Vec<Challenge>>;
}

#[derive(shaku::Component)]
#[shaku(interface = ChallengeStoreInterface)]
pub struct ChallengeStore {
    #[shaku(default)]
    challenges: RwLock<Option<Vec<Challenge>>>,
}

impl ChallengeStore {
    #[cfg(feature = "test-mocks")]
    pub fn new_for_test() -> Self {
        Self {
            challenges: RwLock::new(None),
        }
    }
}

impl Default for ChallengeStore {
    fn default() -> Self {
        Self {
            challenges: RwLock::new(None),
        }
    }
}

impl ChallengeStoreInterface for ChallengeStore {
    fn get_challenges(&self) -> Option<Vec<Challenge>> {
        self.challenges.read().unwrap().clone()
    }

    fn set_challenges(&self, challenges: Vec<Challenge>) {
        *self.challenges.write().unwrap() = Some(challenges);
    }

    fn clear(&self) {
        *self.challenges.write().unwrap() = None;
    }

    fn take_challenges(&self) -> Option<Vec<Challenge>> {
        self.challenges.write().unwrap().take()
    }
}
