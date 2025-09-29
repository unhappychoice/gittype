use gittype::domain::services::source_code_parser::parsers::parse_with_thread_local;
use gittype::domain::services::source_code_parser::CommonExtractor;
use gittype::presentation::game::typing_core::{ProcessingOptions, TypingCore};
use insta::assert_snapshot;
use std::fmt;

#[derive(Debug)]
pub struct TypingSnapshot {
    pub text_original: String,
    pub text_to_type: String,
    pub text_to_display: String,
}

impl fmt::Display for TypingSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "# text_original\n{}\n\n# text_to_type\n{}\n\n# text_to_display\n{}\n",
            self.text_original, self.text_to_type, self.text_to_display
        )
    }
}

/// Test case for typing core functionality
pub struct TypingCoreTestCase {
    pub name: &'static str,
    pub code: &'static str,
    pub comment_ranges: Vec<(usize, usize)>,
    pub options: Option<ProcessingOptions>,
}

/// Common function to verify typing simulation works correctly
pub fn verify_typing_simulation(core: &mut TypingCore) {
    let mut chars_typed = Vec::new();

    while let Some(current_char_type) = core.current_char_to_type() {
        let current_char_display = core
            .text_to_display()
            .chars()
            .nth(core.current_position_to_display());

        // Test display position mapping
        let display_pos = core.current_position_to_display();
        println!(
            "Position {}: type_char='{}', display_char={:?}, display_pos={:?}",
            core.current_position_to_type(),
            current_char_type,
            current_char_display,
            display_pos
        );

        // Verify current_char_type == current_char_display (with special handling for newlines)
        match current_char_display {
            Some(display_char) => {
                if current_char_type == '\n' && display_char == '↵' {
                    // This is expected: newlines in type text map to ↵ symbols in display text
                    println!("  -> Newline mapping OK: '\\n' -> '↵'");
                } else if current_char_type == '\t' && display_char == '→' {
                    // This is expected: tabs in type text map to → symbols in display text
                    println!("  -> Tab mapping OK: '\\t' -> '→'");
                } else if current_char_type != display_char {
                    panic!(
                        "Mismatch at position {}: type_char='{}' != display_char='{}'",
                        core.current_position_to_type(),
                        current_char_type,
                        display_char
                    );
                }
            }
            None => {
                panic!(
                    "No display character at position {}",
                    core.current_position_to_type()
                );
            }
        }

        chars_typed.push(current_char_type);
        core.advance_to_next_character();
    }

    println!("Total characters typed: {}", chars_typed.len());
}

/// Create a snapshot for typing core test
pub fn create_typing_snapshot(core: &TypingCore, original_text: &str) -> TypingSnapshot {
    TypingSnapshot {
        text_original: original_text.to_string(),
        text_to_type: core.text_to_type().to_string(),
        text_to_display: core.text_to_display().to_string(),
    }
}

/// Run a complete typing core test
pub fn run_typing_core_test(test_case: TypingCoreTestCase) {
    let options = test_case.options.unwrap_or_default();
    let mut core = TypingCore::new(test_case.code, &test_case.comment_ranges, options);

    // Verify typing simulation works correctly
    verify_typing_simulation(&mut core);

    let snapshot = create_typing_snapshot(&core, test_case.code);
    assert_snapshot!(test_case.name, snapshot);
}

/// Find comment ranges using the same parser as CodeChunk extraction
pub fn find_comment_ranges_with_parser(code: &str, language: &str) -> Vec<(usize, usize)> {
    if let Some(tree) = parse_with_thread_local(language, code) {
        CommonExtractor::extract_comment_ranges(&tree, code, language, &[]).unwrap_or_default()
    } else {
        Vec::new()
    }
}

/// Macro to define typing core test cases using the real parser
#[macro_export]
macro_rules! typing_core_test_with_parser {
    ($test_name:ident, $lang:expr, $code:expr) => {
        #[test]
        fn $test_name() {
            use $crate::integration::languages::typing_core_common::*;

            let comment_ranges = find_comment_ranges_with_parser($code, $lang);
            let test_case = TypingCoreTestCase {
                name: stringify!($test_name),
                code: $code,
                comment_ranges,
                options: None,
            };

            run_typing_core_test(test_case);
        }
    };

    ($test_name:ident, $lang:expr, $code:expr, $options:expr) => {
        #[test]
        fn $test_name() {
            use $crate::integration::languages::typing_core_common::*;

            let comment_ranges = find_comment_ranges_with_parser($code, $lang);
            let test_case = TypingCoreTestCase {
                name: stringify!($test_name),
                code: $code,
                comment_ranges,
                options: Some($options),
            };

            run_typing_core_test(test_case);
        }
    };
}
