use gittype::domain::models::git_repository::GitRepository;
use gittype::domain::repositories::git_repository_repository::GitRepositoryRepositoryInterface;
use gittype::presentation::di::AppModule;
use shaku::HasComponent;
use std::sync::Arc;

fn create_test_repository() -> GitRepository {
    GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc123".to_string()),
        is_dirty: false,
        root_path: None,
    }
}

fn create_test_repo_repository() -> Arc<dyn GitRepositoryRepositoryInterface> {
    let module = AppModule::builder().build();
    module.resolve()
}

#[test]
fn creates_repository_via_di() {
    let _repo = create_test_repo_repository();
    // Test passes if construction succeeds
}

#[test]
fn ensure_repository_returns_id() {
    let repo_repo = create_test_repo_repository();
    let git_repo = create_test_repository();

    let result = repo_repo.ensure_repository(&git_repo);
    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[test]
fn get_all_repositories_returns_result() {
    let repo_repo = create_test_repo_repository();
    let result = repo_repo.get_all_repositories();
    assert!(result.is_ok());
}

#[test]
fn get_repository_by_id_returns_result() {
    let repo_repo = create_test_repo_repository();
    let result = repo_repo.get_repository_by_id(1);
    assert!(result.is_ok());
}

#[test]
fn find_repository_returns_result() {
    let repo_repo = create_test_repo_repository();
    let result = repo_repo.find_repository("testuser", "testrepo");
    assert!(result.is_ok());
}

#[test]
fn get_user_repositories_returns_result() {
    let repo_repo = create_test_repo_repository();
    let result = repo_repo.get_user_repositories("testuser");
    assert!(result.is_ok());
}

#[test]
fn ensure_repository_twice_returns_same_id() {
    let repo_repo = create_test_repo_repository();
    let git_repo = create_test_repository();

    let id1 = repo_repo
        .ensure_repository(&git_repo)
        .expect("First insert");
    let id2 = repo_repo
        .ensure_repository(&git_repo)
        .expect("Second insert");

    assert_eq!(id1, id2);
}

#[test]
fn get_repository_by_id_finds_inserted_repository() {
    let repo_repo = create_test_repo_repository();
    let git_repo = create_test_repository();

    let id = repo_repo
        .ensure_repository(&git_repo)
        .expect("Insert repository");
    let found = repo_repo
        .get_repository_by_id(id)
        .expect("Query repository");

    assert!(found.is_some());
    let stored = found.unwrap();
    assert_eq!(stored.user_name, git_repo.user_name);
    assert_eq!(stored.repository_name, git_repo.repository_name);
}

#[test]
fn find_repository_finds_inserted_repository() {
    let repo_repo = create_test_repo_repository();
    let git_repo = create_test_repository();

    repo_repo
        .ensure_repository(&git_repo)
        .expect("Insert repository");
    let found = repo_repo
        .find_repository(&git_repo.user_name, &git_repo.repository_name)
        .expect("Query repository");

    assert!(found.is_some());
    let stored = found.unwrap();
    assert_eq!(stored.user_name, git_repo.user_name);
    assert_eq!(stored.repository_name, git_repo.repository_name);
}

#[test]
fn get_user_repositories_filters_by_user() {
    let repo_repo = create_test_repo_repository();

    let mut repo1 = create_test_repository();
    repo1.user_name = "user1".to_string();
    repo1.repository_name = "repo1".to_string();

    let mut repo2 = create_test_repository();
    repo2.user_name = "user2".to_string();
    repo2.repository_name = "repo2".to_string();

    repo_repo.ensure_repository(&repo1).expect("Insert repo1");
    repo_repo.ensure_repository(&repo2).expect("Insert repo2");

    let user1_repos = repo_repo
        .get_user_repositories("user1")
        .expect("Query user1 repos");
    let user2_repos = repo_repo
        .get_user_repositories("user2")
        .expect("Query user2 repos");

    assert!(user1_repos.iter().all(|r| r.user_name == "user1"));
    assert!(user2_repos.iter().all(|r| r.user_name == "user2"));
}
