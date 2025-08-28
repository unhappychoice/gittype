use crate::Result;
use super::screens::TypingScreen;

pub struct GameSession {
    challenge_text: String,
}

impl GameSession {
    pub fn new(challenge_text: String) -> Self {
        Self { challenge_text }
    }

    pub fn start(&self) -> Result<()> {
        let mut screen = TypingScreen::new(self.challenge_text.clone());
        screen.run_full_session()
    }
}
