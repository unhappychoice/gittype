pub mod infrastructure;
pub mod cli;
pub mod config;
pub mod error;
pub mod extractor;
pub mod game;
pub mod logging;
pub mod models;
pub mod repository_manager;
pub mod scoring;
pub mod sharing;
pub mod signal_handler;
pub mod storage;
pub mod ui;
pub mod version;

pub use error::{GitTypeError, Result};
