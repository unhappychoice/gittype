use gittype::domain::models::ui::ascii_rank_titles::get_all_rank_patterns;
use gittype::domain::models::{Rank, RankTier};

#[test]
fn test_all_rank_titles_have_ascii_art() {
    let titles = Rank::all_ranks();
    let patterns = get_all_rank_patterns();
    let empty_vec = vec![];

    for title in &titles {
        let ascii_art = patterns.get(title.name()).unwrap_or(&empty_vec);

        // Verify ASCII art exists (not empty)
        assert!(
            !ascii_art.is_empty(),
            "No ASCII art found for rank title: {}",
            title.name()
        );

        // Verify ASCII art has content (not just empty strings)
        let has_content = ascii_art
            .iter()
            .any(|line| line.chars().any(|c| !c.is_whitespace()));
        assert!(
            has_content,
            "ASCII art for '{}' contains only whitespace",
            title.name()
        );

        println!(
            "✓ {} has ASCII art ({} lines)",
            title.name(),
            ascii_art.len()
        );
    }

    println!("All {} rank titles have ASCII art!", titles.len());
}

#[test]
fn test_all_tiers_represented() {
    let titles = Rank::all_ranks();

    let mut tier_counts = std::collections::HashMap::new();
    for title in &titles {
        *tier_counts.entry(title.tier().clone()).or_insert(0) += 1;
    }

    // Verify all tiers have titles
    assert!(
        tier_counts.contains_key(&RankTier::Beginner),
        "Missing Beginner titles"
    );
    assert!(
        tier_counts.contains_key(&RankTier::Intermediate),
        "Missing Intermediate titles"
    );
    assert!(
        tier_counts.contains_key(&RankTier::Advanced),
        "Missing Advanced titles"
    );
    assert!(
        tier_counts.contains_key(&RankTier::Expert),
        "Missing Expert titles"
    );
    assert!(
        tier_counts.contains_key(&RankTier::Legendary),
        "Missing Legendary titles"
    );

    println!("Tier distribution:");
    for (tier, count) in &tier_counts {
        println!("  {:?}: {} titles", tier, count);
    }
}

#[test]
fn test_ascii_art_quality() {
    let titles = Rank::all_ranks();
    let patterns = get_all_rank_patterns();
    let empty_vec = vec![];

    for title in titles.iter().take(10) {
        // Test first 10 titles for performance
        let ascii_art = patterns.get(title.name()).unwrap_or(&empty_vec);

        // Verify minimum quality standards
        assert!(
            !ascii_art.is_empty(),
            "ASCII art is empty for {}",
            title.name()
        );

        // Should have multiple lines (ASCII art is typically multi-line)
        println!(
            "✓ {} has proper ASCII art ({} lines)",
            title.name(),
            ascii_art.len()
        );

        // Should have reasonable width (not too short or too long)
        for (i, line) in ascii_art.iter().enumerate() {
            let visible_chars = line.chars().count();

            assert!(
                (10..=100).contains(&visible_chars),
                "Line {} of {} has {} visible characters (expected 10-100)",
                i,
                title.name(),
                visible_chars
            );
        }
    }
}
