use gittype::domain::models::storage::repository::{
    StoredRepository, StoredRepositoryWithLanguages,
};

#[test]
fn stored_repository_clone() {
    let repo = StoredRepository {
        id: 1,
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo.git".to_string(),
    };

    let cloned = repo.clone();
    assert_eq!(cloned.id, repo.id);
    assert_eq!(cloned.user_name, repo.user_name);
    assert_eq!(cloned.repository_name, repo.repository_name);
    assert_eq!(cloned.remote_url, repo.remote_url);
}

#[test]
fn stored_repository_with_languages_clone() {
    let repo = StoredRepositoryWithLanguages {
        id: 1,
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo.git".to_string(),
        languages: vec!["Rust".to_string(), "Python".to_string()],
    };

    let cloned = repo.clone();
    assert_eq!(cloned.id, repo.id);
    assert_eq!(cloned.languages.len(), 2);
}

#[test]
fn http_url_with_valid_github_url() {
    let repo = StoredRepositoryWithLanguages {
        id: 1,
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "github.com/testuser/testrepo".to_string(),
        languages: vec![],
    };

    let url = repo.http_url();
    // Parser will parse github.com/testuser/testrepo as origin=github.com, owner=testuser, name=testrepo
    assert!(url.starts_with("https://"));
    assert!(url.contains("github.com"));
    assert!(url.contains("testuser"));
    assert!(url.contains("testrepo"));
}

#[test]
fn http_url_with_already_formatted_url() {
    let repo = StoredRepositoryWithLanguages {
        id: 1,
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo.git".to_string(),
        languages: vec![],
    };

    let url = repo.http_url();
    // Should parse and reformat
    assert!(url.starts_with("https://"));
    assert!(url.contains("github.com"));
}

#[test]
fn http_url_with_invalid_format_returns_original() {
    let repo = StoredRepositoryWithLanguages {
        id: 1,
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "invalid-url".to_string(),
        languages: vec![],
    };

    let url = repo.http_url();
    assert_eq!(url, "invalid-url");
}

#[test]
fn stored_repository_with_languages_has_empty_languages() {
    let repo = StoredRepositoryWithLanguages {
        id: 1,
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo.git".to_string(),
        languages: vec![],
    };

    assert!(repo.languages.is_empty());
}

#[test]
fn stored_repository_with_languages_has_multiple_languages() {
    let repo = StoredRepositoryWithLanguages {
        id: 1,
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo.git".to_string(),
        languages: vec![
            "Rust".to_string(),
            "Python".to_string(),
            "JavaScript".to_string(),
        ],
    };

    assert_eq!(repo.languages.len(), 3);
    assert!(repo.languages.contains(&"Rust".to_string()));
    assert!(repo.languages.contains(&"Python".to_string()));
    assert!(repo.languages.contains(&"JavaScript".to_string()));
}
