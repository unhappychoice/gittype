use gittype::domain::models::countdown::Countdown;
use std::time::Duration;

#[test]
fn countdown_new_is_inactive() {
    let countdown = Countdown::new();

    assert!(!countdown.is_active());
    assert_eq!(countdown.get_current_count(), None);
}

#[test]
fn countdown_default_is_inactive() {
    let countdown = Countdown::default();

    assert!(!countdown.is_active());
    assert_eq!(countdown.get_current_count(), None);
}

#[test]
fn start_countdown_activates_countdown() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    assert!(countdown.is_active());
}

#[test]
fn start_countdown_sets_current_number_to_3() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    assert_eq!(countdown.get_current_count(), Some(3));
}

#[test]
fn get_current_count_returns_none_when_inactive() {
    let countdown = Countdown::new();

    assert_eq!(countdown.get_current_count(), None);
}

#[test]
fn get_current_count_returns_number_when_active() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    assert!(countdown.get_current_count().is_some());
}

#[test]
fn update_state_returns_none_when_inactive() {
    let mut countdown = Countdown::new();

    assert_eq!(countdown.update_state(), None);
}

#[test]
fn update_state_returns_none_while_counting() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    // Immediately after start, should still be counting
    assert_eq!(countdown.update_state(), None);
}

#[test]
fn countdown_advances_through_numbers_and_finishes() {
    let mut countdown = Countdown::new();
    assert!(!countdown.is_active());

    countdown.start_countdown();
    assert!(countdown.is_active());
    assert_eq!(countdown.get_current_count(), Some(3));

    countdown.fast_forward_for_test(Duration::from_millis(650));
    countdown.update_state();
    assert_eq!(countdown.get_current_count(), Some(2));

    countdown.fast_forward_for_test(Duration::from_millis(650));
    countdown.update_state();
    assert_eq!(countdown.get_current_count(), Some(1));

    countdown.fast_forward_for_test(Duration::from_millis(650));
    countdown.update_state();
    assert_eq!(countdown.get_current_count(), Some(0));

    countdown.fast_forward_for_test(Duration::from_millis(450));
    let finished_at = countdown.update_state();
    assert!(finished_at.is_some());
    assert!(!countdown.is_active());
    assert_eq!(countdown.get_current_count(), None);
}

#[test]
fn update_state_transitions_from_3_to_2() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    assert_eq!(countdown.get_current_count(), Some(3));

    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();

    assert_eq!(countdown.get_current_count(), Some(2));
}

#[test]
fn update_state_transitions_from_2_to_1() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();

    assert_eq!(countdown.get_current_count(), Some(2));

    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();

    assert_eq!(countdown.get_current_count(), Some(1));
}

#[test]
fn update_state_transitions_from_1_to_go() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();
    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();

    assert_eq!(countdown.get_current_count(), Some(1));

    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();

    assert_eq!(countdown.get_current_count(), Some(0));
}

#[test]
fn countdown_pause_and_resume_holds_state() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    countdown.fast_forward_for_test(Duration::from_millis(300));
    countdown.pause();
    let before_pause = countdown.get_current_count();

    countdown.fast_forward_for_test(Duration::from_millis(500));
    countdown.update_state();
    assert_eq!(countdown.get_current_count(), before_pause);

    countdown.resume();
    countdown.fast_forward_for_test(Duration::from_millis(350));
    countdown.update_state();
    assert_ne!(countdown.get_current_count(), before_pause);
}

#[test]
fn pause_when_already_paused_does_nothing() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();
    countdown.pause();
    countdown.pause();

    assert!(countdown.is_active());
    assert_eq!(countdown.get_current_count(), Some(3));
}

#[test]
fn pause_when_inactive_does_nothing() {
    let mut countdown = Countdown::new();
    countdown.pause();

    assert!(!countdown.is_active());
    assert_eq!(countdown.get_current_count(), None);
}

#[test]
fn resume_when_not_paused_does_nothing() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();
    countdown.resume();

    assert!(countdown.is_active());
    assert_eq!(countdown.get_current_count(), Some(3));
}

#[test]
fn is_active_returns_false_initially() {
    let countdown = Countdown::new();
    assert!(!countdown.is_active());
}

#[test]
fn is_active_returns_true_after_start() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();
    assert!(countdown.is_active());
}

#[test]
fn is_active_returns_false_after_completion() {
    let mut countdown = Countdown::new();
    countdown.start_countdown();

    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();
    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();
    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();
    countdown.fast_forward_for_test(Duration::from_millis(500));
    countdown.update_state();

    assert!(!countdown.is_active());
}

#[test]
fn countdown_can_be_restarted() {
    let mut countdown = Countdown::new();

    countdown.start_countdown();
    assert_eq!(countdown.get_current_count(), Some(3));

    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();
    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();
    countdown.fast_forward_for_test(Duration::from_millis(700));
    countdown.update_state();
    countdown.fast_forward_for_test(Duration::from_millis(500));
    countdown.update_state();

    assert!(!countdown.is_active());

    countdown.start_countdown();
    assert!(countdown.is_active());
    assert_eq!(countdown.get_current_count(), Some(3));
}
