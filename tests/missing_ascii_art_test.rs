use gittype::game::ascii_rank_titles_generated::get_rank_title_display;
use gittype::models::RankingTitle;

#[test]
fn test_identify_missing_ascii_art() {
    let titles = RankingTitle::all_titles();

    let mut missing_art = Vec::new();
    let mut has_art = Vec::new();

    for title in &titles {
        let ascii_art = get_rank_title_display(title.name());

        if ascii_art.len() == 1 && ascii_art[0] == title.name() {
            // This is fallback behavior - no actual ASCII art
            missing_art.push(title.name());
        } else {
            has_art.push(title.name());
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
