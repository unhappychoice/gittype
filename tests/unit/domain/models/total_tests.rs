use gittype::domain::models::challenge::Challenge;
use gittype::domain::models::session::Session;
use gittype::domain::models::stage::Stage;
use gittype::domain::models::total::Total;
use gittype::domain::models::total::TotalResult;

fn sample_session(id: &str) -> Session {
    let stage = Stage::new(Challenge::new(id.into(), "fn main() {}".into()), 1);
    Session::new(vec![stage])
}

#[test]
fn total_result_finalize_resets_worst_metrics_when_unused() {
    let mut result = TotalResult::new();
    result.finalize();

    assert_eq!(result.worst_session_wpm, 0.0);
    assert_eq!(result.worst_session_accuracy, 0.0);
    assert!(result.total_duration.as_secs_f64() >= 0.0);
}

#[test]
fn total_result_completion_status_variants() {
    let mut result = TotalResult::new();
    assert_eq!(result.get_completion_status(), "No sessions attempted");

    result.total_sessions_attempted = 2;
    result.total_sessions_completed = 2;
    assert_eq!(
        result.get_completion_status(),
        "Perfect! 2 sessions completed"
    );

    result.total_sessions_completed = 1;
    assert_eq!(result.get_completion_status(), "1/2 sessions completed");
}

#[test]
fn total_result_share_text_contains_key_metrics() {
    let mut result = TotalResult::new();
    result.total_keystrokes = 1234;
    result.total_score = 9876.0;
    result.overall_cpm = 432.1;
    result.total_sessions_completed = 3;
    result.total_sessions_attempted = 4;
    result.total_duration = std::time::Duration::from_secs(600);

    let share = result.create_share_text();
    assert!(share.contains("1234 keystrokes"));
    assert!(share.contains("Total Score: 9876"));
    assert!(share.contains("CPM: 432"));
    assert!(share.contains("Sessions: 3/4"));
}

#[test]
fn total_add_session_appends_sessions() {
    let mut total = Total::new();
    assert!(total.sessions.is_empty());

    total.add_session(sample_session("a"));
    total.add_session(sample_session("b"));

    assert_eq!(total.sessions.len(), 2);
    assert_eq!(total.sessions[0].stages[0].challenge.id, "a");
    assert_eq!(total.sessions[1].stages[0].challenge.id, "b");
}
