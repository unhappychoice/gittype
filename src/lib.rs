pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod error;
pub mod logging;
pub mod repository_manager;
pub mod sharing;
pub mod signal_handler;


pub use error::{GitTypeError, Result};
