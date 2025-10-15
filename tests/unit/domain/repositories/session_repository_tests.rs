use gittype::domain::repositories::session_repository::BestStatus;

#[test]
fn test_best_status_new() {
    let status = BestStatus::new();
    assert!(!status.is_todays_best);
    assert!(!status.is_weekly_best);
    assert!(!status.is_all_time_best);
    assert!(status.best_type.is_none());
    assert_eq!(status.todays_best_score, 0.0);
}
