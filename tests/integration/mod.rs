pub mod ascii_art_coverage_tests;
pub mod comment_processing_tests;
pub mod indent_treesitter_tests;
pub mod languages;
pub mod missing_ascii_art_test;

use gittype::extractor::models::language::LanguageRegistry;
use gittype::extractor::models::CodeChunk;
use gittype::extractor::{CodeChunkExtractor, ExtractionOptions, NoOpProgressReporter};
use gittype::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

// Helper function for consistent test extraction options
pub fn test_extraction_options() -> ExtractionOptions {
    let mut options = ExtractionOptions::default();
    // Remove tmp/** pattern for tests since we use temp directories
    options.exclude_patterns.retain(|p| p != "**/tmp/**");
    options
}

// Helper function to collect files with languages from a directory
fn collect_files_with_languages(
    repo_path: &Path,
) -> Vec<(PathBuf, Box<dyn gittype::extractor::Language>)> {
    WalkBuilder::new(repo_path)
        .hidden(false) // Include hidden files
        .git_ignore(true) // Respect .gitignore
        .git_global(true) // Respect global gitignore
        .git_exclude(true) // Respect .git/info/exclude
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| {
            let path = entry.path();
            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                LanguageRegistry::from_extension(extension)
                    .map(|language| (path.to_owned(), language))
            } else {
                None
            }
        })
        .collect()
}

// Helper function to extract chunks with NoOpProgressReporter for tests
pub fn extract_chunks_for_test(
    extractor: &mut CodeChunkExtractor,
    repo_path: &Path,
    options: ExtractionOptions,
) -> Result<Vec<CodeChunk>> {
    let files_to_process = collect_files_with_languages(repo_path);
    extractor.extract_chunks_from_files_with_progress(
        files_to_process,
        options,
        &NoOpProgressReporter,
    )
}

pub fn extract_challenges_for_test(
    repo_extractor: &mut gittype::extractor::RepositoryExtractor,
    repo_path: &Path,
    options: gittype::extractor::ExtractionOptions,
) -> gittype::Result<Vec<gittype::models::Challenge>> {
    use gittype::extractor::NoOpProgressReporter;

    // Step 1: Collect source files
    let files =
        repo_extractor.collect_source_files_with_progress(repo_path, &NoOpProgressReporter)?;

    // Step 2: Extract chunks from files
    let chunks = repo_extractor.extract_chunks_from_scanned_files_with_progress(
        &files,
        options,
        &NoOpProgressReporter,
    )?;

    // Step 3: Convert to challenges
    let challenges = repo_extractor.convert_chunks_and_files_to_challenges_with_progress(
        chunks,
        files,
        Some(repo_path),
        &NoOpProgressReporter,
    );

    Ok(challenges)
}
