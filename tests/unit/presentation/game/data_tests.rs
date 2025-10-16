use std::path::PathBuf;
use std::sync::Once;

use crate::fixtures::models::{challenge, git_repository};
use gittype::domain::models::git_repository::GitRepository;
use gittype::domain::models::ExtractionOptions;
use gittype::presentation::game::GameData;

fn setup_game_data() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let _ = GameData::initialize();
    });
    // Reset may fail if already reset, ignore the error
    let _ = GameData::reset();
}

fn sample_repo(root: Option<PathBuf>) -> GitRepository {
    if let Some(root) = root {
        git_repository::build_with_root_path(root)
    } else {
        git_repository::build()
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
        challenge::build_with_id_and_code("1", "fn main() {}"),
        challenge::build_with_id_and_code("2", "fn helper() {}"),
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
