use gittype::domain::repositories::stage_repository::StageRepository;

#[test]
fn test_new_creates_repository() {
    let result = StageRepository::new();
    assert!(result.is_ok());
}

#[test]
fn test_default_creates_repository() {
    let _repo = StageRepository::default();
    // Test passes if construction succeeds
}

#[test]
fn test_new_returns_ok_with_database() {
    let repo = StageRepository::new();
    assert!(repo.is_ok());

    // Verify we can use the repository
    let repo = repo.unwrap();
    let result = repo.get_stage_statistics(None);
    // Even if there's no data, the query should succeed
    assert!(result.is_ok());
}

#[test]
fn test_get_completed_stages_returns_ok() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_completed_stages(None);
    assert!(result.is_ok());
}

#[test]
fn test_get_completed_stages_with_repository_filter() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_completed_stages(Some(1));
    assert!(result.is_ok());
}

#[test]
fn test_get_completed_stages_by_language_returns_ok() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_completed_stages_by_language("rust", None);
    assert!(result.is_ok());
}

#[test]
fn test_get_completed_stages_by_language_with_repository() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_completed_stages_by_language("python", Some(1));
    assert!(result.is_ok());
}

#[test]
fn test_get_completed_stages_by_difficulty_returns_ok() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_completed_stages_by_difficulty("easy", None);
    assert!(result.is_ok());
}

#[test]
fn test_get_completed_stages_by_difficulty_with_repository() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_completed_stages_by_difficulty("medium", Some(1));
    assert!(result.is_ok());
}

#[test]
fn test_get_stage_statistics_returns_ok() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_stage_statistics(None);
    assert!(result.is_ok());

    let stats = result.unwrap();
    // Default statistics should have zero values
    assert!(stats.total_completed >= 0);
    assert!(stats.avg_wpm >= 0.0);
    assert!(stats.avg_accuracy >= 0.0);
}

#[test]
fn test_get_stage_statistics_with_repository_filter() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_stage_statistics(Some(1));
    assert!(result.is_ok());
}

#[test]
fn test_get_language_breakdown_returns_ok() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_language_breakdown(None);
    assert!(result.is_ok());

    let breakdown = result.unwrap();
    // Should return a vector (possibly empty)
    // Just verify it returns a valid vector
    let _ = breakdown.len();
}

#[test]
fn test_get_language_breakdown_with_repository() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_language_breakdown(Some(1));
    assert!(result.is_ok());
}

#[test]
fn test_get_difficulty_breakdown_returns_ok() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_difficulty_breakdown(None);
    assert!(result.is_ok());

    let breakdown = result.unwrap();
    // Should return a vector (possibly empty)
    // Just verify it returns a valid vector
    let _ = breakdown.len();
}

#[test]
fn test_get_difficulty_breakdown_with_repository() {
    let repo = StageRepository::new().unwrap();
    let result = repo.get_difficulty_breakdown(Some(1));
    assert!(result.is_ok());
}

#[test]
fn test_multiple_calls_to_same_repository() {
    let repo = StageRepository::new().unwrap();

    // Multiple calls should all succeed
    let result1 = repo.get_completed_stages(None);
    let result2 = repo.get_stage_statistics(None);
    let result3 = repo.get_language_breakdown(None);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
}
