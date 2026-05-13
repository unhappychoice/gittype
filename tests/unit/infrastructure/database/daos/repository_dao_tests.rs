use gittype::domain::models::storage::SaveSessionResultParams;
use gittype::domain::models::{Challenge, GitRepository, SessionResult};
use gittype::infrastructure::database::daos::{
    ChallengeDao, ChallengeDaoInterface, RepositoryDao, RepositoryDaoInterface, SessionDao,
    SessionDaoInterface,
};
use gittype::infrastructure::database::database::{Database, DatabaseInterface};
use std::sync::Arc;

#[test]
fn test_new_creates_dao() {
    let db =
        Arc::new(Database::new().expect("Failed to create database")) as Arc<dyn DatabaseInterface>;
    let _dao = RepositoryDao::new(Arc::clone(&db));
}

#[test]
fn test_ensure_repository_creates_new_repository() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let git_repo = GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    };

    let id = dao.ensure_repository(&git_repo).unwrap();
    assert!(id > 0, "Should return positive repository ID");
}

#[test]
fn test_ensure_repository_returns_existing_repository() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let git_repo = GitRepository {
        user_name: "existinguser".to_string(),
        repository_name: "existingrepo".to_string(),
        remote_url: "https://github.com/existinguser/existingrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("xyz789".to_string()),
        is_dirty: false,
        root_path: None,
    };

    // Insert first time
    let id1 = dao.ensure_repository(&git_repo).unwrap();

    // Insert second time - should return same ID
    let id2 = dao.ensure_repository(&git_repo).unwrap();

    assert_eq!(
        id1, id2,
        "Should return same ID for existing repository based on user_name and repository_name"
    );
}

#[test]
fn test_ensure_repository_in_transaction() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let git_repo = GitRepository {
        user_name: "txuser".to_string(),
        repository_name: "txrepo".to_string(),
        remote_url: "https://github.com/txuser/txrepo".to_string(),
        branch: Some("develop".to_string()),
        commit_hash: Some("def456".to_string()),
        is_dirty: true,
        root_path: None,
    };

    let conn = db.get_connection().unwrap();
    let tx = conn.unchecked_transaction().unwrap();
    let id = dao
        .ensure_repository_in_transaction(&tx, &git_repo)
        .unwrap();
    tx.commit().unwrap();
    drop(conn);

    assert!(id > 0, "Should return positive repository ID");

    // Verify the repository was actually created
    let found = dao
        .find_repository(&git_repo.user_name, &git_repo.repository_name)
        .unwrap();
    assert!(found.is_some(), "Repository should exist after transaction");
    assert_eq!(found.unwrap().id, id);
}

#[test]
fn test_get_all_repositories() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    // Create multiple repositories
    let repos = vec![
        GitRepository {
            user_name: "user1".to_string(),
            repository_name: "repo1".to_string(),
            remote_url: "https://github.com/user1/repo1".to_string(),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            root_path: None,
        },
        GitRepository {
            user_name: "user2".to_string(),
            repository_name: "repo2".to_string(),
            remote_url: "https://github.com/user2/repo2".to_string(),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            root_path: None,
        },
        GitRepository {
            user_name: "user1".to_string(),
            repository_name: "repo3".to_string(),
            remote_url: "https://github.com/user1/repo3".to_string(),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            root_path: None,
        },
    ];

    for repo in &repos {
        dao.ensure_repository(repo).unwrap();
    }

    let all_repos = dao.get_all_repositories().unwrap();
    assert_eq!(
        all_repos.len(),
        3,
        "Should return all 3 created repositories"
    );

    // Verify sorted by user_name, repository_name
    assert_eq!(all_repos[0].user_name, "user1");
    assert_eq!(all_repos[0].repository_name, "repo1");
    assert_eq!(all_repos[1].user_name, "user1");
    assert_eq!(all_repos[1].repository_name, "repo3");
    assert_eq!(all_repos[2].user_name, "user2");
    assert_eq!(all_repos[2].repository_name, "repo2");
}

#[test]
fn test_get_repository_by_id() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let git_repo = GitRepository {
        user_name: "fetchuser".to_string(),
        repository_name: "fetchrepo".to_string(),
        remote_url: "https://github.com/fetchuser/fetchrepo".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };

    let id = dao.ensure_repository(&git_repo).unwrap();

    // Get by ID
    let found = dao.get_repository_by_id(id).unwrap();
    assert!(found.is_some(), "Should find repository by ID");

    let repo = found.unwrap();
    assert_eq!(repo.id, id);
    assert_eq!(repo.user_name, "fetchuser");
    assert_eq!(repo.repository_name, "fetchrepo");
    assert_eq!(
        repo.remote_url,
        "https://github.com/fetchuser/fetchrepo".to_string()
    );
}

#[test]
fn test_get_repository_by_id_not_found() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let found = dao.get_repository_by_id(99999).unwrap();
    assert!(found.is_none(), "Should return None for non-existent ID");
}

#[test]
fn test_find_repository() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let git_repo = GitRepository {
        user_name: "searchuser".to_string(),
        repository_name: "searchrepo".to_string(),
        remote_url: "https://github.com/searchuser/searchrepo".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };

    let id = dao.ensure_repository(&git_repo).unwrap();

    // Find by user_name and repository_name
    let found = dao.find_repository("searchuser", "searchrepo").unwrap();
    assert!(found.is_some(), "Should find repository by name");

    let repo = found.unwrap();
    assert_eq!(repo.id, id);
    assert_eq!(repo.user_name, "searchuser");
    assert_eq!(repo.repository_name, "searchrepo");
}

#[test]
fn test_find_repository_not_found() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let found = dao
        .find_repository("nonexistentuser", "nonexistentrepo")
        .unwrap();
    assert!(
        found.is_none(),
        "Should return None for non-existent repository"
    );
}

#[test]
fn test_get_all_repositories_with_languages() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    // Create a repository
    let git_repo = GitRepository {
        user_name: "languser".to_string(),
        repository_name: "langrepo".to_string(),
        remote_url: "https://github.com/languser/langrepo".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };

    dao.ensure_repository(&git_repo).unwrap();

    // Get repositories with languages
    let repos = dao.get_all_repositories_with_languages().unwrap();
    assert!(!repos.is_empty(), "Should return at least one repository");

    let repo = repos
        .iter()
        .find(|r| r.user_name == "languser" && r.repository_name == "langrepo");
    assert!(repo.is_some(), "Should find the created repository");

    // Without sessions/stage_results, languages should be empty
    let repo = repo.unwrap();
    assert_eq!(
        repo.languages.len(),
        0,
        "Languages should be empty without sessions"
    );
}

fn insert_stage_result_with_language(
    db: &Arc<dyn DatabaseInterface>,
    repository_id: i64,
    language: &str,
    challenge_id: &str,
) {
    let session_dao = SessionDao::new(Arc::clone(db));
    let challenge_dao = ChallengeDao::new(Arc::clone(db));
    let git_repo = GitRepository {
        user_name: "languser".to_string(),
        repository_name: "langrepo".to_string(),
        remote_url: "https://github.com/languser/langrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc".to_string()),
        is_dirty: false,
        root_path: None,
    };
    let session_result = SessionResult::new();

    let challenge = Challenge::new(challenge_id.to_string(), "fn dummy() {}".to_string())
        .with_language(language.to_string());

    let conn = db.get_connection().unwrap();
    let tx = conn.unchecked_transaction().unwrap();
    challenge_dao
        .ensure_challenge_in_transaction(&tx, &challenge)
        .unwrap();
    let session_id = session_dao
        .create_session_in_transaction(
            &tx,
            Some(repository_id),
            &session_result,
            Some(&git_repo),
            "normal",
            Some("easy"),
        )
        .unwrap();
    session_dao
        .save_session_result_in_transaction(
            &tx,
            SaveSessionResultParams {
                session_id,
                repository_id: Some(repository_id),
                session_result: &session_result,
                stage_engines: &[],
                game_mode: "normal",
                difficulty_level: Some("easy"),
            },
        )
        .unwrap();

    let completed_at = chrono::Utc::now().to_rfc3339();
    tx.execute(
        "INSERT INTO stages (session_id, challenge_id, stage_number, started_at, completed_at)
         VALUES (?, ?, ?, ?, ?)",
        rusqlite::params![session_id, challenge_id, 1i64, &completed_at, &completed_at],
    )
    .unwrap();
    let stage_id = tx.last_insert_rowid();

    tx.execute(
        "INSERT INTO stage_results (
            stage_id, session_id, repository_id, keystrokes, mistakes, duration_ms,
            wpm, cpm, accuracy, consistency_streaks, score, rank_name, tier_name,
            rank_position, rank_total, position, total,
            was_skipped, was_failed, completed_at, language, difficulty_level
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            stage_id,
            session_id,
            repository_id,
            10i64,
            0i64,
            1000i64,
            50.0,
            250.0,
            100.0,
            "[]",
            50.0,
            "Beginner",
            "Bronze",
            1i64,
            100i64,
            1i64,
            500i64,
            false,
            false,
            &completed_at,
            language,
            "Easy",
        ],
    )
    .unwrap();

    tx.commit().unwrap();
}

#[test]
fn test_get_all_repositories_with_languages_aggregates_multiple_languages() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let git_repo = GitRepository {
        user_name: "languser".to_string(),
        repository_name: "langrepo".to_string(),
        remote_url: "https://github.com/languser/langrepo".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };
    let repository_id = dao.ensure_repository(&git_repo).unwrap();

    insert_stage_result_with_language(&db, repository_id, "rust", "challenge-rust-1");
    insert_stage_result_with_language(&db, repository_id, "javascript", "challenge-js-1");
    insert_stage_result_with_language(&db, repository_id, "rust", "challenge-rust-2");

    let repos = dao.get_all_repositories_with_languages().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.user_name == "languser" && r.repository_name == "langrepo")
        .expect("Repository should exist");

    assert_eq!(
        repo.languages.len(),
        2,
        "DISTINCT should de-duplicate the doubled rust language entry"
    );
    assert!(repo.languages.iter().any(|l| l == "rust"));
    assert!(repo.languages.iter().any(|l| l == "javascript"));
}

#[test]
fn test_get_all_repositories_with_languages_handles_empty_database() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let repos = dao.get_all_repositories_with_languages().unwrap();
    assert!(repos.is_empty());
}

#[test]
fn test_find_repository_distinguishes_by_user_and_name() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let repo_a = GitRepository {
        user_name: "alice".to_string(),
        repository_name: "project".to_string(),
        remote_url: "https://github.com/alice/project".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };
    let repo_b = GitRepository {
        user_name: "bob".to_string(),
        repository_name: "project".to_string(),
        remote_url: "https://github.com/bob/project".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };

    let id_a = dao.ensure_repository(&repo_a).unwrap();
    let id_b = dao.ensure_repository(&repo_b).unwrap();
    assert_ne!(id_a, id_b);

    let found_a = dao.find_repository("alice", "project").unwrap().unwrap();
    let found_b = dao.find_repository("bob", "project").unwrap().unwrap();
    assert_eq!(found_a.id, id_a);
    assert_eq!(found_b.id, id_b);
    assert_eq!(found_a.remote_url, "https://github.com/alice/project");
}

#[test]
fn test_get_repository_by_id_returns_correct_record() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let repos = [
        ("first", "repo_one"),
        ("second", "repo_two"),
        ("third", "repo_three"),
    ];
    let mut ids = Vec::new();
    for (user, name) in repos.iter() {
        let git_repo = GitRepository {
            user_name: user.to_string(),
            repository_name: name.to_string(),
            remote_url: format!("https://github.com/{}/{}", user, name),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            root_path: None,
        };
        ids.push(dao.ensure_repository(&git_repo).unwrap());
    }

    for ((user, name), id) in repos.iter().zip(ids.iter()) {
        let found = dao.get_repository_by_id(*id).unwrap().unwrap();
        assert_eq!(found.user_name, *user);
        assert_eq!(found.repository_name, *name);
    }
}

#[test]
fn test_repository_uniqueness_constraint() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    // Create two repositories with same user_name and repository_name but different remote_url
    let git_repo1 = GitRepository {
        user_name: "sameuser".to_string(),
        repository_name: "samerepo".to_string(),
        remote_url: "https://github.com/sameuser/samerepo".to_string(),
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };

    let git_repo2 = GitRepository {
        user_name: "sameuser".to_string(),
        repository_name: "samerepo".to_string(),
        remote_url: "https://gitlab.com/sameuser/samerepo".to_string(), // Different URL
        branch: None,
        commit_hash: None,
        is_dirty: false,
        root_path: None,
    };

    let id1 = dao.ensure_repository(&git_repo1).unwrap();
    let id2 = dao.ensure_repository(&git_repo2).unwrap();

    // Should return the same ID because uniqueness is based on user_name and repository_name
    assert_eq!(
        id1, id2,
        "Should return same ID for same user_name and repository_name"
    );
}

#[test]
fn test_multiple_repositories_in_transaction() {
    let db_impl = Database::new().unwrap();
    db_impl.init().unwrap();
    let db = Arc::new(db_impl) as Arc<dyn DatabaseInterface>;
    let dao = RepositoryDao::new(Arc::clone(&db));

    let repos = vec![
        GitRepository {
            user_name: "txuser1".to_string(),
            repository_name: "txrepo1".to_string(),
            remote_url: "https://github.com/txuser1/txrepo1".to_string(),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            root_path: None,
        },
        GitRepository {
            user_name: "txuser2".to_string(),
            repository_name: "txrepo2".to_string(),
            remote_url: "https://github.com/txuser2/txrepo2".to_string(),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            root_path: None,
        },
        GitRepository {
            user_name: "txuser3".to_string(),
            repository_name: "txrepo3".to_string(),
            remote_url: "https://github.com/txuser3/txrepo3".to_string(),
            branch: None,
            commit_hash: None,
            is_dirty: false,
            root_path: None,
        },
    ];

    let conn = db.get_connection().unwrap();
    let tx = conn.unchecked_transaction().unwrap();

    let mut ids = Vec::new();
    for repo in &repos {
        let id = dao.ensure_repository_in_transaction(&tx, repo).unwrap();
        ids.push(id);
    }

    tx.commit().unwrap();
    drop(conn);

    // All IDs should be unique and positive
    assert_eq!(ids.len(), 3, "Should create 3 repositories");
    for id in &ids {
        assert!(*id > 0, "Repository ID should be positive");
    }

    // Check uniqueness
    let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(unique_ids.len(), 3, "All repository IDs should be unique");

    // Verify all repositories exist
    let all_repos = dao.get_all_repositories().unwrap();
    assert!(
        all_repos.len() >= 3,
        "Should have at least 3 repositories after transaction"
    );
}
