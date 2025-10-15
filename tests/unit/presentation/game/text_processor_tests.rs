use gittype::presentation::game::text_processor::TextProcessor;

#[test]
fn process_challenge_text_empty_string() {
    let result = TextProcessor::process_challenge_text("");
    assert_eq!(result, "");
}

#[test]
fn process_challenge_text_single_line() {
    let result = TextProcessor::process_challenge_text("hello world");
    assert_eq!(result, "hello world");
}

#[test]
fn process_challenge_text_removes_trailing_whitespace() {
    let result = TextProcessor::process_challenge_text("hello world   ");
    assert_eq!(result, "hello world");
}

#[test]
fn process_challenge_text_removes_empty_lines() {
    let text = "line1\n\nline2";
    let result = TextProcessor::process_challenge_text(text);
    assert_eq!(result, "line1\nline2");
}

#[test]
fn process_challenge_text_multiple_lines() {
    let text = "line1\nline2\nline3";
    let result = TextProcessor::process_challenge_text(text);
    assert_eq!(result, "line1\nline2\nline3");
}

#[test]
fn process_challenge_text_preserves_leading_whitespace() {
    let text = "  indented";
    let result = TextProcessor::process_challenge_text(text);
    // Empty lines are filtered, but leading whitespace on non-empty lines is preserved
    assert_eq!(result, "  indented");
}

#[test]
fn calculate_line_starts_empty() {
    let starts = TextProcessor::calculate_line_starts("");
    assert_eq!(starts, vec![0]);
}

#[test]
fn calculate_line_starts_single_line() {
    let starts = TextProcessor::calculate_line_starts("hello");
    assert_eq!(starts, vec![0]);
}

#[test]
fn calculate_line_starts_multiple_lines() {
    let starts = TextProcessor::calculate_line_starts("hello\nworld\ntest");
    assert_eq!(starts, vec![0, 6, 12]);
}

#[test]
fn calculate_line_starts_with_newline_at_end() {
    let starts = TextProcessor::calculate_line_starts("hello\n");
    assert_eq!(starts, vec![0]);
}

#[test]
fn should_skip_final_newline_at_end() {
    let text = "hello\n";
    let result = TextProcessor::should_skip_final_newline(text, 5);
    assert!(result);
}

#[test]
fn should_skip_final_newline_not_at_end() {
    let text = "hello\nworld";
    let result = TextProcessor::should_skip_final_newline(text, 5);
    assert!(!result);
}

#[test]
fn should_skip_final_newline_out_of_bounds() {
    let text = "hello";
    let result = TextProcessor::should_skip_final_newline(text, 10);
    assert!(!result);
}

#[test]
fn is_rest_of_line_comment_only_empty() {
    let result = TextProcessor::is_rest_of_line_comment_only("", 0, &[]);
    assert!(!result);
}

#[test]
fn is_rest_of_line_comment_only_with_comment() {
    let text = "code // comment";
    let comment_ranges = vec![(5, 15)];
    let result = TextProcessor::is_rest_of_line_comment_only(text, 5, &comment_ranges);
    assert!(result);
}

#[test]
fn is_rest_of_line_comment_only_with_code() {
    let text = "code more_code";
    let result = TextProcessor::is_rest_of_line_comment_only(text, 5, &[]);
    assert!(!result);
}

#[test]
fn process_challenge_text_with_comment_mapping_empty() {
    let (text, ranges) = TextProcessor::process_challenge_text_with_comment_mapping("", &[]);
    assert_eq!(text, "");
    assert_eq!(ranges, vec![]);
}

#[test]
fn process_challenge_text_with_comment_mapping_no_comments() {
    let (text, ranges) =
        TextProcessor::process_challenge_text_with_comment_mapping("hello world", &[]);
    assert_eq!(text, "hello world");
    assert_eq!(ranges, vec![]);
}

#[test]
fn process_challenge_text_with_comment_mapping_preserve_empty_true() {
    let (text, _ranges) = TextProcessor::process_challenge_text_with_comment_mapping_preserve_empty(
        "line1\n\nline2",
        &[],
        true,
    );
    // With preserve_empty_lines = true, empty lines should be kept
    assert!(text.contains("\n\n") || text.lines().count() == 3);
}

#[test]
fn process_challenge_text_with_comment_mapping_preserve_empty_false() {
    let (text, _ranges) = TextProcessor::process_challenge_text_with_comment_mapping_preserve_empty(
        "line1\n\nline2",
        &[],
        false,
    );
    // With preserve_empty_lines = false, empty lines should be removed
    assert_eq!(text, "line1\nline2");
}

#[test]
fn should_skip_character_newline_at_end() {
    let text = "hello\n";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let result = TextProcessor::should_skip_character(text, 5, &line_starts, &[]);
    assert!(result); // Final newline should be skipped
}

#[test]
fn should_skip_character_regular_char() {
    let text = "hello";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let result = TextProcessor::should_skip_character(text, 0, &line_starts, &[]);
    assert!(!result);
}

#[test]
fn should_skip_character_in_comment() {
    let text = "code // comment";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let comment_ranges = vec![(5, 15)];
    let result = TextProcessor::should_skip_character(text, 8, &line_starts, &comment_ranges);
    assert!(result);
}

#[test]
fn is_at_end_of_line_content_at_newline() {
    let text = "hello\nworld";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let result = TextProcessor::is_at_end_of_line_content(text, 5, &line_starts, &[]);
    assert!(result);
}

#[test]
fn is_at_end_of_line_content_middle_of_line() {
    let text = "hello world";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let result = TextProcessor::is_at_end_of_line_content(text, 5, &line_starts, &[]);
    assert!(!result);
}

#[test]
fn is_at_end_of_line_content_at_end_of_text() {
    let text = "hello";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let result = TextProcessor::is_at_end_of_line_content(text, 5, &line_starts, &[]);
    assert!(!result); // Out of bounds
}
