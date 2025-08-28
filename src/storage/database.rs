use rusqlite::Connection;
use crate::Result;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("gittype.db")?;
        Ok(Self { connection })
    }

    pub fn init_tables(&self) -> Result<()> {
        // TODO: Initialize database tables
        Ok(())
    }
}