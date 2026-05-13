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
    assert!(
        text.contains("Mistakes: 5"),
        "should contain total mistakes (3+2)"
    );
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

#[test]
fn sharing_platform_debug_format_is_not_empty() {
    let labels: Vec<String> = SharingPlatform::all()
        .into_iter()
        .map(|p| format!("{:?}", p))
        .collect();

    assert_eq!(labels.len(), 4);
    assert!(labels.iter().all(|s| !s.is_empty()));
}

// ---------------------------------------------------------------------------
// share_result — under test-mocks, browser::open_url always returns Ok, so
// these tests exercise the success path (and the open_browser wrapper) without
// touching the real browser or the fallback URL display.
// ---------------------------------------------------------------------------
#[test]
fn share_result_opens_browser_for_every_platform_without_repo() {
    let metrics = make_metrics(125.0, 270.0, 1, 4);

    for platform in SharingPlatform::all() {
        let result = SharingService::share_result(&metrics, platform, &None);
        assert!(result.is_ok(), "share_result should succeed under mocks");
    }
}

#[test]
fn share_result_opens_browser_for_every_platform_with_repo() {
    let metrics = make_metrics(99.0, 175.0, 0, 1);
    let repo = make_repo();

    for platform in SharingPlatform::all() {
        let result = SharingService::share_result(&metrics, platform, &Some(repo.clone()));
        assert!(
            result.is_ok(),
            "share_result with repo should succeed under mocks"
        );
    }
}

// ---------------------------------------------------------------------------
// generate_share_url URL-encoding sanity checks
// ---------------------------------------------------------------------------
#[test]
fn generate_share_url_x_encodes_text_payload() {
    let metrics = make_metrics(100.0, 250.0, 2, 1);
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::X, &None);

    assert!(url.contains("text="));
    assert!(
        !url.contains(' '),
        "URL should be percent-encoded, found raw spaces in {url}"
    );
    assert!(!url.contains('\n'), "URL should not contain raw newlines");
}

#[test]
fn generate_share_url_reddit_includes_rank_name_in_title() {
    let metrics = make_metrics(0.0, 0.0, 0, 0);
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::Reddit, &None);

    let title_segment = url
        .split("title=")
        .nth(1)
        .and_then(|rest| rest.split('&').next())
        .expect("reddit URL must have a title= segment");

    let title = urlencoding::decode(title_segment).expect("title must be valid percent-encoding");
    assert!(title.contains("rank"));
    assert!(title.contains("0 points"));
}

#[test]
fn generate_share_url_facebook_encodes_repo_link_separately_from_quote() {
    let metrics = make_metrics(10.0, 20.0, 0, 0);
    let url = SharingService::generate_share_url(&metrics, &SharingPlatform::Facebook, &None);

    let u_segment = url
        .split("u=")
        .nth(1)
        .and_then(|rest| rest.split('&').next())
        .expect("facebook URL must have a u= segment");
    let quote_segment = url
        .split("quote=")
        .nth(1)
        .expect("facebook URL must have a quote= segment");

    let decoded_u = urlencoding::decode(u_segment).expect("u must be valid percent-encoding");
    let decoded_quote =
        urlencoding::decode(quote_segment).expect("quote must be valid percent-encoding");

    assert_eq!(decoded_u, "https://github.com/unhappychoice/gittype");
    assert!(decoded_quote.contains("gittype"));
}
