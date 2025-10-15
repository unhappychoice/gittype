use crossterm::style::Color as TerminalColor;
use gittype::domain::models::rank::{Rank, RankTier};

#[test]
fn rank_tier_color_palette_mappings_are_consistent() {
    assert_eq!(RankTier::Beginner.color_palette(), "grad-blue");
    assert_eq!(RankTier::Intermediate.color_palette(), "dawn");
    assert_eq!(RankTier::Advanced.color_palette(), "forest");
    assert_eq!(RankTier::Expert.color_palette(), "gold");
    assert_eq!(RankTier::Legendary.color_palette(), "fire");
}

#[test]
fn rank_tier_terminal_colors_are_rgb() {
    // Just verify that terminal colors are RGB variants
    assert!(matches!(
        RankTier::Beginner.terminal_color(),
        TerminalColor::Rgb { .. }
    ));
    assert!(matches!(
        RankTier::Intermediate.terminal_color(),
        TerminalColor::Rgb { .. }
    ));
    assert!(matches!(
        RankTier::Advanced.terminal_color(),
        TerminalColor::Rgb { .. }
    ));
    assert!(matches!(
        RankTier::Expert.terminal_color(),
        TerminalColor::Rgb { .. }
    ));
    assert!(matches!(
        RankTier::Legendary.terminal_color(),
        TerminalColor::Rgb { .. }
    ));
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

#[test]
fn rank_tier_clone() {
    let tier = RankTier::Advanced;
    let cloned = tier.clone();
    assert_eq!(tier, cloned);
}

#[test]
fn rank_tier_equality() {
    assert_eq!(RankTier::Beginner, RankTier::Beginner);
    assert_ne!(RankTier::Beginner, RankTier::Legendary);
}

#[test]
fn rank_new() {
    let rank = Rank::new("Test Rank", RankTier::Advanced, 1000, 2000);
    assert_eq!(rank.name(), "Test Rank");
    assert_eq!(rank.tier(), &RankTier::Advanced);
    assert_eq!(rank.min_score, 1000);
    assert_eq!(rank.max_score, 2000);
}

#[test]
fn rank_all_ranks_count() {
    let ranks = Rank::all_ranks();
    assert_eq!(ranks.len(), 63); // 12+12+12+12+15
}

#[test]
fn rank_all_ranks_first() {
    let ranks = Rank::all_ranks();
    assert_eq!(ranks[0].name(), "Hello World");
    assert_eq!(ranks[0].tier(), &RankTier::Beginner);
    assert_eq!(ranks[0].min_score, 0);
}

#[test]
fn rank_all_ranks_last() {
    let ranks = Rank::all_ranks();
    let last = &ranks[ranks.len() - 1];
    assert_eq!(last.name(), "Kernel Panic");
    assert_eq!(last.tier(), &RankTier::Legendary);
}

#[test]
fn rank_for_score_zero() {
    let rank = Rank::for_score(0.0);
    assert_eq!(rank.name(), "Hello World");
}

#[test]
fn rank_for_score_advanced() {
    let rank = Rank::for_score(8500.0);
    assert_eq!(rank.tier(), &RankTier::Advanced);
}

#[test]
fn rank_clone() {
    let rank = Rank::new("Test", RankTier::Advanced, 1000, 2000);
    let cloned = rank.clone();
    assert_eq!(rank.name(), cloned.name());
    assert_eq!(rank.tier(), cloned.tier());
}

#[test]
fn rank_partial_eq() {
    let rank1 = Rank::new("Test", RankTier::Advanced, 1000, 2000);
    let rank2 = Rank::new("Test", RankTier::Advanced, 1000, 2000);
    assert_eq!(rank1, rank2);
}
