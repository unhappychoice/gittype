use crate::domain::models::storage::{
    DifficultyStats, LanguageStats, StageStatistics, StoredStageResult,
};
use crate::infrastructure::database::daos::StageDao;
use crate::infrastructure::database::database::{Database, DatabaseInterface, HasDatabase};
use crate::Result;
use std::sync::Arc;

pub trait StageRepositoryTrait: shaku::Interface {
    fn get_completed_stages(&self, repository_id: Option<i64>) -> Result<Vec<StoredStageResult>>;
    fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics>;
}

/// Repository for stage-based business logic
pub struct StageRepository {
    database: Arc<dyn DatabaseInterface>,
}

impl shaku::Component<crate::presentation::di::AppModule> for StageRepository {
    type Interface = dyn StageRepositoryTrait;
    type Parameters = ();

    fn build(
        _context: &mut shaku::ModuleBuildContext<crate::presentation::di::AppModule>,
        _params: Self::Parameters,
    ) -> Box<dyn StageRepositoryTrait> {
        Box::new(StageRepository::default())
    }
}

impl StageRepositoryTrait for StageRepository {
    fn get_completed_stages(&self, repository_id: Option<i64>) -> Result<Vec<StoredStageResult>> {
        StageRepository::get_completed_stages(self, repository_id)
    }

    fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics> {
        StageRepository::get_stage_statistics(self, repository_id)
    }
}

impl StageRepository {
    pub fn new() -> Result<Self> {
        let database = Database::new()?;
        Ok(Self {
            database: Arc::new(database) as Arc<dyn DatabaseInterface>,
        })
    }

    /// Get completed stages for a specific repository (excludes skipped/failed)
    pub fn get_completed_stages(
        &self,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        

        let dao = StageDao::new(Arc::clone(&self.database));
        dao.get_completed_stages(repository_id)
    }

    /// Get completed stages filtered by language
    pub fn get_completed_stages_by_language(
        &self,
        language: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        

        let dao = StageDao::new(Arc::clone(&self.database));
        dao.get_completed_stages_by_language(language, repository_id)
    }

    /// Get completed stages filtered by difficulty level
    pub fn get_completed_stages_by_difficulty(
        &self,
        difficulty: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        

        let dao = StageDao::new(Arc::clone(&self.database));
        dao.get_completed_stages_by_difficulty(difficulty, repository_id)
    }

    /// Get stage statistics for completed stages only
    pub fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics> {
        

        let dao = StageDao::new(Arc::clone(&self.database));
        dao.get_stage_statistics(repository_id)
    }

    /// Get language breakdown for completed stages
    pub fn get_language_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<LanguageStats>> {
        

        let dao = StageDao::new(Arc::clone(&self.database));
        dao.get_language_breakdown(repository_id)
    }

    /// Get difficulty breakdown for completed stages  
    pub fn get_difficulty_breakdown(
        &self,
        repository_id: Option<i64>,
    ) -> Result<Vec<DifficultyStats>> {
        

        let dao = StageDao::new(Arc::clone(&self.database));
        dao.get_difficulty_breakdown(repository_id)
    }
}

impl HasDatabase for StageRepository {
    fn database(&self) -> &Arc<dyn DatabaseInterface> {
        &self.database
    }
}

impl Default for StageRepository {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            log::warn!("Failed to initialize StageRepository: {}", e);
            // Return a dummy repository that will fail gracefully
            Self {
                database: Arc::new(
                    Database::new().expect("Failed to create fallback database"),
                ) as Arc<dyn DatabaseInterface>,
            }
        })
    }
}
