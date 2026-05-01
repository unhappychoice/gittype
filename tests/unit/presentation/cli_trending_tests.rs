use gittype::presentation::cli::commands::run_trending;
use gittype::GitTypeError;

#[test]
fn run_trending_rejects_unsupported_language() {
    let result = run_trending(
        Some("invalid-language".to_string()),
        None,
        "daily".to_string(),
    );

    assert!(matches!(
        result,
        Err(GitTypeError::ValidationError(message))
        if message == "Unsupported language: invalid-language"
    ));
}

#[test]
fn run_trending_accepts_supported_language_before_repo_name_validation() {
    let result = run_trending(
        Some("rUsT".to_string()),
        Some("repoonly".to_string()),
        "weekly".to_string(),
    );

    assert!(result.is_ok());
}
