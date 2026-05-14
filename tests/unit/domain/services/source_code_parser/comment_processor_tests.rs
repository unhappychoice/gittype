use gittype::domain::services::source_code_parser::CommentProcessor;

#[test]
fn convert_parent_comment_ranges_returns_empty_when_chunk_is_missing() {
    let ranges = CommentProcessor::convert_parent_comment_ranges_to_chunk(
        &[(0, 4)],
        &[0, 1, 2, 3, 4],
        "let value = 1;",
        "missing chunk",
    );

    assert!(ranges.is_empty());
}
