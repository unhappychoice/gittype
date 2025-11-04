use super::migrations::{get_all_migrations, get_latest_version};
use crate::{domain::error::GitTypeError, Result};
use rusqlite::Connection;
use shaku::Interface;
#[cfg(not(feature = "test-mocks"))]
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

pub trait DatabaseInterface: Interface {
    fn get_connection(&self) -> Result<MutexGuard<'_, Connection>>;
    fn init_tables(&self) -> Result<()>;
    fn get_current_schema_version(&self) -> Result<i32>;
}

pub struct Database {
    connection: Mutex<Connection>,
}

impl shaku::Component<crate::presentation::di::AppModule> for Database {
    type Interface = dyn DatabaseInterface;
    type Parameters = ();

    fn build(
        _context: &mut shaku::ModuleBuildContext<crate::presentation::di::AppModule>,
        _params: Self::Parameters,
    ) -> Box<dyn DatabaseInterface> {
        Box::new(Database::default())
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new().expect("Failed to initialize database")
    }
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
        let db = Self {
            connection: Mutex::new(connection),
        };
        Ok(db)
    }

    #[cfg(feature = "test-mocks")]
    pub fn new() -> Result<Self> {
        // Use in-memory database for tests
        let connection = Connection::open(":memory:")?;
        // Enable foreign key constraints
        connection.execute("PRAGMA foreign_keys = ON", [])?;
        let db = Self {
            connection: Mutex::new(connection),
        };
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

    pub fn with_connection<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&Connection) -> Result<R>,
    {
        let conn = self
            .connection
            .lock()
            .map_err(|e| GitTypeError::database_error(format!("Failed to acquire lock: {}", e)))?;
        f(&conn)
    }

    pub fn get_connection(&self) -> Result<MutexGuard<'_, Connection>> {
        self.connection
            .lock()
            .map_err(|e| GitTypeError::database_error(format!("Failed to acquire lock: {}", e)))
    }

    fn create_schema_version_table(&self) -> Result<()> {
        self.with_connection(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS schema_version (
                    version INTEGER PRIMARY KEY,
                    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )?;
            Ok(())
        })
    }

    pub fn get_current_schema_version(&self) -> Result<i32> {
        self.with_connection(|conn| {
            let version = conn
                .prepare("SELECT MAX(version) FROM schema_version")?
                .query_row([], |row| {
                    let version: Option<i32> = row.get(0)?;
                    Ok(version.unwrap_or(0))
                })?;
            Ok(version)
        })
    }

    fn set_schema_version(&self, version: i32) -> Result<()> {
        self.with_connection(|conn| {
            conn.execute("INSERT INTO schema_version (version) VALUES (?)", [version])?;
            Ok(())
        })
    }

    fn run_migrations(&self) -> Result<()> {
        let current_version = self.get_current_schema_version()?;
        let latest_version = get_latest_version();

        if current_version < latest_version {
            let migrations = get_all_migrations();

            for migration in migrations {
                let version = migration.version();
                if version > current_version {
                    self.with_connection(|conn| migration.up(conn))?;
                    self.set_schema_version(version)?;
                }
            }
        }

        Ok(())
    }
}

impl DatabaseInterface for Database {
    fn get_connection(&self) -> Result<MutexGuard<'_, Connection>> {
        self.get_connection()
    }

    fn init_tables(&self) -> Result<()> {
        self.init_tables()
    }

    fn get_current_schema_version(&self) -> Result<i32> {
        self.get_current_schema_version()
    }
}

pub trait HasDatabase {
    fn database(&self) -> &Arc<dyn DatabaseInterface>;
}
