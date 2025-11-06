use crate::domain::services::scoring::StageResult;

#[derive(Debug, Clone)]
pub enum SessionAction {
    Start,
    CompleteStage(StageResult),
    Complete,
    Abort,
    Reset,
}
