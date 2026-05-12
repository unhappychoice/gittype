use gittype::domain::models::ui::ascii_rank_titles::get_rank_display;

#[test]
fn get_rank_display_returns_ascii_art_for_known_rank() {
    let display = get_rank_display("Compiler");

    assert!(display.len() > 1);
    assert!(display.iter().any(|line| line.contains("___")));
}

#[test]
fn get_rank_display_falls_back_to_rank_name_for_unknown_rank() {
    let display = get_rank_display("Unknown Rank");

    assert_eq!(display, vec!["Unknown Rank".to_string()]);
}
