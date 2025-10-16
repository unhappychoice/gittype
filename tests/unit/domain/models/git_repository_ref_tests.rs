use gittype::domain::models::git_repository_ref::GitRepositoryRef;

#[test]
fn http_url_returns_correct_format() {
    let git_ref = GitRepositoryRef {
        origin: "github.com".to_string(),
        owner: "testuser".to_string(),
        name: "testrepo".to_string(),
    };

    let url = git_ref.http_url();
    assert_eq!(url, "https://github.com/testuser/testrepo.git");
}

#[test]
fn http_url_with_different_origin() {
    let git_ref = GitRepositoryRef {
        origin: "gitlab.com".to_string(),
        owner: "myuser".to_string(),
        name: "myproject".to_string(),
    };

    let url = git_ref.http_url();
    assert_eq!(url, "https://gitlab.com/myuser/myproject.git");
}

#[test]
fn clone_creates_copy() {
    let git_ref = GitRepositoryRef {
        origin: "github.com".to_string(),
        owner: "user".to_string(),
        name: "repo".to_string(),
    };

    let cloned = git_ref.clone();
    assert_eq!(cloned.origin, git_ref.origin);
    assert_eq!(cloned.owner, git_ref.owner);
    assert_eq!(cloned.name, git_ref.name);
}

#[test]
fn http_url_with_special_characters() {
    let git_ref = GitRepositoryRef {
        origin: "github.com".to_string(),
        owner: "test-user".to_string(),
        name: "test_repo".to_string(),
    };

    let url = git_ref.http_url();
    assert_eq!(url, "https://github.com/test-user/test_repo.git");
}
