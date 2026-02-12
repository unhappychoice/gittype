use std::collections::HashSet;

use gittype::domain::models::{GitRepository, SessionResult};
use gittype::presentation::sharing::{SharingPlatform, SharingService};

#[test]
fn sharing_platform_all_lists_every_variant_once() {
    let platforms = SharingPlatform::all();
    let names: HashSet<_> = platforms.iter().map(|p| p.name()).collect();

    assert_eq!(platforms.len(), 4);
    assert_eq!(names.len(), 4);
    assert!(names.contains("X"));
    assert!(names.contains("Reddit"));
    assert!(names.contains("LinkedIn"));
    assert!(names.contains("Facebook"));
}

#[test]
fn sharing_platform_name_matches_variant() {
    assert_eq!(SharingPlatform::X.name(), "X");
    assert_eq!(SharingPlatform::Reddit.name(), "Reddit");
    assert_eq!(SharingPlatform::LinkedIn.name(), "LinkedIn");
    assert_eq!(SharingPlatform::Facebook.name(), "Facebook");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
fn make_metrics(
    score: f64,
    cpm: f64,
    valid_mistakes: usize,
    invalid_mistakes: usize,
) -> SessionResult {
    let mut result = SessionResult::new();
    result.session_score = score;
    result.overall_cpm = cpm;
    result.valid_mistakes = valid_mistakes;
    result.invalid_mistakes = invalid_mistakes;
    result
}

fn make_repo() -> GitRepository {
    GitRepository {
        user_name: "testuser".to_string(),
        repository_name: "testrepo".to_string(),
        remote_url: "https://github.com/testuser/testrepo".to_string(),
        branch: Some("main".to_string()),
        commit_hash: Some("abc".to_string()),
        is_dirty: false,
        root_path: None,
    }
}

// ---------------------------------------------------------------------------
// create_share_text tests
// ---------------------------------------------------------------------------
#[test]
fn create_share_text_without_repo() {
    let metrics = make_metrics(150.0, 300.0, 3, 2);
    let text = SharingService::create_share_text(&metrics, &None);

    assert!(text.contains("150"), "should contain score");
    assert!(text.contains("300"), "should contain cpm");
    assert!(text.contains("5"), "should contain total mistakes (3+2)");
    assert!(text.contains("#gittype"));
    assert!(text.contains("github.com/unhappychoice/gittype"));
    // Should NOT contain repo info
    assert!(!text.contains("testuser"));
}

#[test]
fn create_share_text_with_repo() {
    let metrics = make_metrics(200.0, 400.0, 1, 0);
    let repo = make_repo();
    let text = SharingService::create_share_text(&metrics, &Some(repo));

    assert!(text.contains("200"), "should contain score");
    assert!(text.contains("400"), "should contain cpm");
    assert!(
        text.contains("testuser/testrepo"),
        "should contain repo info"
    );
    assert!(text.contains("#gittype"));
}

// ---------------------------------------------------------------------------
// generate_share_url tests — one per platform
// ---------------------------------------------------------------------------
#[test]
fn generate_share_url_x() {
    let metrics = make_metrics(100.0, 250.0, 2, 1);
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::X, &None);
    assert!(url.starts_with("https://x.com/intent/tweet?text="));
    assert!(url.contains("gittype"));
}

#[test]
fn generate_share_url_reddit() {
    let metrics = make_metrics(100.0, 250.0, 2, 1);
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::Reddit, &None);
    assert!(url.starts_with("https://www.reddit.com/submit?"));
    assert!(url.contains("title="));
    assert!(url.contains("selftext=true"));
    assert!(url.contains("text="));
}

#[test]
fn generate_share_url_linkedin() {
    let metrics = make_metrics(100.0, 250.0, 2, 1);
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::LinkedIn, &None);
    assert!(url.starts_with("https://www.linkedin.com/feed/"));
    assert!(url.contains("shareActive=true"));
}

#[test]
fn generate_share_url_facebook() {
    let metrics = make_metrics(100.0, 250.0, 2, 1);
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::Facebook, &None);
    assert!(url.starts_with("https://www.facebook.com/sharer/"));
    assert!(url.contains("quote="));
}

#[test]
fn generate_share_url_x_with_repo() {
    let metrics = make_metrics(300.0, 600.0, 0, 0);
    let repo = make_repo();
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::X, &Some(repo));
    assert!(url.starts_with("https://x.com/intent/tweet?text="));
    // URL-encoded repo name should be present
    assert!(url.contains("testuser"));
}

#[test]
fn generate_share_url_reddit_with_repo() {
    let metrics = make_metrics(300.0, 600.0, 0, 0);
    let repo = make_repo();
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::Reddit, &Some(repo));
    assert!(url.contains("reddit.com"));
    assert!(url.contains("title="));
}

#[test]
fn generate_share_url_linkedin_with_repo() {
    let metrics = make_metrics(300.0, 600.0, 0, 0);
    let repo = make_repo();
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::LinkedIn, &Some(repo));
    assert!(url.contains("linkedin.com"));
}

#[test]
fn generate_share_url_facebook_with_repo() {
    let metrics = make_metrics(300.0, 600.0, 0, 0);
    let repo = make_repo();
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::Facebook, &Some(repo));
    assert!(url.contains("facebook.com"));
}

// ---------------------------------------------------------------------------
// SharingPlatform Clone + Debug
// ---------------------------------------------------------------------------
#[test]
fn sharing_platform_clone() {
    let p = SharingPlatform::X;
    let p2 = p.clone();
    assert_eq!(p.name(), p2.name());
}
