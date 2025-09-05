pub mod v001_initial_schema;

use crate::Result;
use rusqlite::Connection;

pub trait Migration {
    fn version(&self) -> i32;
    fn description(&self) -> &str;
    fn up(&self, conn: &Connection) -> Result<()>;
}

pub fn get_all_migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(v001_initial_schema::InitialSchema)]
}

pub fn get_latest_version() -> i32 {
    get_all_migrations()
        .iter()
        .map(|m| m.version())
        .max()
        .unwrap_or(0)
}
