use gittype::infrastructure::database::daos::StageDao;
use gittype::infrastructure::database::database::Database;

#[test]
fn stage_dao_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    let _dao = StageDao::new(&db);
    // Test passes if construction succeeds
}
