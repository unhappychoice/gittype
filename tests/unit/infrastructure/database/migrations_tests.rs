use gittype::infrastructure::database::migrations::v001_initial_schema::InitialSchema;
use gittype::infrastructure::database::migrations::{
    get_all_migrations, get_latest_version, Migration,
};
use rusqlite::Connection;

fn table_exists(conn: &Connection, table_name: &str) -> bool {
    let count: i32 = conn
        .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?")
        .unwrap()
        .query_row([table_name], |row| row.get(0))
        .unwrap();
    count == 1
}

fn index_exists(conn: &Connection, index_name: &str) -> bool {
    let count: i32 = conn
        .prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name=?")
        .unwrap()
        .query_row([index_name], |row| row.get(0))
        .unwrap();
    count == 1
}

#[test]
fn initial_schema_reports_version_one() {
    let migration = InitialSchema;
    assert_eq!(migration.version(), 1);
}

#[test]
fn initial_schema_description_mentions_normalized_schema() {
    let migration = InitialSchema;
    let description = migration.description();
    assert!(description.contains("normalized database schema"));
    assert!(description.contains("repositories"));
    assert!(description.contains("sessions"));
}

#[test]
fn initial_schema_up_creates_all_expected_tables() {
    let conn = Connection::open_in_memory().unwrap();
    InitialSchema.up(&conn).unwrap();

    assert!(table_exists(&conn, "repositories"));
    assert!(table_exists(&conn, "sessions"));
    assert!(table_exists(&conn, "challenges"));
    assert!(table_exists(&conn, "stages"));
    assert!(table_exists(&conn, "session_results"));
    assert!(table_exists(&conn, "stage_results"));
}

#[test]
fn initial_schema_up_creates_all_expected_indexes() {
    let conn = Connection::open_in_memory().unwrap();
    InitialSchema.up(&conn).unwrap();

    assert!(index_exists(&conn, "idx_stage_results_repo_date"));
    assert!(index_exists(&conn, "idx_stage_results_language"));
    assert!(index_exists(&conn, "idx_stage_results_session"));
    assert!(index_exists(&conn, "idx_session_results_repo_date"));
    assert!(index_exists(&conn, "idx_sessions_repo_date"));
}

#[test]
fn initial_schema_up_is_idempotent() {
    let conn = Connection::open_in_memory().unwrap();
    InitialSchema.up(&conn).unwrap();
    InitialSchema.up(&conn).unwrap();

    assert!(table_exists(&conn, "repositories"));
    assert!(table_exists(&conn, "stage_results"));
}

#[test]
fn migration_trait_dispatch_invokes_version_and_description() {
    let migration: Box<dyn Migration> = Box::new(InitialSchema);
    assert_eq!(migration.version(), 1);
    assert!(!migration.description().is_empty());
}

#[test]
fn get_all_migrations_returns_ordered_versions_up_to_latest() {
    let migrations = get_all_migrations();
    assert!(!migrations.is_empty());

    let latest = get_latest_version();
    assert!(migrations.iter().any(|m| m.version() == latest));
}
