pub mod calculator;
pub mod rank_calculator;
pub mod score_calculator;
pub mod tracker;

pub use crate::models::{Rank, RankTier, SessionResult, StageResult, TotalResult};
pub use calculator::{
    RealTimeCalculator, RealTimeResult, SessionCalculator, StageCalculator, TotalCalculator,
};
pub use rank_calculator::RankCalculator;
pub use score_calculator::ScoreCalculator;
pub use tracker::{
    Keystroke, SessionTracker, SessionTrackerData, StageInput, StageTracker, StageTrackerData,
    TotalTracker, TotalTrackerData,
};

// Export global tracker instances
pub use tracker::session::GLOBAL_SESSION_TRACKER;
pub use tracker::total::GLOBAL_TOTAL_TRACKER;
