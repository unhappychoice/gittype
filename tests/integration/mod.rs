pub mod ascii_art_coverage_tests;
pub mod comment_processing_tests;
pub mod indent_treesitter_tests;
pub mod languages;
pub mod missing_ascii_art_test;

use gittype::extractor::ExtractionOptions;

// Helper function for consistent test extraction options
pub fn test_extraction_options() -> ExtractionOptions {
    let mut options = ExtractionOptions::default();
    // Remove tmp/** pattern for tests since we use temp directories
    options.exclude_patterns.retain(|p| p != "**/tmp/**");
    options
}
