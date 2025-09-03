pub mod chunk;
pub mod challenge;
pub mod stage;
pub mod session;
pub mod total;
pub mod rank;
pub mod git_repository;

// Re-export main types for easy access
pub use chunk::{ChunkType, CodeChunk};
pub use challenge::Challenge;
pub use stage::{Stage, StageResult};
pub use session::{Session, SessionResult};
pub use total::{Total, TotalResult};
pub use rank::{Rank, RankingTitle};
pub use git_repository::GitRepository;