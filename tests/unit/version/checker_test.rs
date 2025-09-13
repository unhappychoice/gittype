use gittype::version::VersionChecker;

#[test]
fn test_parse_version() {
    assert_eq!(
        VersionChecker::parse_version("1.2.3").unwrap(),
        vec![1, 2, 3]
    );
    assert_eq!(
        VersionChecker::parse_version("0.5.1").unwrap(),
        vec![0, 5, 1]
    );
    assert!(VersionChecker::parse_version("1.0.0-beta").is_err());
}

#[test]
fn test_is_version_newer() {
    assert!(VersionChecker::is_version_newer("1.2.3", "1.2.2"));
    assert!(VersionChecker::is_version_newer("1.3.0", "1.2.9"));
    assert!(VersionChecker::is_version_newer("2.0.0", "1.9.9"));
    assert!(!VersionChecker::is_version_newer("1.2.3", "1.2.3"));
    assert!(!VersionChecker::is_version_newer("1.2.2", "1.2.3"));
    assert!(!VersionChecker::is_version_newer("1.2", "1.2.0"));
    assert!(VersionChecker::is_version_newer("1.2.0", "1.2"));

    // Test the specific case causing issues
    println!(
        "Testing 0.6.2 vs 0.6.2: {}",
        VersionChecker::is_version_newer("0.6.2", "0.6.2")
    );
    assert!(!VersionChecker::is_version_newer("0.6.2", "0.6.2"));
}

#[test]
fn test_current_version_exists() {
    assert!(!VersionChecker::CURRENT_VERSION.is_empty());
    assert!(VersionChecker::parse_version(VersionChecker::CURRENT_VERSION).is_ok());
}
