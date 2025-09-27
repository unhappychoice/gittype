use super::super::database::Database;
use crate::domain::models::Challenge;
use crate::{domain::error::GitTypeError, Result};
use rusqlite::{params, Transaction};
use serde_json;

pub struct ChallengeDao<'a> {
    #[allow(dead_code)]
    db: &'a Database,
}

impl<'a> ChallengeDao<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Get or create challenge record within an existing transaction
    pub fn ensure_challenge_in_transaction(
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
