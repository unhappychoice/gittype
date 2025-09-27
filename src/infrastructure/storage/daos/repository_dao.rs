use super::super::database::Database;
use crate::domain::models::GitRepository;
use crate::domain::models::storage::{StoredRepository, StoredRepositoryWithLanguages};
use crate::{domain::error::GitTypeError, Result};
use rusqlite::{params, Transaction};

pub struct RepositoryDao<'a> {
    db: &'a Database,
}

impl<'a> RepositoryDao<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Get or create repository record
    pub fn ensure_repository(&self, git_repo: &GitRepository) -> Result<i64> {
        let conn = self.db.get_connection();
        let tx = conn.unchecked_transaction()?;
        let id = self.ensure_repository_in_transaction(&tx, git_repo)?;
        tx.commit()?;
        Ok(id)
    }

    /// Get or create repository record within an existing transaction
    pub fn ensure_repository_in_transaction(
        &self,
        tx: &Transaction,
        git_repo: &GitRepository,
    ) -> Result<i64> {
        // Try to find existing repository by user_name and repository_name only
        // (matching the UNIQUE constraint in the database schema)
        let existing = tx
            .prepare("SELECT id FROM repositories WHERE user_name = ? AND repository_name = ?")?
            .query_row(
                params![git_repo.user_name, git_repo.repository_name],
                |row| row.get::<_, i64>(0),
            );

        match existing {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Create new repository
                tx.execute(
                    "INSERT INTO repositories (user_name, repository_name, remote_url) VALUES (?, ?, ?)",
                    params![git_repo.user_name, git_repo.repository_name, git_repo.remote_url],
                )?;
                Ok(tx.last_insert_rowid())
            }
            Err(e) => Err(GitTypeError::database_error(format!(
                "Database error: {}",
                e
            ))),
        }
    }

    /// Get all repositories
    pub fn get_all_repositories(&self) -> Result<Vec<StoredRepository>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, user_name, repository_name, remote_url FROM repositories ORDER BY user_name, repository_name",
        )?;

        let repositories = stmt
            .query_map([], |row| {
                Ok(StoredRepository {
                    id: row.get(0)?,
                    user_name: row.get(1)?,
                    repository_name: row.get(2)?,
                    remote_url: row.get(3)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(repositories)
    }

    /// Get a specific repository by ID
    pub fn get_repository_by_id(&self, repository_id: i64) -> Result<Option<StoredRepository>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, user_name, repository_name, remote_url FROM repositories WHERE id = ?",
        )?;

        match stmt.query_row(params![repository_id], |row| {
            Ok(StoredRepository {
                id: row.get(0)?,
                user_name: row.get(1)?,
                repository_name: row.get(2)?,
                remote_url: row.get(3)?,
            })
        }) {
            Ok(repo) => Ok(Some(repo)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(GitTypeError::database_error(format!(
                "Database error: {}",
                e
            ))),
        }
    }

    /// Find repository by user and repository name
    pub fn find_repository(
        &self,
        user_name: &str,
        repository_name: &str,
    ) -> Result<Option<StoredRepository>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, user_name, repository_name, remote_url FROM repositories WHERE user_name = ? AND repository_name = ?",
        )?;

        match stmt.query_row(params![user_name, repository_name], |row| {
            Ok(StoredRepository {
                id: row.get(0)?,
                user_name: row.get(1)?,
                repository_name: row.get(2)?,
                remote_url: row.get(3)?,
            })
        }) {
            Ok(repo) => Ok(Some(repo)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(GitTypeError::database_error(format!(
                "Database error: {}",
                e
            ))),
        }
    }

    /// Get all repositories with their languages
    pub fn get_all_repositories_with_languages(
        &self,
    ) -> Result<Vec<StoredRepositoryWithLanguages>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT r.id, r.user_name, r.repository_name, r.remote_url, 
                    GROUP_CONCAT(DISTINCT sr.language) as languages
             FROM repositories r 
             LEFT JOIN sessions s ON r.id = s.repository_id
             LEFT JOIN stage_results sr ON s.id = sr.session_id
             GROUP BY r.id, r.user_name, r.repository_name, r.remote_url
             ORDER BY r.user_name, r.repository_name",
        )?;

        let repositories = stmt
            .query_map([], |row| {
                let languages_str: Option<String> = row.get(4)?;
                let languages = languages_str
                    .unwrap_or_default()
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.trim().to_string())
                    .collect();

                Ok(StoredRepositoryWithLanguages {
                    id: row.get(0)?,
                    user_name: row.get(1)?,
                    repository_name: row.get(2)?,
                    remote_url: row.get(3)?,
                    languages,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(repositories)
    }
}
