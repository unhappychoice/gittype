use super::migrations::{get_all_migrations, get_latest_version};
use crate::{domain::error::GitTypeError, Result};
use rusqlite::Connection;
#[cfg(not(feature = "test-mocks"))]
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Database {
    connection: Connection,
}

impl Database {
    #[cfg(not(feature = "test-mocks"))]
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

    #[cfg(feature = "test-mocks")]
    pub fn new() -> Result<Self> {
        // Use in-memory database for tests
        let connection = Connection::open(":memory:")?;
        // Enable foreign key constraints
        connection.execute("PRAGMA foreign_keys = ON", [])?;
        let db = Self { connection };
        // Automatically initialize schema for tests
        db.init()?;
        Ok(db)
    }

    pub fn init(&self) -> Result<()> {
        self.init_tables()
    }

    #[cfg(not(feature = "test-mocks"))]
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

    pub fn get_current_schema_version(&self) -> Result<i32> {
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
