use gittype::models::challenge::Challenge;
use gittype::models::stage::{Stage, StageResult};

fn sample_challenge() -> Challenge {
    Challenge::new("id".into(), "fn main() {}".into())
        .with_source_info("src/main.rs".into(), 1, 10)
}

#[test]
fn stage_new_assigns_fields() {
    let challenge = sample_challenge();
    let stage = Stage::new(challenge.clone(), 2);

    assert_eq!(stage.stage_number, 2);
    assert_eq!(stage.challenge.id, "id");
    assert_eq!(stage.challenge.start_line, Some(1));
}

#[test]
fn stage_result_default_values_are_sensible() {
    let stage_result = StageResult::default();

    assert_eq!(stage_result.rank_name, "Unranked");
    assert_eq!(stage_result.tier_name, "Beginner");
    assert_eq!(stage_result.completion_time.as_secs(), 0);
    assert!(!stage_result.was_skipped);
    assert!(!stage_result.was_failed);
    assert_eq!(stage_result.consistency_streaks, Vec::<usize>::new());
}
