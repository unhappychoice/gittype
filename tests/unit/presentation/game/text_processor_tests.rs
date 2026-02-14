use gittype::domain::services::text_processor::TextProcessor;

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

// ============================================
// should_skip_character - middle newline
// ============================================

#[test]
fn should_skip_character_newline_in_middle() {
    let text = "hello\nworld";
    let line_starts = TextProcessor::calculate_line_starts(text);
    // Newline at position 5 is NOT the final newline, so should NOT be skipped
    let result = TextProcessor::should_skip_character(text, 5, &line_starts, &[]);
    assert!(!result);
}

#[test]
fn should_skip_character_out_of_bounds() {
    let text = "hello";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let result = TextProcessor::should_skip_character(text, 100, &line_starts, &[]);
    assert!(!result);
}

// ============================================
// should_skip_character - whitespace before comment
// ============================================

#[test]
fn should_skip_character_whitespace_before_comment() {
    // "code  // comment" where comment starts at position 6
    let text = "code  // comment";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let comment_ranges = vec![(6, 16)];
    // Position 4 is space before the comment
    let result = TextProcessor::should_skip_character(text, 4, &line_starts, &comment_ranges);
    assert!(result); // whitespace before comment should be skipped
}

#[test]
fn should_skip_character_whitespace_not_before_comment() {
    let text = "code  more";
    let line_starts = TextProcessor::calculate_line_starts(text);
    // Position 4 is space, but no comment follows
    let result = TextProcessor::should_skip_character(text, 4, &line_starts, &[]);
    assert!(!result);
}

// ============================================
// process_challenge_text_with_comment_mapping - with comment ranges
// ============================================

#[test]
fn process_with_comment_mapping_maps_comments_correctly() {
    let text = "code // comment\nnext line";
    let comment_ranges = vec![(5, 15)];
    let (processed, mapped_ranges) =
        TextProcessor::process_challenge_text_with_comment_mapping(text, &comment_ranges);
    assert_eq!(processed, "code // comment\nnext line");
    assert!(!mapped_ranges.is_empty());
    // The comment range should be mapped to the same positions in processed text
    let (start, end) = mapped_ranges[0];
    assert_eq!(start, 5);
    assert!(end > start);
}

#[test]
fn process_with_comment_mapping_removes_empty_lines_and_adjusts_ranges() {
    let text = "code // comment\n\nnext line";
    let comment_ranges = vec![(5, 15)];
    let (processed, mapped_ranges) =
        TextProcessor::process_challenge_text_with_comment_mapping(text, &comment_ranges);
    // Empty line should be removed
    assert_eq!(processed, "code // comment\nnext line");
    // Comment range should still be valid in the processed text
    assert!(!mapped_ranges.is_empty());
}

#[test]
fn process_with_comment_mapping_comment_in_removed_line() {
    // Comment is entirely in an empty/removed section
    let text = "\n// only comment\ncode";
    // First line is empty, gets removed. Comment is on second line (positions 1-16)
    let comment_ranges = vec![(1, 17)];
    let (processed, mapped_ranges) =
        TextProcessor::process_challenge_text_with_comment_mapping(text, &comment_ranges);
    // "// only comment" line is not empty so should be kept
    assert!(processed.contains("code"));
    // mapped_ranges should still have the comment
    let _ = mapped_ranges; // just checking it doesn't panic
}

#[test]
fn process_with_comment_mapping_preserve_empty_with_comments() {
    let text = "code // comment\n\nnext line";
    let comment_ranges = vec![(5, 15)];
    let (processed, mapped_ranges) =
        TextProcessor::process_challenge_text_with_comment_mapping_preserve_empty(
            text,
            &comment_ranges,
            true,
        );
    // With preserve_empty = true, empty line should be kept
    assert!(processed.lines().count() >= 3);
    assert!(!mapped_ranges.is_empty());
}

// ============================================
// is_at_end_of_line_content - with comment ranges
// ============================================

#[test]
fn is_at_end_of_line_content_before_comment() {
    let text = "code // comment\nnext";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let comment_ranges = vec![(5, 15)];
    // Position 4 is space before comment - rest of line is comment only
    let result = TextProcessor::is_at_end_of_line_content(text, 4, &line_starts, &comment_ranges);
    assert!(result);
}

#[test]
fn is_at_end_of_line_content_before_code() {
    let text = "code more_code";
    let line_starts = TextProcessor::calculate_line_starts(text);
    let result = TextProcessor::is_at_end_of_line_content(text, 4, &line_starts, &[]);
    assert!(!result);
}

// ============================================
// is_rest_of_line_comment_only - edge cases
// ============================================

#[test]
fn is_rest_of_line_comment_only_whitespace_then_comment() {
    let text = "code   // comment";
    let comment_ranges = vec![(7, 17)];
    // Position 4 has spaces then comment
    let result = TextProcessor::is_rest_of_line_comment_only(text, 4, &comment_ranges);
    assert!(result);
}

#[test]
fn is_rest_of_line_comment_only_out_of_bounds() {
    let result = TextProcessor::is_rest_of_line_comment_only("hello", 100, &[]);
    assert!(!result);
}

#[test]
fn is_rest_of_line_comment_only_at_newline_boundary() {
    let text = "code // comment\nnext";
    let comment_ranges = vec![(5, 15)];
    let result = TextProcessor::is_rest_of_line_comment_only(text, 5, &comment_ranges);
    assert!(result);
}

// ============================================
// process_challenge_text - more edge cases
// ============================================

#[test]
fn process_challenge_text_only_whitespace_lines() {
    let text = "   \n  \n   ";
    let result = TextProcessor::process_challenge_text(text);
    assert_eq!(result, "");
}

#[test]
fn process_challenge_text_mixed_empty_and_content() {
    let text = "\nline1\n\n\nline2\n";
    let result = TextProcessor::process_challenge_text(text);
    assert_eq!(result, "line1\nline2");
}

#[test]
fn should_skip_final_newline_non_newline_char() {
    let text = "hello";
    let result = TextProcessor::should_skip_final_newline(text, 4);
    assert!(!result); // 'o' is not a newline
}
