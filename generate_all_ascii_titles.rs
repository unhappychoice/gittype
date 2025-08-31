// Add the source modules to make RankingTitle available
#[path = "src/scoring/ranking_title.rs"]
mod ranking_title;

use ranking_title::RankingTitle;
use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all rank titles from the actual RankingTitle implementation
    let all_titles = RankingTitle::all_titles();
    println!(
        "Found {} rank titles from RankingTitle::all_titles()",
        all_titles.len()
    );

    let rank_titles: Vec<(&str, &str)> = all_titles
        .iter()
        .map(|title| (title.name(), title.color_palette()))
        .collect();

    println!(
        "Generating ASCII art for {} rank titles...",
        rank_titles.len()
    );

    let mut generated_patterns = HashMap::new();

    // Generate ASCII art for each title
    for (title, palette) in &rank_titles {
        print!("Generating '{}' with palette '{}'... ", title, palette);
        std::io::stdout().flush()?;

        // Run npx oh-my-logo
        let output = Command::new("npx")
            .args(["oh-my-logo", title, palette])
            .env("FORCE_COLOR", "1")
            .output()?;

        if output.status.success() {
            let ascii_output = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<String> = ascii_output
                .lines()
                .map(|line| line.to_string())
                .filter(|line| !line.trim().is_empty()) // Remove empty lines
                .collect();

            if !lines.is_empty() {
                generated_patterns.insert(title.to_string(), lines);
                println!("✓ ({} lines)", generated_patterns[*title].len());
            } else {
                println!("❌ (empty output)");
            }
        } else {
            println!("❌ (command failed)");
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.trim().is_empty() {
                eprintln!("Error: {}", stderr);
            }
        }
    }

    println!(
        "\nGenerated ASCII art for {} titles",
        generated_patterns.len()
    );

    // Generate the Rust file
    println!("Writing ascii_rank_titles_generated.rs...");

    let mut file = std::fs::File::create("src/game/ascii_rank_titles_generated.rs")?;

    writeln!(
        file,
        "/// ASCII art rank title patterns using oh-my-logo style with colored ANSI codes"
    )?;
    writeln!(
        file,
        "/// Auto-generated from oh-my-logo with different palettes for different rank categories"
    )?;
    writeln!(file)?;
    writeln!(file, "use std::collections::HashMap;")?;
    writeln!(file)?;
    writeln!(
        file,
        "pub fn get_all_rank_patterns() -> HashMap<String, Vec<String>> {{"
    )?;
    writeln!(file, "    let mut patterns = HashMap::new();")?;
    writeln!(file)?;

    // Sort titles for consistent output
    let mut sorted_titles: Vec<_> = generated_patterns.keys().collect();
    sorted_titles.sort();

    for title in sorted_titles {
        let lines = &generated_patterns[title];
        writeln!(file, "    // {}", title)?;
        writeln!(file, "    patterns.insert(\"{}\".to_string(), vec![", title)?;

        for line in lines {
            // Escape the line for Rust string literal
            let escaped_line = line
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\x1b", "\\x1b");
            writeln!(file, "        \"{}\".to_string(),", escaped_line)?;
        }

        writeln!(file, "    ]);")?;
        writeln!(file)?;
    }

    writeln!(file, "    patterns")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(
        file,
        "pub fn get_rank_title_display(rank_title: &str) -> Vec<String> {{"
    )?;
    writeln!(file, "    let patterns = get_all_rank_patterns();")?;
    writeln!(file, "    patterns.get(rank_title)")?;
    writeln!(file, "        .cloned()")?;
    writeln!(
        file,
        "        .unwrap_or_else(|| vec![rank_title.to_string()])"
    )?;
    writeln!(file, "}}")?;

    println!("✓ Generated src/game/ascii_rank_titles_generated.rs");
    println!("Total titles with ASCII art: {}", generated_patterns.len());

    // Show summary by tier
    let mut tier_counts = HashMap::new();
    for (title, palette) in &rank_titles {
        if generated_patterns.contains_key(*title) {
            *tier_counts.entry(*palette).or_insert(0) += 1;
        }
    }

    println!("\nGenerated ASCII art by tier:");
    for (palette, count) in &tier_counts {
        println!("  {}: {} titles", palette, count);
    }

    Ok(())
}
