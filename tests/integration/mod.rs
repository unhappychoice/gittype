pub mod ascii_art_coverage_tests;
pub mod comment_processing_tests;
pub mod indent_treesitter_tests;
pub mod languages;
pub mod missing_ascii_art_test;

use gittype::domain::models::{Challenge, CodeChunk, ExtractionOptions, Language};
use gittype::domain::services::challenge_generator::ChallengeGenerator;
use gittype::domain::services::source_code_parser::parsers::parse_with_thread_local;
use gittype::domain::services::source_code_parser::CommonExtractor;
use gittype::domain::services::source_code_parser::LanguageRegistry;
use gittype::domain::services::source_code_parser::SourceCodeParser;
use gittype::domain::services::source_file_extractor::SourceFileExtractor;
use gittype::presentation::game::screens::loading_screen::NoOpProgressReporter;
use gittype::GitTypeError;
use gittype::Result;
use ignore::WalkBuilder;
use std::fs;
use std::path::{Path, PathBuf};

// Helper function for consistent test extraction options
pub fn test_extraction_options() -> ExtractionOptions {
    let mut options = ExtractionOptions::default();
    // Remove tmp/** pattern for tests since we use temp directories
    options.exclude_patterns.retain(|p| p != "**/tmp/**");
    options
}

// Helper function to collect files with languages from a directory
fn collect_files_with_languages(repo_path: &Path) -> Vec<(PathBuf, Box<dyn Language>)> {
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

// Test-specific helper functions
pub fn extract_from_file_for_test(file_path: &Path, language: &str) -> Result<Vec<CodeChunk>> {
    let content = fs::read_to_string(file_path)?;
    let tree = parse_with_thread_local(language, &content).ok_or_else(|| {
        GitTypeError::ExtractionFailed(format!("Failed to parse file: {:?}", file_path))
    })?;

    // Use parent directory as git_root for tests
    let git_root = file_path.parent().unwrap_or(Path::new("."));
    CommonExtractor::extract_chunks_from_tree(&tree, &content, file_path, git_root, language)
}

fn extract_chunks_from_scanned_files_for_test(
    scanned_files: &[PathBuf],
    _options: &ExtractionOptions,
) -> Result<Vec<CodeChunk>> {
    let mut all_chunks = Vec::new();

    // Convert scanned files to chunks using test function
    for file_path in scanned_files {
        if let Some(extension) = file_path.extension().and_then(|e| e.to_str()) {
            if let Some(language) = LanguageRegistry::from_extension(extension) {
                if let Ok(chunks) = extract_from_file_for_test(file_path, language.name()) {
                    all_chunks.extend(chunks);
                }
            }
        }
    }

    Ok(all_chunks)
}

// Helper function to extract chunks with NoOpProgressReporter for tests
pub fn extract_chunks_for_test(
    _extractor: &mut SourceCodeParser,
    repo_path: &Path,
    _options: ExtractionOptions,
) -> Result<Vec<CodeChunk>> {
    let files_to_process = collect_files_with_languages(repo_path);
    let mut all_chunks = Vec::new();

    for (file_path, language) in files_to_process {
        if let Ok(chunks) = extract_from_file_for_test(&file_path, language.name()) {
            all_chunks.extend(chunks);
        }
    }

    Ok(all_chunks)
}

pub fn extract_challenges_for_test(
    repo_extractor: &mut SourceFileExtractor,
    repo_path: &Path,
    options: ExtractionOptions,
) -> gittype::Result<Vec<Challenge>> {
    let files =
        repo_extractor.collect_source_files_with_progress(repo_path, &NoOpProgressReporter)?;

    let chunks = extract_chunks_from_scanned_files_for_test(&files, &options)?;

    let converter = ChallengeGenerator::new();
    let challenges = converter.convert_with_progress(chunks, &NoOpProgressReporter);

    Ok(challenges)
}
