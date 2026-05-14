use gittype::domain::models::typing::{InputResult, ProcessingOptions};
use gittype::domain::services::typing_core::TypingCore;

#[test]
fn new_normalizes_out_of_range_comment_ranges_without_panicking() {
    let core = TypingCore::new(
        "let value = 1;",
        &[(100, 120)],
        ProcessingOptions::default(),
    );

    assert_eq!(core.text_to_type(), "let value = 1;");
    assert_eq!(core.current_position_to_type(), 0);
}

#[test]
fn process_character_input_returns_no_action_after_completion() {
    let mut core = TypingCore::new("a", &[], ProcessingOptions::default());

    assert_eq!(core.process_character_input('a'), InputResult::Completed);
    assert_eq!(core.process_character_input('a'), InputResult::NoAction);
}

#[test]
fn process_enter_input_returns_no_action_after_completion() {
    let mut core = TypingCore::new("a\nb", &[], ProcessingOptions::default());

    assert_eq!(core.process_character_input('a'), InputResult::Correct);
    assert_eq!(core.process_enter_input(), InputResult::Correct);
    assert_eq!(core.process_character_input('b'), InputResult::Completed);
    assert_eq!(core.process_enter_input(), InputResult::NoAction);
}

#[test]
fn process_tab_input_accepts_internal_tabs() {
    let mut core = TypingCore::new("a\tb", &[], ProcessingOptions::default());

    assert_eq!(core.process_character_input('a'), InputResult::Correct);
    assert_eq!(core.process_tab_input(), InputResult::Correct);
    assert_eq!(core.process_character_input('b'), InputResult::Completed);
    assert_eq!(core.process_tab_input(), InputResult::NoAction);
}
