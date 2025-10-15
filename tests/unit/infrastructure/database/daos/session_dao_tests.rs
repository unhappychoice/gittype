use gittype::infrastructure::database::daos::SessionDao;
use gittype::infrastructure::database::database::Database;

#[test]
fn session_dao_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    db.init().expect("Failed to initialize database");
    let _dao = SessionDao::new(&db);
    // Test passes if construction succeeds
}
