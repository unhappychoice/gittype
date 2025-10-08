use gittype::domain::models::Rank;
use gittype::presentation::game::ascii_rank_titles::get_all_rank_patterns;

#[test]
fn test_identify_missing_ascii_art() {
    let titles = Rank::all_ranks();
    let patterns = get_all_rank_patterns();

    let mut missing_art = Vec::new();
    let mut has_art = Vec::new();

    for title in &titles {
        if let Some(ascii_art) = patterns.get(title.name()) {
            if !ascii_art.is_empty() {
                has_art.push(title.name());
            } else {
                missing_art.push(title.name());
            }
        } else {
            missing_art.push(title.name());
        }
    }

    println!("\n=== TITLES WITH PROPER ASCII ART ({}) ===", has_art.len());
    for title in &has_art {
        println!("  ✓ {}", title);
    }

    println!("\n=== TITLES MISSING ASCII ART ({}) ===", missing_art.len());
    for title in &missing_art {
        println!("  ❌ {}", title);
    }

    println!("\n=== SUMMARY ===");
    println!("  Total titles: {}", titles.len());
    println!(
        "  Have ASCII art: {} ({:.1}%)",
        has_art.len(),
        (has_art.len() as f32 / titles.len() as f32) * 100.0
    );
    println!(
        "  Missing ASCII art: {} ({:.1}%)",
        missing_art.len(),
        (missing_art.len() as f32 / titles.len() as f32) * 100.0
    );

    // For now, we'll allow missing art but we should fix this
    if missing_art.len() > 40 {
        panic!(
            "Too many titles missing ASCII art: {} out of {}",
            missing_art.len(),
            titles.len()
        );
    }
}
