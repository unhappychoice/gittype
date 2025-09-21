use crossterm::style::Color as TerminalColor;
use gittype::models::rank::{Rank, RankTier};
use gittype::ui::Colors;

#[test]
fn rank_tier_color_palette_mappings_are_consistent() {
    assert_eq!(RankTier::Beginner.color_palette(), "grad-blue");
    assert_eq!(RankTier::Intermediate.color_palette(), "dawn");
    assert_eq!(RankTier::Advanced.color_palette(), "forest");
    assert_eq!(RankTier::Expert.color_palette(), "gold");
    assert_eq!(RankTier::Legendary.color_palette(), "fire");
}

#[test]
fn rank_tier_terminal_colors_follow_ui_colors() {
    assert_eq!(
        RankTier::Beginner.terminal_color(),
        Colors::to_crossterm(Colors::info())
    );
    assert_eq!(
        RankTier::Legendary.terminal_color(),
        Colors::to_crossterm(Colors::error())
    );
}

#[test]
fn rank_contains_score_checks_bounds_inclusively() {
    let rank = Rank::new("test", RankTier::Intermediate, 100, 200);
    assert!(rank.contains_score(100.0));
    assert!(rank.contains_score(150.0));
    assert!(rank.contains_score(200.0));
    assert!(!rank.contains_score(99.0));
    assert!(!rank.contains_score(201.0));
}

#[test]
fn rank_color_palette_delegates_to_tier() {
    let rank = Rank::new("test", RankTier::Advanced, 100, 200);
    assert_eq!(rank.color_palette(), "forest");
}

#[test]
fn rank_terminal_color_matches_tier_output() {
    let rank = Rank::new("test", RankTier::Expert, 100, 200);
    assert_eq!(rank.terminal_color(), RankTier::Expert.terminal_color());
}

#[test]
fn rank_for_score_returns_correct_rank() {
    let rank = Rank::for_score(6000.0);
    assert_eq!(rank.name(), "Junior Dev");
    assert_eq!(rank.tier(), &RankTier::Intermediate);
}

#[test]
fn rank_for_score_defaults_to_highest_when_exceeded() {
    let rank = Rank::for_score(999_999.0);
    assert_eq!(rank.tier(), &RankTier::Legendary);
    // Legendary tier uses error color which is now RGB
    assert!(matches!(rank.terminal_color(), TerminalColor::Rgb { .. }));
}
