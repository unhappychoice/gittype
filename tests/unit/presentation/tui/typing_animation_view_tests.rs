use gittype::domain::models::RankTier;
use gittype::presentation::tui::views::typing::typing_animation_view::{
    AnimationPhase, TypingAnimationView,
};
use std::time::Duration;

#[test]
fn new_starts_with_empty_concentration_state() {
    let view = TypingAnimationView::new(RankTier::Beginner, 80, 24);

    assert!(matches!(
        view.get_current_phase(),
        AnimationPhase::ConcentrationLines
    ));
    assert!(view.get_hacking_lines().is_empty());
    assert_eq!(view.get_current_line(), 0);
    assert_eq!(view.get_pause_dots(), 0);
    assert!(!view.is_complete());
}

#[test]
fn set_rank_messages_initializes_hacking_lines() {
    let mut view = TypingAnimationView::new(RankTier::Beginner, 80, 24);

    view.set_rank_messages("Hello World");

    let lines = view.get_hacking_lines();
    assert_eq!(lines.len(), 4);
    assert!(lines.iter().all(|line| !line.text.is_empty()));
    assert!(lines.iter().all(|line| line.typed_length == 0));
    assert!(lines.iter().all(|line| !line.completed));
    assert!(lines.iter().all(|line| line.start_time.is_none()));
}

#[test]
fn update_starts_typing_first_rank_message() {
    let mut view = TypingAnimationView::new(RankTier::Beginner, 80, 24);
    view.set_rank_messages("Hello World");

    assert!(view.update());

    let first_line = &view.get_hacking_lines()[0];
    assert_eq!(view.get_current_line(), 0);
    assert_eq!(first_line.typed_length, 1);
    assert!(!first_line.completed);
    assert!(first_line.start_time.is_some());
}

#[test]
fn completed_line_advances_after_delay() {
    let mut view = TypingAnimationView::new(RankTier::Beginner, 80, 24);
    view.set_rank_messages("Hello World");

    let first_line_len = view.get_hacking_lines()[0].text.len();
    (0..first_line_len).for_each(|_| {
        assert!(view.update());
        std::thread::sleep(Duration::from_millis(45));
    });

    assert!(view.update());
    assert_eq!(view.get_current_line(), 0);
    assert!(view.get_hacking_lines()[0].completed);
    assert!(view.get_hacking_lines()[0].completion_time.is_some());

    std::thread::sleep(Duration::from_millis(550));
    assert!(view.update());

    assert_eq!(view.get_current_line(), 1);
}

#[test]
fn empty_animation_progresses_from_pause_to_complete() {
    let mut view = TypingAnimationView::new(RankTier::Beginner, 80, 24);

    assert!(view.update());
    assert!(matches!(view.get_current_phase(), AnimationPhase::Pause));

    std::thread::sleep(Duration::from_millis(550));
    assert!(view.update());
    assert_eq!(view.get_pause_dots(), 1);

    std::thread::sleep(Duration::from_millis(3_100));
    assert!(view.update());
    assert!(matches!(view.get_current_phase(), AnimationPhase::Complete));
    assert!(view.is_complete());
    assert!(!view.update());
}
