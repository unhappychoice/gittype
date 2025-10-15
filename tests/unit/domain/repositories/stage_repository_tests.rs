use gittype::domain::repositories::stage_repository::StageRepository;

#[test]
fn new_creates_repository() {
    let result = StageRepository::new();
    assert!(result.is_ok());
}

#[test]
fn default_creates_repository() {
    let _repo = StageRepository::default();
    // Test passes if construction succeeds
}
