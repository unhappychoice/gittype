pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod error;
pub mod extractor;
pub mod game;
pub mod logging;
pub mod repository_manager;
pub mod scoring;
pub mod sharing;
pub mod signal_handler;
pub mod storage;
pub mod ui;

pub use error::{GitTypeError, Result};
