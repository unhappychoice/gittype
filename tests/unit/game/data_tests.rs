use std::path::PathBuf;
use std::sync::Once;

use gittype::domain::models::challenge::Challenge;
use gittype::domain::models::git_repository::GitRepository;
use gittype::domain::models::ExtractionOptions;
use gittype::game::GameData;

fn setup_game_data() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        GameData::initialize().expect("GameData should initialize once");
    });
    GameData::reset().expect("GameData reset should succeed");
}

fn sample_repo(root: Option<PathBuf>) -> GitRepository {
    GitRepository {
        user_name: "user".into(),
        repository_name: "repo".into(),
        remote_url: "https://example.com/user/repo.git".into(),
        branch: Some("main".into()),
        commit_hash: Some("abc123".into()),
        is_dirty: false,
        root_path: root,
    }
}

#[test]
fn processing_parameters_roundtrip() {
    setup_game_data();

    let options = ExtractionOptions {
        include_patterns: vec!["**/*.rs".into()],
        exclude_patterns: vec!["target".into()],
        languages: Some(vec!["rust".into()]),
        max_file_size_bytes: 1024 * 1024, // 1MB
    };

    let repo_path = PathBuf::from("/tmp/repo");
    GameData::set_processing_parameters(Some("owner/repo"), Some(&repo_path), &options)
        .expect("should store parameters");

    let (spec, path, stored) =
        GameData::get_processing_parameters().expect("processing parameters should be available");

    assert_eq!(spec, Some("owner/repo".into()));
    assert_eq!(path, Some(repo_path));
    assert_eq!(stored.include_patterns, options.include_patterns);
    assert_eq!(stored.exclude_patterns, options.exclude_patterns);
    assert_eq!(stored.languages, options.languages);
}

#[test]
fn set_results_and_take_challenges() {
    setup_game_data();

    let challenges = vec![
        Challenge::new("1".into(), "fn main() {}".into()),
        Challenge::new("2".into(), "fn helper() {}".into()),
    ];

    GameData::set_results(challenges.clone(), Some(sample_repo(None))).expect("set_results");

    assert!(GameData::is_loading_completed());

    let ids =
        GameData::with_challenges(|list| list.iter().map(|c| c.id.clone()).collect::<Vec<_>>())
            .expect("with_challenges should see stored data");
    assert_eq!(ids, vec!["1", "2"]);

    let taken = GameData::take_challenges().expect("challenges should be taken once");
    assert_eq!(taken.len(), 2);
    assert!(GameData::take_challenges().is_none());
}

#[test]
fn loading_failure_sets_flags() {
    setup_game_data();

    GameData::set_loading_failed("boom".into()).expect("set_loading_failed");

    assert!(GameData::is_loading_failed());
    assert!(!GameData::is_loading_completed());
}

#[test]
fn git_repository_roundtrip() {
    setup_game_data();

    let temp_dir = tempfile::TempDir::new().expect("temp dir");
    let repo = sample_repo(Some(temp_dir.path().to_path_buf()));

    GameData::set_git_repository(Some(repo.clone())).expect("set_git_repository");
    assert_eq!(GameData::get_git_repository(), Some(repo.clone()));

    GameData::set_git_repository(None).expect("clear git repository");
    assert_eq!(GameData::get_git_repository(), None);
}
