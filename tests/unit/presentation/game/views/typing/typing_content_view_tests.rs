use gittype::presentation::tui::views::typing::typing_content_view::TypingContentView;

#[test]
fn test_calculate_scroll_offset() {
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 10), 0);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 15), 5);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 30), 20);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 25, 50), 5);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 30, 15), 5);
    assert_eq!(TypingContentView::calculate_scroll_offset(20, 100, 5), 0);
}
