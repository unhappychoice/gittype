use gittype::models::git_repository::GitRepository;

#[test]
fn git_repository_equality_depends_on_all_fields() {
    let repo = GitRepository {
        user_name: "user".into(),
        repository_name: "repo".into(),
        remote_url: "https://example.com/repo.git".into(),
        branch: Some("main".into()),
        commit_hash: Some("abc".into()),
        is_dirty: false,
        root_path: None,
    };

    let mut same = repo.clone();
    assert_eq!(repo, same);

    same.is_dirty = true;
    assert_ne!(repo, same);
}
