use chrono::{DateTime, Utc};
use rusqlite::params;
use shaku::{Component, Interface};

use std::sync::Arc;

use crate::domain::error::GitTypeError;
use crate::domain::models::storage::{
    DifficultyStats, LanguageStats, StageStatistics, StoredStageResult,
};
use crate::Result;

use super::super::database::DatabaseInterface;

pub trait StageDaoInterface: Interface {
    fn get_completed_stages(&self, repository_id: Option<i64>) -> Result<Vec<StoredStageResult>>;
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
    fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics>;
    fn get_language_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<LanguageStats>>;
    fn get_difficulty_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<DifficultyStats>>;
}

#[derive(Component)]
#[shaku(interface = StageDaoInterface)]
pub struct StageDao {
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

impl StageDao {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
        Self { db }
    }
}

impl StageDaoInterface for StageDao {
    /// Get completed stages for a specific repository (excludes skipped/failed)
    fn get_completed_stages(&self, repository_id: Option<i64>) -> Result<Vec<StoredStageResult>> {
        let conn = self.db.get_connection()?;

        let query = if repository_id.is_some() {
            "SELECT sr.id, sr.repository_id, r.repository_name, r.user_name,
                    sr.wpm, sr.cpm, sr.accuracy, sr.keystrokes, sr.mistakes, 
                    sr.duration_ms, sr.score, sr.language, sr.difficulty_level,
                    sr.completed_at, sr.rank_name, sr.tier_name
             FROM stage_results sr
             LEFT JOIN repositories r ON sr.repository_id = r.id
             WHERE sr.repository_id = ? AND sr.was_skipped = 0 AND sr.was_failed = 0
             ORDER BY sr.completed_at DESC"
        } else {
            "SELECT sr.id, sr.repository_id, r.repository_name, r.user_name,
                    sr.wpm, sr.cpm, sr.accuracy, sr.keystrokes, sr.mistakes, 
                    sr.duration_ms, sr.score, sr.language, sr.difficulty_level,
                    sr.completed_at, sr.rank_name, sr.tier_name
             FROM stage_results sr
             LEFT JOIN repositories r ON sr.repository_id = r.id
             WHERE sr.was_skipped = 0 AND sr.was_failed = 0
             ORDER BY sr.completed_at DESC"
        };

        let mut stmt = conn.prepare(query)?;

        let stages = if let Some(repo_id) = repository_id {
            let stage_iter =
                stmt.query_map(params![repo_id], |row| self.map_stage_result_row(row))?;
            stage_iter.collect::<std::result::Result<Vec<_>, _>>()?
        } else {
            let stage_iter = stmt.query_map([], |row| self.map_stage_result_row(row))?;
            stage_iter.collect::<std::result::Result<Vec<_>, _>>()?
        };
        Ok(stages)
    }

    /// Get completed stages filtered by language
    fn get_completed_stages_by_language(
        &self,
        language: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        let conn = self.db.get_connection()?;

        let query = if repository_id.is_some() {
            "SELECT sr.id, sr.repository_id, r.repository_name, r.user_name,
                    sr.wpm, sr.cpm, sr.accuracy, sr.keystrokes, sr.mistakes, 
                    sr.duration_ms, sr.score, sr.language, sr.difficulty_level,
                    sr.completed_at, sr.rank_name, sr.tier_name
             FROM stage_results sr
             LEFT JOIN repositories r ON sr.repository_id = r.id
             WHERE sr.repository_id = ? AND sr.language = ? 
                   AND sr.was_skipped = 0 AND sr.was_failed = 0
             ORDER BY sr.completed_at DESC"
        } else {
            "SELECT sr.id, sr.repository_id, r.repository_name, r.user_name,
                    sr.wpm, sr.cpm, sr.accuracy, sr.keystrokes, sr.mistakes, 
                    sr.duration_ms, sr.score, sr.language, sr.difficulty_level,
                    sr.completed_at, sr.rank_name, sr.tier_name
             FROM stage_results sr
             LEFT JOIN repositories r ON sr.repository_id = r.id
             WHERE sr.language = ? AND sr.was_skipped = 0 AND sr.was_failed = 0
             ORDER BY sr.completed_at DESC"
        };

        let mut stmt = conn.prepare(query)?;

        let stages = if let Some(repo_id) = repository_id {
            let stage_iter = stmt.query_map(params![repo_id, language], |row| {
                self.map_stage_result_row(row)
            })?;
            stage_iter.collect::<std::result::Result<Vec<_>, _>>()?
        } else {
            let stage_iter =
                stmt.query_map(params![language], |row| self.map_stage_result_row(row))?;
            stage_iter.collect::<std::result::Result<Vec<_>, _>>()?
        };
        Ok(stages)
    }

    /// Get completed stages filtered by difficulty level
    fn get_completed_stages_by_difficulty(
        &self,
        difficulty: &str,
        repository_id: Option<i64>,
    ) -> Result<Vec<StoredStageResult>> {
        let conn = self.db.get_connection()?;

        let query = if repository_id.is_some() {
            "SELECT sr.id, sr.repository_id, r.repository_name, r.user_name,
                    sr.wpm, sr.cpm, sr.accuracy, sr.keystrokes, sr.mistakes, 
                    sr.duration_ms, sr.score, sr.language, sr.difficulty_level,
                    sr.completed_at, sr.rank_name, sr.tier_name
             FROM stage_results sr
             LEFT JOIN repositories r ON sr.repository_id = r.id
             WHERE sr.repository_id = ? AND sr.difficulty_level = ? 
                   AND sr.was_skipped = 0 AND sr.was_failed = 0
             ORDER BY sr.completed_at DESC"
        } else {
            "SELECT sr.id, sr.repository_id, r.repository_name, r.user_name,
                    sr.wpm, sr.cpm, sr.accuracy, sr.keystrokes, sr.mistakes, 
                    sr.duration_ms, sr.score, sr.language, sr.difficulty_level,
                    sr.completed_at, sr.rank_name, sr.tier_name
             FROM stage_results sr
             LEFT JOIN repositories r ON sr.repository_id = r.id
             WHERE sr.difficulty_level = ? AND sr.was_skipped = 0 AND sr.was_failed = 0
             ORDER BY sr.completed_at DESC"
        };

        let mut stmt = conn.prepare(query)?;

        let stages = if let Some(repo_id) = repository_id {
            let stage_iter = stmt.query_map(params![repo_id, difficulty], |row| {
                self.map_stage_result_row(row)
            })?;
            stage_iter.collect::<std::result::Result<Vec<_>, _>>()?
        } else {
            let stage_iter =
                stmt.query_map(params![difficulty], |row| self.map_stage_result_row(row))?;
            stage_iter.collect::<std::result::Result<Vec<_>, _>>()?
        };
        Ok(stages)
    }

    /// Get stage statistics for completed stages only
    fn get_stage_statistics(&self, repository_id: Option<i64>) -> Result<StageStatistics> {
        let conn = self.db.get_connection()?;

        let query = if repository_id.is_some() {
            "SELECT 
                COUNT(*) as total_completed,
                AVG(wpm) as avg_wpm,
                MAX(wpm) as max_wpm,
                MIN(wpm) as min_wpm,
                AVG(accuracy) as avg_accuracy,
                MAX(accuracy) as max_accuracy,
                MIN(accuracy) as min_accuracy,
                AVG(score) as avg_score,
                SUM(keystrokes) as total_keystrokes,
                SUM(mistakes) as total_mistakes
             FROM stage_results 
             WHERE repository_id = ? AND was_skipped = 0 AND was_failed = 0"
        } else {
            "SELECT 
                COUNT(*) as total_completed,
                AVG(wpm) as avg_wpm,
                MAX(wpm) as max_wpm,
                MIN(wpm) as min_wpm,
                AVG(accuracy) as avg_accuracy,
                MAX(accuracy) as max_accuracy,
                MIN(accuracy) as min_accuracy,
                AVG(score) as avg_score,
                SUM(keystrokes) as total_keystrokes,
                SUM(mistakes) as total_mistakes
             FROM stage_results 
             WHERE was_skipped = 0 AND was_failed = 0"
        };

        let mut stmt = conn.prepare(query)?;

        let stats = if let Some(repo_id) = repository_id {
            stmt.query_row(params![repo_id], |row| {
                Ok(StageStatistics {
                    total_completed: row.get(0)?,
                    avg_wpm: row.get::<_, Option<f64>>(1)?.unwrap_or(0.0),
                    max_wpm: row.get::<_, Option<f64>>(2)?.unwrap_or(0.0),
                    min_wpm: row.get::<_, Option<f64>>(3)?.unwrap_or(0.0),
                    avg_accuracy: row.get::<_, Option<f64>>(4)?.unwrap_or(0.0),
                    max_accuracy: row.get::<_, Option<f64>>(5)?.unwrap_or(0.0),
                    min_accuracy: row.get::<_, Option<f64>>(6)?.unwrap_or(0.0),
                    avg_score: row.get::<_, Option<f64>>(7)?.unwrap_or(0.0),
                    total_keystrokes: row.get::<_, Option<i64>>(8)?.unwrap_or(0),
                    total_mistakes: row.get::<_, Option<i64>>(9)?.unwrap_or(0),
                })
            })?
        } else {
            stmt.query_row([], |row| {
                Ok(StageStatistics {
                    total_completed: row.get(0)?,
                    avg_wpm: row.get::<_, Option<f64>>(1)?.unwrap_or(0.0),
                    max_wpm: row.get::<_, Option<f64>>(2)?.unwrap_or(0.0),
                    min_wpm: row.get::<_, Option<f64>>(3)?.unwrap_or(0.0),
                    avg_accuracy: row.get::<_, Option<f64>>(4)?.unwrap_or(0.0),
                    max_accuracy: row.get::<_, Option<f64>>(5)?.unwrap_or(0.0),
                    min_accuracy: row.get::<_, Option<f64>>(6)?.unwrap_or(0.0),
                    avg_score: row.get::<_, Option<f64>>(7)?.unwrap_or(0.0),
                    total_keystrokes: row.get::<_, Option<i64>>(8)?.unwrap_or(0),
                    total_mistakes: row.get::<_, Option<i64>>(9)?.unwrap_or(0),
                })
            })?
        };

        Ok(stats)
    }

    /// Get language breakdown for completed stages
    fn get_language_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<LanguageStats>> {
        let conn = self.db.get_connection()?;

        let query = if repository_id.is_some() {
            "SELECT 
                language,
                COUNT(*) as stage_count,
                AVG(wpm) as avg_wpm,
                AVG(accuracy) as avg_accuracy,
                AVG(score) as avg_score
             FROM stage_results 
             WHERE repository_id = ? AND was_skipped = 0 AND was_failed = 0 AND language IS NOT NULL
             GROUP BY language
             ORDER BY stage_count DESC"
        } else {
            "SELECT 
                language,
                COUNT(*) as stage_count,
                AVG(wpm) as avg_wpm,
                AVG(accuracy) as avg_accuracy,
                AVG(score) as avg_score
             FROM stage_results 
             WHERE was_skipped = 0 AND was_failed = 0 AND language IS NOT NULL
             GROUP BY language
             ORDER BY stage_count DESC"
        };

        let mut stmt = conn.prepare(query)?;

        let languages = if let Some(repo_id) = repository_id {
            let language_iter = stmt.query_map(params![repo_id], |row| {
                Ok(LanguageStats {
                    language: row.get(0)?,
                    stage_count: row.get(1)?,
                    avg_wpm: row.get(2)?,
                    avg_accuracy: row.get(3)?,
                    avg_score: row.get(4)?,
                })
            })?;
            language_iter.collect::<std::result::Result<Vec<_>, _>>()?
        } else {
            let language_iter = stmt.query_map([], |row| {
                Ok(LanguageStats {
                    language: row.get(0)?,
                    stage_count: row.get(1)?,
                    avg_wpm: row.get(2)?,
                    avg_accuracy: row.get(3)?,
                    avg_score: row.get(4)?,
                })
            })?;
            language_iter.collect::<std::result::Result<Vec<_>, _>>()?
        };
        Ok(languages)
    }

    /// Get difficulty breakdown for completed stages
    fn get_difficulty_breakdown(&self, repository_id: Option<i64>) -> Result<Vec<DifficultyStats>> {
        let conn = self.db.get_connection()?;

        let query = if repository_id.is_some() {
            "SELECT 
                difficulty_level,
                COUNT(*) as stage_count,
                AVG(wpm) as avg_wpm,
                AVG(accuracy) as avg_accuracy,
                AVG(score) as avg_score
             FROM stage_results 
             WHERE repository_id = ? AND was_skipped = 0 AND was_failed = 0 AND difficulty_level IS NOT NULL
             GROUP BY difficulty_level
             ORDER BY 
                CASE difficulty_level 
                    WHEN 'Easy' THEN 1
                    WHEN 'Normal' THEN 2
                    WHEN 'Hard' THEN 3
                    WHEN 'Wild' THEN 4
                    WHEN 'Zen' THEN 5
                    ELSE 6
                END"
        } else {
            "SELECT 
                difficulty_level,
                COUNT(*) as stage_count,
                AVG(wpm) as avg_wpm,
                AVG(accuracy) as avg_accuracy,
                AVG(score) as avg_score
             FROM stage_results 
             WHERE was_skipped = 0 AND was_failed = 0 AND difficulty_level IS NOT NULL
             GROUP BY difficulty_level
             ORDER BY 
                CASE difficulty_level 
                    WHEN 'Easy' THEN 1
                    WHEN 'Normal' THEN 2
                    WHEN 'Hard' THEN 3
                    WHEN 'Wild' THEN 4
                    WHEN 'Zen' THEN 5
                    ELSE 6
                END"
        };

        let mut stmt = conn.prepare(query)?;

        let difficulties = if let Some(repo_id) = repository_id {
            let difficulty_iter = stmt.query_map(params![repo_id], |row| {
                Ok(DifficultyStats {
                    difficulty_level: row.get(0)?,
                    stage_count: row.get(1)?,
                    avg_wpm: row.get(2)?,
                    avg_accuracy: row.get(3)?,
                    avg_score: row.get(4)?,
                })
            })?;
            difficulty_iter.collect::<std::result::Result<Vec<_>, _>>()?
        } else {
            let difficulty_iter = stmt.query_map([], |row| {
                Ok(DifficultyStats {
                    difficulty_level: row.get(0)?,
                    stage_count: row.get(1)?,
                    avg_wpm: row.get(2)?,
                    avg_accuracy: row.get(3)?,
                    avg_score: row.get(4)?,
                })
            })?;
            difficulty_iter.collect::<std::result::Result<Vec<_>, _>>()?
        };
        Ok(difficulties)
    }
}

impl StageDao {
    /// Helper method to map a row to StoredStageResult
    fn map_stage_result_row(
        &self,
        row: &rusqlite::Row,
    ) -> std::result::Result<StoredStageResult, rusqlite::Error> {
        let timestamp: String = row.get(13)?;
        let completed_at = Self::parse_sqlite_timestamp(&timestamp).unwrap_or_else(|_| Utc::now());

        Ok(StoredStageResult {
            id: row.get(0)?,
            repository_id: row.get(1)?,
            repository_name: row.get(2)?,
            user_name: row.get(3)?,
            wpm: row.get(4)?,
            cpm: row.get(5)?,
            accuracy: row.get(6)?,
            keystrokes: row.get(7)?,
            mistakes: row.get(8)?,
            duration_ms: row.get(9)?,
            score: row.get(10)?,
            language: row.get(11)?,
            difficulty_level: row.get(12)?,
            completed_at,
            rank_name: row.get(14)?,
            tier_name: row.get(15)?,
        })
    }

    /// Parse SQLite timestamp string to DateTime<Utc>
    fn parse_sqlite_timestamp(timestamp: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(timestamp)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| GitTypeError::database_error(format!("Failed to parse timestamp: {}", e)))
    }
}
