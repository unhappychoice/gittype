use super::Event;
use crate::domain::models::chunk::CodeChunk;
use crate::domain::models::rank::Rank;
use std::any::Any;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TypingStarted {
    pub challenge_id: i64,
    pub timestamp: Instant,
}

impl Event for TypingStarted {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct KeyPressed {
    pub key: char,
    pub timestamp: Instant,
}

impl Event for KeyPressed {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct ChallengeCompleted {
    pub challenge_id: i64,
    pub timestamp: Instant,
}

impl Event for ChallengeCompleted {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct StageCompleted {
    pub stage_id: i64,
    pub chunk: CodeChunk,
    pub timestamp: Instant,
}

impl Event for StageCompleted {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct SessionCompleted {
    pub session_id: i64,
    pub timestamp: Instant,
}

impl Event for SessionCompleted {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct ScoreCalculated {
    pub score: f64,
    pub accuracy: f64,
    pub wpm: f64,
    pub rank: Rank,
}

impl Event for ScoreCalculated {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct ErrorOccurred {
    pub message: String,
    pub timestamp: Instant,
}

impl Event for ErrorOccurred {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
