use gittype::domain::repositories::version_repository::VersionRepository;

#[test]
fn new_creates_repository() {
    let result = VersionRepository::new_test();
    assert!(result.is_ok());
}
