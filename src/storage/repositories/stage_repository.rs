use super::super::{
    daos::{DifficultyStats, LanguageStats, StageDao, StageStatistics, StoredStageResult},
    Database, HasDatabase,
};
use crate::Result;
use std::sync::{Arc, Mutex};

/// Repository for stage-based business logic
pub struct StageRepository {
    database: Arc<Mutex<Database>>,
}

impl StageRepository {
    pub fn new() -> Result<Self> {
        let database = Database::new()?;
        Ok(Self {
            database: Arc::new(Mutex::new(database)),
        })
    }

    /// Get completed stages for a specific repository (excludes skipped/failed)
    pub fn get_completed_stages(
        &self,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        let db = self.db_with_lock()?;

        let dao = StageDao::new(&*db);
        dao.get_completed_stages(repository_id)
    }

    /// Get completed stages filtered by language
    pub fn get_completed_stages_by_language(
        &self,
        language: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        let db = self.db_with_lock()?;

        let dao = StageDao::new(&*db);
        dao.get_completed_stages_by_language(language, repository_id)
    }

    /// Get completed stages filtered by difficulty level
    pub fn get_completed_stages_by_difficulty(
        &self,
        difficulty: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        let db = self.db_with_lock()?;

        let dao = StageDao::new(&*db);
        dao.get_completed_stages_by_difficulty(difficulty, repository_id)
    }

    /// Get stage statistics for completed stages only
    pub fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics> {
        let db = self.db_with_lock()?;

        let dao = StageDao::new(&*db);
        dao.get_stage_statistics(repository_id)
    }

    /// Get language breakdown for completed stages
    pub fn get_language_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<LanguageStats>> {
        let db = self.db_with_lock()?;

        let dao = StageDao::new(&*db);
        dao.get_language_breakdown(repository_id)
    }

    /// Get difficulty breakdown for completed stages  
    pub fn get_difficulty_breakdown(
        &self,
        repository_id: Option<i64>,
    ) -> Result<Vec<DifficultyStats>> {
        let db = self.db_with_lock()?;

        let dao = StageDao::new(&*db);
        dao.get_difficulty_breakdown(repository_id)
    }
}

impl HasDatabase for StageRepository {
    fn database(&self) -> &Arc<Mutex<Database>> {
        &self.database
    }
}

impl Default for StageRepository {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            log::warn!("Failed to initialize StageRepository: {}", e);
            // Return a dummy repository that will fail gracefully
            Self {
                database: Arc::new(Mutex::new(
                    Database::new().expect("Failed to create fallback database"),
                )),
            }
        })
    }
}
