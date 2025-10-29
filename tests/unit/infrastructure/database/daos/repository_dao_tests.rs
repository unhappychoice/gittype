use gittype::domain::models::GitRepository;
use gittype::infrastructure::database::daos::RepositoryDao;
use gittype::infrastructure::database::database::Database;

#[test]
fn test_new_creates_dao() {
    let db = Database::new().expect("Failed to create database");
    let _dao = RepositoryDao::new(&db);
}

#[test]
fn test_ensure_repository_creates_new_repository() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

    let found = dao.get_repository_by_id(99999).unwrap();
    assert!(found.is_none(), "Should return None for non-existent ID");
}

#[test]
fn test_find_repository() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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

#[test]
fn test_repository_uniqueness_constraint() {
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
    let db = Database::new().unwrap();
    db.init().unwrap();
    let dao = RepositoryDao::new(&db);

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
