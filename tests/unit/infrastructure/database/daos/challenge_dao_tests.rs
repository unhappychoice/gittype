use gittype::domain::models::{Challenge, DifficultyLevel};
use gittype::infrastructure::database::daos::ChallengeDao;
use gittype::infrastructure::database::database::Database;

#[test]
fn test_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    let _dao = ChallengeDao::new(&db);
}

#[test]
fn test_ensure_challenge_creates_new_challenge() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = ChallengeDao::new(&db);

    let challenge = Challenge::new("test-challenge-1".to_string(), "fn test() {}".to_string())
        .with_language("rust".to_string())
        .with_source_info("src/test.rs".to_string(), 1, 10)
        .with_difficulty_level(DifficultyLevel::Easy);

    let conn = db.get_connection().unwrap();
    let tx = conn.unchecked_transaction().unwrap();

    let rowid = dao
        .ensure_challenge_in_transaction(&tx, &challenge)
        .unwrap();
    tx.commit().unwrap();
    drop(conn);

    assert!(rowid > 0, "Should return positive rowid");
}

#[test]
fn test_ensure_challenge_returns_existing_challenge() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = ChallengeDao::new(&db);

    let challenge = Challenge::new("test-challenge-2".to_string(), "fn test() {}".to_string())
        .with_language("rust".to_string());

    // Insert first time
    let conn = db.get_connection().unwrap();
    let tx1 = conn.unchecked_transaction().unwrap();
    let rowid1 = dao
        .ensure_challenge_in_transaction(&tx1, &challenge)
        .unwrap();
    tx1.commit().unwrap();

    // Insert second time - should return same rowid
    let tx2 = conn.unchecked_transaction().unwrap();
    let rowid2 = dao
        .ensure_challenge_in_transaction(&tx2, &challenge)
        .unwrap();
    tx2.commit().unwrap();

    assert_eq!(
        rowid1, rowid2,
        "Should return same rowid for existing challenge"
    );
}

#[test]
fn test_ensure_challenge_with_different_difficulties() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = ChallengeDao::new(&db);

    let difficulties = [
        DifficultyLevel::Easy,
        DifficultyLevel::Normal,
        DifficultyLevel::Hard,
    ];

    let conn = db.get_connection().unwrap();
    for (i, difficulty) in difficulties.iter().enumerate() {
        let challenge = Challenge::new(
            format!("test-challenge-diff-{}", i),
            format!("fn test_{}() {{}}", i),
        )
        .with_difficulty_level(*difficulty);

        let tx = conn.unchecked_transaction().unwrap();
        let rowid = dao
            .ensure_challenge_in_transaction(&tx, &challenge)
            .unwrap();
        tx.commit().unwrap();

        assert!(
            rowid > 0,
            "Should create challenge with difficulty {:?}",
            difficulty
        );
    }
    drop(conn);
}

#[test]
fn test_ensure_challenge_with_comment_ranges() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = ChallengeDao::new(&db);

    let challenge = Challenge::new(
        "test-challenge-comments".to_string(),
        "// comment\nfn test() {}".to_string(),
    )
    .with_comment_ranges(vec![(0, 10)]);

    let conn = db.get_connection().unwrap();
    let tx = conn.unchecked_transaction().unwrap();
    let rowid = dao
        .ensure_challenge_in_transaction(&tx, &challenge)
        .unwrap();
    tx.commit().unwrap();
    drop(conn);

    assert!(rowid > 0, "Should create challenge with comment ranges");
}

#[test]
fn test_ensure_multiple_challenges_in_transaction() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = ChallengeDao::new(&db);

    let challenges: Vec<Challenge> = (0..5)
        .map(|i| {
            Challenge::new(
                format!("multi-challenge-{}", i),
                format!("fn test_{}() {{}}", i),
            )
            .with_language("rust".to_string())
        })
        .collect();

    let conn = db.get_connection().unwrap();
    let tx = conn.unchecked_transaction().unwrap();

    let mut rowids = Vec::new();
    for challenge in &challenges {
        let rowid = dao.ensure_challenge_in_transaction(&tx, challenge).unwrap();
        rowids.push(rowid);
    }

    tx.commit().unwrap();
    drop(conn);

    // All rowids should be unique and positive
    assert_eq!(rowids.len(), 5, "Should insert 5 challenges");
    for rowid in &rowids {
        assert!(*rowid > 0, "Rowid should be positive");
    }

    // Check uniqueness
    let unique_rowids: std::collections::HashSet<_> = rowids.iter().collect();
    assert_eq!(unique_rowids.len(), 5, "All rowids should be unique");
}
