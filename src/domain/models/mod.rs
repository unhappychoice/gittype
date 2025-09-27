pub mod challenge;
pub mod chunk;
pub mod color_mode;
pub mod color_scheme;
pub mod config;
pub mod countdown;
pub mod extraction_options;
pub mod git_repository;
pub mod language;
pub mod languages;
pub mod rank;
pub mod session;
pub mod stage;
pub mod theme;
pub mod total;
pub mod version;

// Re-export main types for easy access
pub use challenge::Challenge;
pub use chunk::{ChunkType, CodeChunk};
pub use countdown::Countdown;
pub use extraction_options::ExtractionOptions;
pub use git_repository::GitRepository;
pub use language::Language;
pub use rank::{Rank, RankTier};
pub use session::{Session, SessionResult};
pub use stage::{Stage, StageResult};
pub use total::{Total, TotalResult};
