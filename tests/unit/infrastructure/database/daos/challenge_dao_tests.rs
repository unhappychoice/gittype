use gittype::infrastructure::database::daos::ChallengeDao;
use gittype::infrastructure::database::database::Database;

#[test]
fn challenge_dao_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    let _dao = ChallengeDao::new(&db);
    // Test passes if construction succeeds
}
