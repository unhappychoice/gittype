use gittype::game::ascii_rank_titles_generated::get_rank_title_display;
use gittype::models::{RankTier, Rank};

#[test]
fn test_all_rank_titles_have_ascii_art() {
    let titles = Rank::all_ranks();

    for title in &titles {
        let ascii_art = get_rank_title_display(title.name());

        // Verify ASCII art exists (not empty)
        assert!(
            !ascii_art.is_empty(),
            "No ASCII art found for rank title: {}",
            title.name()
        );

        // Verify ASCII art has content (not just empty strings)
        let has_content = ascii_art
            .iter()
            .any(|line| line.chars().any(|c| !c.is_whitespace() && c != '\x1b'));
        assert!(
            has_content,
            "ASCII art for '{}' contains only whitespace or ANSI codes",
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

    for title in titles.iter().take(10) {
        // Test first 10 titles for performance
        let ascii_art = get_rank_title_display(title.name());

        // Verify minimum quality standards
        assert!(
            !ascii_art.is_empty(),
            "ASCII art is empty for {}",
            title.name()
        );

        // Should have multiple lines (ASCII art is typically multi-line) - allow fallback of 1 line
        if ascii_art.len() == 1 {
            println!("⚠️  {} only has fallback text (1 line)", title.name());
        } else {
            println!(
                "✓ {} has proper ASCII art ({} lines)",
                title.name(),
                ascii_art.len()
            );
        }

        // Should contain ANSI color codes (our ASCII art is colored)
        let has_ansi = ascii_art.iter().any(|line| line.contains("\x1b["));
        assert!(
            has_ansi,
            "ASCII art for {} lacks ANSI color codes",
            title.name()
        );

        // Should have reasonable width (not too short or too long)
        for (i, line) in ascii_art.iter().enumerate() {
            // Remove ANSI escape sequences to count only visible characters
            let cleaned_line = remove_ansi_sequences(line);
            let visible_chars = cleaned_line.chars().count();

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

fn remove_ansi_sequences(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip escape sequence: ESC [ ... m
            if let Some('[') = chars.next() {
                // Skip until we find 'm' (end of color sequence)
                for c in chars.by_ref() {
                    if c == 'm' {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}
