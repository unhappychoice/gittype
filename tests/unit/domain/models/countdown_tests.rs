use std::time::Duration;

use gittype::domain::models::countdown::Countdown;

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
