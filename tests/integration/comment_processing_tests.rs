//! Tests for comment processing functionality
//! This module contains all tests related to comment range extraction, processing, and display.

#[cfg(test)]
mod comment_processing_tests {
    use gittype::extractor::challenge_converter::ChallengeConverter;
    use gittype::extractor::core::CommonExtractor;
    use gittype::game::typing_core::TypingCore;
    use std::path::Path;

    /// Tests for the core bug: byte vs character position misalignment
    mod byte_char_position_bugs {
        use super::*;

        #[test]
        fn test_original_indent_chars_fix() {
            // Test the bug in extract_line_indent_chars where byte positions are used as char counts

            // Simulate source code with multi-byte characters in indentation
            let source_with_multibyte_indent = "↵↵↵↵    // Some comment\ncode();";
            let lines: Vec<&str> = source_with_multibyte_indent.lines().collect();
            let first_line = lines[0]; // "↵↵↵↵    // Some comment"

            println!("=== Original Indent Bug Test ===");
            println!("First line: {:?}", first_line);
            println!("First line byte length: {}", first_line.len());
            println!("First line char length: {}", first_line.chars().count());

            // TreeSitter would return column position in BYTES for the comment start
            let comment_start_byte = first_line.find("//").unwrap(); // Should be 16 bytes
            let comment_start_char = first_line[..comment_start_byte].chars().count(); // Should be 8 chars

            println!(
                "Comment starts at byte {}, char {}",
                comment_start_byte, comment_start_char
            );

            // Bug: extract_line_indent_chars uses byte position as char count
            let buggy_indent_chars: String = first_line.chars().take(comment_start_byte).collect();
            let correct_indent_chars: String =
                first_line.chars().take(comment_start_char).collect();

            println!(
                "Buggy indent (taking {} chars): {:?}",
                comment_start_byte, buggy_indent_chars
            );
            println!(
                "Correct indent (taking {} chars): {:?}",
                comment_start_char, correct_indent_chars
            );

            // Now test the fixed version
            let fixed_indent_chars = CommonExtractor::extract_line_indent_chars_corrected(
                first_line,
                0,
                comment_start_byte,
            );

            println!("Fixed indent (corrected method): {:?}", fixed_indent_chars);

            // The fix should produce the correct result
            assert_eq!(
                fixed_indent_chars, correct_indent_chars,
                "Fixed version should match correct indentation"
            );
            assert_ne!(
                buggy_indent_chars, fixed_indent_chars,
                "Fixed version should differ from buggy version"
            );
        }

        #[test]
        fn test_real_models_options_file_reproduces_bug() {
            // Test the actual models/options.rs file that demonstrates the reported bug
            let options_path = Path::new("src/extractor/models/options.rs");

            // Extract chunks from the real file
            let chunks = CommonExtractor::extract_from_file(options_path, "rust").unwrap();

            // Also extract just the comment ranges for debugging
            let content = std::fs::read_to_string(options_path).unwrap();
            let tree =
                gittype::extractor::parsers::parse_with_thread_local("rust", &content).unwrap();
            let comment_ranges =
                CommonExtractor::extract_comment_ranges(&tree, &content, "rust").unwrap();

            println!("=== Real models/options.rs Test ===");
            println!("Found {} chunks", chunks.len());
            println!("Found {} comment ranges", comment_ranges.len());

            // Debug the specific comment range around "Java)"
            for (i, &(start, end)) in comment_ranges.iter().enumerate() {
                if start <= 340 && end >= 330 {
                    let comment_chars: Vec<char> = content.chars().collect();
                    if end <= comment_chars.len() {
                        let comment_text: String = comment_chars[start..end].iter().collect();
                        println!(
                            "Direct comment {}: ({}, {}) = {:?}",
                            i, start, end, comment_text
                        );

                        // Check if this contains "Java)"
                        if comment_text.contains("Java)") {
                            println!("*** Found comment containing Java): {:?}", comment_text);
                        }
                    }
                }
            }

            for (i, chunk) in chunks.iter().enumerate() {
                println!(
                    "Chunk {}: lines {}-{}, {} comment ranges",
                    i,
                    chunk.start_line,
                    chunk.end_line,
                    chunk.comment_ranges.len()
                );

                // Look for the specific chunk with lines 9-14 (the problematic one)
                if chunk.start_line <= 14 && chunk.end_line >= 9 {
                    println!(
                        "Found target chunk (lines {}-{})",
                        chunk.start_line, chunk.end_line
                    );
                    println!(
                        "Content: {:?}",
                        &chunk.content[..100.min(chunk.content.len())]
                    );
                    println!("Comment ranges: {:?}", chunk.comment_ranges);

                    // Debug the specific comment range that should contain Java)
                    for (i, &(start, end)) in chunk.comment_ranges.iter().enumerate() {
                        if start <= 340 && end >= 310 {
                            let chunk_chars: Vec<char> = chunk.content.chars().collect();
                            if end <= chunk_chars.len() {
                                let comment_text: String = chunk_chars[start..end].iter().collect();
                                println!(
                                    "Chunk comment {}: ({}, {}) = {:?}",
                                    i, start, end, comment_text
                                );

                                if comment_text.contains("Rust") || comment_text.contains("Maven") {
                                    println!("*** This is the problematic comment!");
                                    // Check what comes after position end
                                    let after_end = if end < chunk_chars.len() {
                                        chunk_chars[end..].iter().take(10).collect::<String>()
                                    } else {
                                        "END".to_string()
                                    };
                                    println!(
                                        "*** Characters after comment end ({}): {:?}",
                                        end, after_end
                                    );

                                    // Find where Java) appears in chunk
                                    if let Some(java_pos) = chunk.content.find("Java)") {
                                        println!("*** Java) found at chunk position: {}", java_pos);
                                        println!(
                                            "*** Comment ends at: {}, Java) at: {}",
                                            end, java_pos
                                        );
                                    }
                                }
                            }
                        }
                    }

                    // Convert to Challenge
                    let converter = ChallengeConverter::new();
                    let challenge = converter.convert_chunk_to_challenge(chunk.clone());
                    println!("Challenge created successfully");
                    println!("Challenge comment ranges: {:?}", challenge.comment_ranges);

                    // Debug: print the actual original comment vs display text around the problematic area
                    let original_chars: Vec<char> = challenge.code_content.chars().collect();
                    let problematic_comment = &original_chars[317..338];
                    println!(
                        "Original comment (317-338): {:?}",
                        problematic_comment.iter().collect::<String>()
                    );

                    // Create TypingCore and check for the bug
                    let typing_core = TypingCore::from_challenge(&challenge, None);

                    // Also check what's actually in the display text around that range
                    println!(
                        "Display text around 330-360: {:?}",
                        &typing_core.text_to_display()
                            [330..360.min(typing_core.text_to_display().len())]
                    );
                    let display_text = typing_core.text_to_display();
                    let display_ranges = typing_core.display_comment_ranges();

                    println!("Display text length: {}", display_text.len());
                    println!("Display ranges: {:?}", display_ranges);

                    // Look for the reported issues: "ges)" and "ava)" being typeable
                    if let Some(java_pos) = display_text.find("Java)") {
                        let ava_start = java_pos + 1; // "ava)" starts here
                        let ava_end = ava_start + 4; // "ava)" ends here

                        println!("Found 'Java)' at position {}", java_pos);
                        println!("Checking 'ava)' at positions {}-{}", ava_start, ava_end);
                        println!(
                            "Around Java): {:?}",
                            &display_text[java_pos.saturating_sub(10)
                                ..=(java_pos + 10).min(display_text.len() - 1)]
                        );

                        // Check which comment range should contain this
                        for (i, &(start, end)) in display_ranges.iter().enumerate() {
                            println!(
                                "Comment range {}: ({}, {}) = {:?}",
                                i,
                                start,
                                end,
                                &display_text.chars().collect::<Vec<char>>()
                                    [start..end.min(display_text.chars().count())]
                                    .iter()
                                    .collect::<String>()
                            );
                        }

                        // Debug mapping positions around Java)
                        println!("=== Mapping Debug for Java) area ===");
                        let mapping = typing_core.debug_mapping_to_display();
                        for i in 340..365 {
                            if i < mapping.len() {
                                println!("display[{}] -> original[{}]", i, mapping[i]);
                            }
                        }

                        // Find which display positions map to the chunk position 333 (where Java) starts)
                        println!("=== Reverse mapping for chunk Java) position ===");
                        for (display_pos, &orig_pos) in mapping.iter().enumerate() {
                            if (333..=337).contains(&orig_pos) {
                                // Around Java) position
                                println!("display[{}] -> original[{}]", display_pos, orig_pos);
                            }
                        }

                        // Check original comment ranges around this area
                        println!("=== Original Comment Ranges ===");
                        for (i, &(start, end)) in challenge.comment_ranges.iter().enumerate() {
                            if start <= 340 && end >= 330 {
                                let original_chars: Vec<char> =
                                    challenge.code_content.chars().collect();
                                if end <= original_chars.len() {
                                    let comment_text: String =
                                        original_chars[start..end].iter().collect();
                                    println!(
                                        "Original comment {}: ({}, {}) = {:?}",
                                        i, start, end, comment_text
                                    );
                                }
                            }
                        }

                        // Check if ava) is covered by comment ranges
                        let ava_in_comment = display_ranges
                            .iter()
                            .any(|&(start, end)| ava_start >= start && ava_end <= end);

                        println!("Is 'ava)' in comment? {}", ava_in_comment);

                        if !ava_in_comment {
                            println!(
                                "BUG REPRODUCED: 'ava)' is typeable but should be part of comment!"
                            );
                            // Don't panic yet, let's see the pattern first
                            // panic!("Bug reproduced: parts of comment are typeable");
                        }
                    }

                    // Similar check for "languages)" -> "ges)"
                    if let Some(lang_pos) = display_text.find("languages)") {
                        let ges_start = lang_pos + 6; // "ges)" starts here
                        let ges_end = ges_start + 4; // "ges)" ends here

                        println!("Found 'languages)' at position {}", lang_pos);
                        println!("Checking 'ges)' at positions {}-{}", ges_start, ges_end);

                        let ges_in_comment = display_ranges
                            .iter()
                            .any(|&(start, end)| ges_start >= start && ges_end <= end);

                        println!("Is 'ges)' in comment? {}", ges_in_comment);

                        if !ges_in_comment {
                            println!(
                                "BUG REPRODUCED: 'ges)' is typeable but should be part of comment!"
                            );
                            panic!("Bug reproduced: parts of comment are typeable");
                        }
                    }
                    break;
                }
            }
        }
    }

    /// Tests for comment range extraction and processing
    mod comment_range_extraction {
        use super::*;

        #[test]
        fn test_comment_with_special_chars() {
            // Test comments with various symbols that might cause issues
            let code = r#"fn test() {
    // Path symbols: ../.. and ./path and ~/home  
    // Unicode arrows: → ← ↑ ↓ and ↵ symbol
    // Mixed symbols: ../../config.json → ~/.config/
    let x = 42;
}"#;

            // Find all comment ranges
            let comment_ranges = vec![
                (
                    code.find("// Path symbols").unwrap(),
                    code.find("~/home").unwrap() + "~/home".len(),
                ),
                (
                    code.find("// Unicode arrows").unwrap(),
                    code.find("↵ symbol").unwrap() + "↵ symbol".len(),
                ),
                (
                    code.find("// Mixed symbols").unwrap(),
                    code.find("~/.config/").unwrap() + "~/.config/".len(),
                ),
            ];

            let typing_core = TypingCore::new(code, &comment_ranges, Default::default());
            let display_ranges = typing_core.display_comment_ranges();

            println!("=== Special Chars Test ===");
            for (i, &(start, end)) in display_ranges.iter().enumerate() {
                let comment = &typing_core.text_to_display()[start..end];
                println!("Comment {}: {:?}", i, comment);

                // All comments should be complete
                assert!(comment.starts_with("//"), "Comment should start with //");

                match i {
                    0 => assert!(comment.contains("~/home"), "Should contain ~/home"),
                    1 => assert!(comment.contains("↵ symbol"), "Should contain ↵ symbol"),
                    2 => assert!(comment.contains("~/.config/"), "Should contain ~/.config/"),
                    _ => {}
                }
            }
        }

        #[test]
        fn test_position_mapping_debug() {
            // Simple case to debug position mapping
            let code = "// test comment with ../../ path\ncode();";
            let comment_ranges = vec![(0, code.find('\n').unwrap())];

            println!("=== Position Mapping Debug ===");
            println!("Original: {:?}", code);
            println!("Comment range: {:?}", comment_ranges);

            let typing_core = TypingCore::new(code, &comment_ranges, Default::default());
            println!("Display: {:?}", typing_core.text_to_display());

            let display_ranges = typing_core.display_comment_ranges();
            println!("Display ranges: {:?}", display_ranges);

            if !display_ranges.is_empty() {
                let (start, end) = display_ranges[0];
                let comment = &typing_core.text_to_display()[start..end];
                println!("Final comment: {:?}", comment);

                assert!(comment.contains("../../"), "Should contain '../../'");
                assert!(
                    comment.contains("test comment"),
                    "Should contain 'test comment'"
                );
            }
        }
    }

    /// Tests for specific reported issues
    mod regression_tests {
        use super::*;

        #[test]
        fn test_duplicate_comment_strings() {
            // Test the case where same comment string appears multiple times
            let code = r#"// a

functionA();

// a

functionB();"#;

            // Debug the actual positions
            println!("Code: {:?}", code);
            let first_a_pos = code.find("// a").unwrap();
            let second_a_start = code.rfind("// a").unwrap();
            let second_a_end = second_a_start + 4;
            println!("First // a at: {}", first_a_pos);
            println!("Second // a at: {} to {}", second_a_start, second_a_end);

            let comment_ranges = vec![
                (first_a_pos, first_a_pos + 4), // First "// a"
                (second_a_start, second_a_end), // Second "// a"
            ];

            println!("=== Duplicate Comment Test ===");
            for (i, &(start, end)) in comment_ranges.iter().enumerate() {
                println!(
                    "Comment {}: ({}, {}) = {:?}",
                    i,
                    start,
                    end,
                    &code[start..end]
                );
            }

            let typing_core = TypingCore::new(code, &comment_ranges, Default::default());
            let display_ranges = typing_core.display_comment_ranges();

            println!("Display text: {:?}", typing_core.text_to_display());
            println!("Display ranges: {:?}", display_ranges);

            // Should find both comment ranges
            assert_eq!(
                display_ranges.len(),
                2,
                "Should find both duplicate comments"
            );

            for (i, &(start, end)) in display_ranges.iter().enumerate() {
                let display_text = typing_core.text_to_display();
                let comment = &display_text[start..end];
                println!("Final comment {}: {:?}", i, comment);

                assert_eq!(comment, "// a", "Comment should be exactly '// a'");
            }

            // Verify no overlapping ranges
            if display_ranges.len() >= 2 {
                let (start1, end1) = display_ranges[0];
                let (start2, end2) = display_ranges[1];
                assert!(
                    end1 <= start2 || end2 <= start1,
                    "Comment ranges should not overlap"
                );
            }
        }
    }
}
