use super::migrations::{get_all_migrations, get_latest_version};
use crate::{domain::error::GitTypeError, Result};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_database_path()?;

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let connection = Connection::open(&db_path)?;
        // Enable foreign key constraints
        connection.execute("PRAGMA foreign_keys = ON", [])?;
        let db = Self { connection };
        Ok(db)
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Self> {
        // Use in-memory database for tests
        let connection = Connection::open(":memory:")?;
        // Enable foreign key constraints
        connection.execute("PRAGMA foreign_keys = ON", [])?;
        let db = Self { connection };
        Ok(db)
    }

    pub fn init(&self) -> Result<()> {
        self.init_tables()
    }

    fn get_database_path() -> Result<PathBuf> {
        if cfg!(test) {
            // Test: use in-memory database (shouldn't be called in tests)
            Ok(PathBuf::from(":memory:"))
        } else if cfg!(debug_assertions) {
            // Development: use project directory
            let current_dir = std::env::current_dir().map_err(|e| {
                GitTypeError::ExtractionFailed(format!("Could not get current directory: {}", e))
            })?;
            Ok(current_dir.join("gittype-dev.db"))
        } else {
            // Release: use home directory
            let home_dir = dirs::home_dir().ok_or_else(|| {
                GitTypeError::ExtractionFailed("Could not determine home directory".to_string())
            })?;
            Ok(home_dir.join(".gittype").join("gittype.db"))
        }
    }

    pub fn init_tables(&self) -> Result<()> {
        self.create_schema_version_table()?;
        self.run_migrations()?;
        Ok(())
    }

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }

    fn create_schema_version_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(())
    }

    fn get_current_schema_version(&self) -> Result<i32> {
        let version = self
            .connection
            .prepare("SELECT MAX(version) FROM schema_version")?
            .query_row([], |row| {
                let version: Option<i32> = row.get(0)?;
                Ok(version.unwrap_or(0))
            })?;
        Ok(version)
    }

    fn set_schema_version(&self, version: i32) -> Result<()> {
        self.connection
            .execute("INSERT INTO schema_version (version) VALUES (?)", [version])?;
        Ok(())
    }

    fn run_migrations(&self) -> Result<()> {
        let current_version = self.get_current_schema_version()?;
        let latest_version = get_latest_version();

        if current_version < latest_version {
            let migrations = get_all_migrations();

            for migration in migrations {
                let version = migration.version();
                if version > current_version {
                    migration.up(&self.connection)?;
                    self.set_schema_version(version)?;
                }
            }
        }

        Ok(())
    }
}

pub trait HasDatabase {
    fn database(&self) -> &Arc<Mutex<Database>>;

    fn db_with_lock(&self) -> Result<MutexGuard<'_, Database>> {
        self.database()
            .lock()
            .map_err(|e| GitTypeError::database_error(format!("Failed to acquire lock: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let result = Database::new_test();
        assert!(result.is_ok());

        let db = result.unwrap();
        db.init().expect("Failed to initialize test database");
        assert!(db.get_connection().prepare("SELECT 1").is_ok());
    }

    #[test]
    fn test_tables_creation() {
        let db = Database::new_test().unwrap();
        db.init().expect("Failed to initialize test database");
        let conn = db.get_connection();

        // Check schema_version table
        let count: i32 = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='schema_version'",
            )
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        // Check repositories table
        let count: i32 = conn
            .prepare(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='repositories'",
            )
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        // Check sessions table
        let count: i32 = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sessions'")
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        // Check stages table
        let count: i32 = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='stages'")
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_schema_versioning() {
        let db = Database::new_test().unwrap();
        db.init().expect("Failed to initialize test database");

        // Check that schema version is set correctly
        let version = db.get_current_schema_version().unwrap();
        assert_eq!(version, get_latest_version());

        // Check that schema_version table has the correct entry
        let conn = db.get_connection();
        let count: i32 = conn
            .prepare("SELECT COUNT(*) FROM schema_version WHERE version = ?")
            .unwrap()
            .query_row([get_latest_version()], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_migration_idempotency() {
        // Create database twice
        let db1 = Database::new_test().unwrap();
        db1.init()
            .expect("Failed to initialize first test database");

        let db2 = Database::new_test().unwrap();
        db2.init()
            .expect("Failed to initialize second test database");

        // Schema version should still be correct
        let version = db2.get_current_schema_version().unwrap();
        assert_eq!(version, get_latest_version());

        // Should have the correct number of version entries (one per migration)
        let conn = db2.get_connection();
        let count: i32 = conn
            .prepare("SELECT COUNT(*) FROM schema_version")
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert_eq!(count, get_latest_version());
    }

    #[test]
    fn test_normalized_tables_structure() {
        let db = Database::new_test().unwrap();
        db.init().expect("Failed to initialize test database");
        let conn = db.get_connection();

        // Check all tables exist
        let tables = vec![
            "repositories",
            "sessions",
            "session_results",
            "challenges",
            "stages",
            "stage_results",
            "schema_version",
        ];
        for table in tables {
            let count: i32 = conn
                .prepare(&format!(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'",
                    table
                ))
                .unwrap()
                .query_row([], |row| row.get(0))
                .unwrap();
            assert_eq!(count, 1, "Table {} should exist", table);
        }

        // Verify sessions table structure (basic info only)
        let mut stmt = conn.prepare("PRAGMA table_info(sessions)").unwrap();
        let column_names: Vec<String> = stmt
            .query_map([], |row| {
                row.get::<_, String>(1) // column name is at index 1
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        assert!(column_names.contains(&"id".to_string()));
        assert!(column_names.contains(&"repository_id".to_string()));
        assert!(column_names.contains(&"started_at".to_string()));
        assert!(column_names.contains(&"completed_at".to_string()));
        assert!(column_names.contains(&"branch".to_string()));
        assert!(column_names.contains(&"commit_hash".to_string()));
        assert!(column_names.contains(&"is_dirty".to_string()));
        assert!(column_names.contains(&"game_mode".to_string()));
        assert!(column_names.contains(&"difficulty_level".to_string()));
        // Should NOT contain metrics fields (moved to typing_metrics)
        assert!(!column_names.contains(&"keystrokes".to_string()));
        assert!(!column_names.contains(&"mistakes".to_string()));
        assert!(!column_names.contains(&"wpm".to_string()));
        assert!(!column_names.contains(&"cpm".to_string()));

        // Verify session_results table exists with result fields
        let mut stmt = conn.prepare("PRAGMA table_info(session_results)").unwrap();
        let column_names: Vec<String> = stmt
            .query_map([], |row| {
                row.get::<_, String>(1) // column name is at index 1
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        assert!(column_names.contains(&"session_id".to_string()));
        assert!(column_names.contains(&"repository_id".to_string()));
        assert!(column_names.contains(&"keystrokes".to_string()));
        assert!(column_names.contains(&"mistakes".to_string()));
        assert!(column_names.contains(&"duration_ms".to_string()));
        assert!(column_names.contains(&"wpm".to_string()));
        assert!(column_names.contains(&"accuracy".to_string()));
        assert!(column_names.contains(&"stages_completed".to_string()));
        assert!(column_names.contains(&"game_mode".to_string()));
        assert!(column_names.contains(&"difficulty_level".to_string()));
        assert!(column_names.contains(&"score".to_string()));
    }

    #[test]
    fn test_foreign_key_constraints() {
        let db = Database::new_test().unwrap();
        db.init().expect("Failed to initialize test database");
        let conn = db.get_connection();

        // Check foreign keys are enabled
        let fk_enabled: i32 = conn
            .prepare("PRAGMA foreign_keys")
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert_eq!(fk_enabled, 1);

        // Test that foreign key constraint works
        let result = conn.execute(
            "INSERT INTO sessions (id, repository_id, started_at, game_mode) 
             VALUES (1, 999, datetime('now'), 'Normal')",
            [],
        );
        assert!(result.is_err(), "Should fail due to foreign key constraint");
    }

    #[test]
    fn test_indexes_created() {
        let db = Database::new_test().unwrap();
        db.init().expect("Failed to initialize test database");
        let conn = db.get_connection();

        // Check that indexes were created
        let index_count: i32 = conn
            .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'")
            .unwrap()
            .query_row([], |row| row.get(0))
            .unwrap();
        assert!(index_count >= 5, "Should have at least 5 custom indexes");
    }
}
