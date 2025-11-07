use std::sync::Arc;

use crate::domain::models::storage::{
    DifficultyStats, LanguageStats, StageStatistics, StoredStageResult,
};
use crate::infrastructure::database::daos::StageDaoInterface;
use crate::Result;

pub trait StageRepositoryTrait: shaku::Interface {
    fn get_completed_stages(&self, repository_id: Option<i64>) -> Result<Vec<StoredStageResult>>;
    fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics>;
    fn get_completed_stages_by_language(
        &self,
        language: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>>;
    fn get_completed_stages_by_difficulty(
        &self,
        difficulty: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>>;
    fn get_language_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<LanguageStats>>;
    fn get_difficulty_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<DifficultyStats>>;
}

/// Repository for stage-based business logic
#[derive(shaku::Component)]
#[shaku(interface = StageRepositoryTrait)]
pub struct StageRepository {
    #[shaku(inject)]
    stage_dao: Arc<dyn StageDaoInterface>,
}

impl StageRepositoryTrait for StageRepository {
    /// Get completed stages for a specific repository (excludes skipped/failed)
    fn get_completed_stages(&self, repository_id: Option<i64>) -> Result<Vec<StoredStageResult>> {
        self.stage_dao.get_completed_stages(repository_id)
    }

    /// Get stage statistics for completed stages only
    fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics> {
        self.stage_dao.get_stage_statistics(repository_id)
    }

    /// Get completed stages filtered by language
    fn get_completed_stages_by_language(
        &self,
        language: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        self.stage_dao
            .get_completed_stages_by_language(language, repository_id)
    }

    /// Get completed stages filtered by difficulty level
    fn get_completed_stages_by_difficulty(
        &self,
        difficulty: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        self.stage_dao
            .get_completed_stages_by_difficulty(difficulty, repository_id)
    }

    /// Get language breakdown for completed stages
    fn get_language_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<LanguageStats>> {
        self.stage_dao.get_language_breakdown(repository_id)
    }

    /// Get difficulty breakdown for completed stages
    fn get_difficulty_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<DifficultyStats>> {
        self.stage_dao.get_difficulty_breakdown(repository_id)
    }
}
