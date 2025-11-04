use super::super::database::DatabaseInterface;
use crate::domain::models::Challenge;
use crate::{domain::error::GitTypeError, Result};
use rusqlite::{params, Transaction};
use serde_json;
use shaku::{Component, Interface};
use std::sync::Arc;

pub trait ChallengeDaoInterface: Interface {
    fn ensure_challenge_in_transaction(
        &self,
        tx: &Transaction,
        challenge: &Challenge,
    ) -> Result<i64>;
}

#[derive(Component)]
#[shaku(interface = ChallengeDaoInterface)]
pub struct ChallengeDao {
    #[allow(dead_code)]
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

impl ChallengeDao {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
        Self { db }
    }
}

impl ChallengeDaoInterface for ChallengeDao {
    /// Get or create challenge record within an existing transaction
    fn ensure_challenge_in_transaction(
        &self,
        tx: &Transaction,
        challenge: &Challenge,
    ) -> Result<i64> {
        // Try to find existing challenge
        let existing = tx
            .prepare("SELECT rowid FROM challenges WHERE id = ?")?
            .query_row(params![challenge.id], |row| row.get::<_, i64>(0));

        match existing {
            Ok(rowid) => Ok(rowid),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Create new challenge
                tx.execute(
                    "INSERT INTO challenges (id, file_path, start_line, end_line, language, code_content, comment_ranges, difficulty_level) 
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                    params![
                        challenge.id,
                        challenge.source_file_path,
                        challenge.start_line,
                        challenge.end_line,
                        challenge.language,
                        challenge.code_content,
                        serde_json::to_string(&challenge.comment_ranges).unwrap_or_default(),
                        challenge.difficulty_level.as_ref().map(|d| format!("{:?}", d))
                    ],
                )?;
                Ok(tx.last_insert_rowid())
            }
            Err(e) => Err(GitTypeError::database_error(format!(
                "Database error: {}",
                e
            ))),
        }
    }
}
