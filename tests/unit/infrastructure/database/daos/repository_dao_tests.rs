use gittype::infrastructure::database::daos::RepositoryDao;
use gittype::infrastructure::database::database::Database;

#[test]
fn repository_dao_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    db.init().expect("Failed to initialize database");
    let _dao = RepositoryDao::new(&db);
    // Test passes if construction succeeds
}
