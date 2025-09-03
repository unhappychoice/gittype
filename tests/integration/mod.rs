pub mod languages;

use gittype::extractor::ExtractionOptions;

pub fn test_extraction_options() -> ExtractionOptions {
    let mut options = ExtractionOptions::default();
    // Remove tmp/** pattern for tests since we're using temp directories
    options.exclude_patterns.retain(|p| p != "**/tmp/**");
    options
}
