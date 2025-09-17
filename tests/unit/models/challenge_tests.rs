use gittype::game::DifficultyLevel;
use gittype::models::challenge::Challenge;
use gittype::models::git_repository::GitRepository;

fn build_repo() -> GitRepository {
    GitRepository {
        user_name: "user".into(),
        repository_name: "repo".into(),
        remote_url: "https://example.com/user/repo.git".into(),
        branch: Some("main".into()),
        commit_hash: Some("abc123".into()),
        is_dirty: false,
        root_path: None,
    }
}

#[test]
fn display_title_defaults_to_id() {
    let challenge = Challenge::new("42".into(), "fn main() {}".into());
    assert_eq!(challenge.get_display_title(), "Challenge 42");
}

#[test]
fn display_title_uses_relative_path_and_lines() {
    let challenge = Challenge::new("x".into(), "fn main() {}".into())
        .with_source_info("/tmp/src/main.rs".into(), 5, 10);

    assert_eq!(challenge.get_display_title(), "src/main.rs:5-10");
}

#[test]
fn display_title_with_repo_prefixes_repository() {
    let challenge = Challenge::new("x".into(), "fn main() {}".into())
        .with_source_info("/tmp/src/main.rs".into(), 1, 2)
        .with_difficulty_level(DifficultyLevel::Easy);
    let repo = build_repo();

    assert_eq!(
        challenge.get_display_title_with_repo(&Some(repo)),
        "[user/repo] src/main.rs:1-2"
    );
}

#[test]
fn builders_set_optional_fields() {
    let challenge = Challenge::new("x".into(), "fn main() {}".into())
        .with_source_info("main.rs".into(), 1, 2)
        .with_language("rust".into())
        .with_comment_ranges(vec![(0, 5)])
        .with_difficulty_level(DifficultyLevel::Hard);

    assert_eq!(challenge.source_file_path.as_deref(), Some("main.rs"));
    assert_eq!(challenge.start_line, Some(1));
    assert_eq!(challenge.end_line, Some(2));
    assert_eq!(challenge.language.as_deref(), Some("rust"));
    assert_eq!(challenge.comment_ranges, vec![(0, 5)]);
    assert_eq!(challenge.difficulty_level, Some(DifficultyLevel::Hard));
}

#[test]
fn display_title_with_repo_without_source_falls_back_to_id() {
    let challenge = Challenge::new("7".into(), "fn main() {}".into());
    assert_eq!(
        challenge.get_display_title_with_repo(&Some(build_repo())),
        "Challenge 7"
    );
}
