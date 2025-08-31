use gittype::extractor::{ExtractionOptions, RepositoryLoader};
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_nested_and_oneline_structures() {
    let rust_code = r#"mod calculator {
    pub struct Calculator;

    impl Calculator {
        pub fn new() -> Self { Self }

        pub fn complex_calculation(&self, values: &[i32]) -> i32 {
            values.iter().sum()
        }
    }

    impl Default for Calculator {
        fn default() -> Self {
            Self::new()
        }
    }

    mod advanced {
        use super::Calculator;

        impl Calculator {
            pub fn advanced_method(&self) -> String {
                "advanced".to_string()
            }
        }
    }
}
"#;

    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().with_extension("rs");
    fs::write(&temp_path, rust_code).expect("Failed to write test file");

    let mut loader = RepositoryLoader::new().expect("Failed to create loader");
    let options = ExtractionOptions::default();

    let challenges = loader
        .load_challenges_from_repository(&temp_path, Some(options))
        .expect("Failed to load challenges");

    println!("Found {} challenges", challenges.len());

    for (i, challenge) in challenges.iter().enumerate() {
        println!("\n=== Challenge {} ===", i + 1);
        println!("Raw content:");
        for (line_num, line) in challenge.code_content.lines().enumerate() {
            println!("  {}: '{}'", line_num + 1, line);
        }

        // Apply processing (indentation normalization is now done in extractor)
        let (processed, mapped_ranges) = gittype::game::text_processor::TextProcessor::process_challenge_text_with_comment_mapping(
            &challenge.code_content,
            &challenge.comment_ranges
        );

        println!("\nFinal normalized content:");
        for (line_num, line) in processed.lines().enumerate() {
            println!("  {}: '{}'", line_num + 1, line);
        }

        println!("Comment ranges: {:?}", mapped_ranges);
    }

    let _ = fs::remove_file(&temp_path);
}

#[test]
fn test_comment_ranges_in_real_challenge() {
    let rust_code = r#"// Sample function with comments
fn calculate_sum(a: i32, b: i32) -> i32 {
    let result = a + b; // Add the numbers
    /*
     * Return the result
     */
    result
}
"#;

    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().with_extension("rs");
    fs::write(&temp_path, rust_code).expect("Failed to write test file");

    let mut loader = RepositoryLoader::new().expect("Failed to create loader");
    let options = ExtractionOptions::default();

    let challenges = loader
        .load_challenges_from_repository(&temp_path, Some(options))
        .expect("Failed to load challenges");

    println!("Found {} challenges for comment test", challenges.len());
    for (i, challenge) in challenges.iter().enumerate() {
        println!(
            "Challenge {}: '{}'",
            i + 1,
            challenge.code_content.replace('\n', "\\n")
        );
    }

    // The extractor now creates both function-based and file-based challenges
    assert!(!challenges.is_empty(), "Expected at least 1 challenge");

    let challenge = &challenges[0];
    println!("Challenge content: '{}'", challenge.code_content);
    println!("Comment ranges: {:?}", challenge.comment_ranges);

    let chars: Vec<char> = challenge.code_content.chars().collect();
    println!("Content length: {} chars", chars.len());

    for (start, end) in &challenge.comment_ranges {
        if *end <= chars.len() {
            let comment_text: String = chars[*start..*end].iter().collect();
            println!("Comment at {}-{}: '{}'", start, end, comment_text);

            // Verify it's actually a comment
            assert!(
                comment_text.starts_with("//") || comment_text.starts_with("/*"),
                "Text at {}-{} should be a comment but got: '{}'",
                start,
                end,
                comment_text
            );
        } else {
            panic!(
                "Comment range {}-{} exceeds content length {}",
                start,
                end,
                chars.len()
            );
        }
    }

    let _ = fs::remove_file(&temp_path);
}
