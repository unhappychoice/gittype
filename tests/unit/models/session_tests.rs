use gittype::models::challenge::Challenge;
use gittype::models::session::Session;
use gittype::models::session::SessionResult;
use gittype::models::stage::Stage;

fn sample_stage(id: &str) -> Stage {
    let challenge = Challenge::new(id.into(), "fn main() {}".into());
    Stage::new(challenge, 1)
}

#[test]
fn session_new_preserves_stage_list() {
    let stages = vec![sample_stage("a"), sample_stage("b")];
    let session = Session::new(stages.clone());

    assert_eq!(session.stages.len(), 2);
    assert_eq!(session.stages[0].challenge.id, "a");
    assert_eq!(session.stages[1].challenge.id, "b");
}

#[test]
fn session_result_completion_status_covers_cases() {
    let mut result = SessionResult::new();
    assert_eq!(
        result.get_session_completion_status(),
        "No challenges attempted"
    );

    result.stages_completed = 2;
    assert_eq!(
        result.get_session_completion_status(),
        "Perfect session! 2 challenges completed"
    );

    result.stages_skipped = 1;
    assert_eq!(
        result.get_session_completion_status(),
        "2 completed, 1 skipped"
    );
}

#[test]
fn session_result_new_initializes_metrics() {
    let result = SessionResult::new();

    assert_eq!(result.stages_completed, 0);
    assert_eq!(result.overall_accuracy, 0.0);
    assert_eq!(result.worst_stage_wpm, f64::MAX);
    assert!(!result.session_successful);
}
