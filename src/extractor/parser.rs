use super::{CodeChunk, Language, NoOpProgressReporter, ProgressReporter};
use crate::extractor::core::CommonExtractor;
use crate::extractor::models::ExtractionOptions;
use crate::{GitTypeError, Result};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::path::Path;

pub struct CodeExtractor;

impl CodeExtractor {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn extract_chunks(
        &mut self,
        repo_path: &Path,
        _options: ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        self.extract_chunks_with_progress(repo_path, _options, &NoOpProgressReporter)
    }

    pub fn extract_chunks_with_progress<P: ProgressReporter + ?Sized>(
        &mut self,
        repo_path: &Path,
        _options: ExtractionOptions,
        progress: &P,
    ) -> Result<Vec<CodeChunk>> {
        progress.set_phase("Scanning repository".to_string());

        // Use ignore crate to respect .gitignore files
        let walker = WalkBuilder::new(repo_path)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .git_global(true) // Respect global gitignore
            .git_exclude(true) // Respect .git/info/exclude
            .build();

        // Collect all files to process first to get total count
        let mut files_to_process = Vec::new();
        for entry in walker {
            let entry =
                entry.map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                if let Some(language) = Language::from_extension(extension) {
                    if Self::should_process_file_static(path, &_options) {
                        files_to_process.push((path.to_path_buf(), language));
                    }
                }
            }
        }

        let total_files = files_to_process.len();
        progress.set_phase("Parsing AST".to_string());

        // Process files in parallel with better progress tracking
        // Split files into smaller chunks for better progress visibility
        let chunk_size = (total_files / 20).clamp(1, 10); // Process in smaller chunks of 1-10 files
        let mut all_chunks = Vec::new();
        let mut processed_files = 0;

        for chunk in files_to_process.chunks(chunk_size) {
            // Process this chunk in parallel
            let chunk_results: Result<Vec<Vec<CodeChunk>>> = chunk
                .par_iter()
                .map(|(path, language)| Self::extract_from_file_static(path, *language, &_options))
                .collect();

            // Update progress after each chunk
            processed_files += chunk.len();
            progress.set_file_counts(processed_files, total_files);

            // Update spinner for each chunk to show progress
            progress.update_spinner();

            // Collect results
            let chunk_results = chunk_results?;
            for file_chunks in chunk_results {
                all_chunks.extend(file_chunks);
            }
        }

        progress.set_file_counts(total_files, total_files);
        progress.set_current_file(None);
        progress.set_phase("Finalizing".to_string());

        Ok(all_chunks)
    }

    fn should_process_file_static(path: &Path, _options: &ExtractionOptions) -> bool {
        let path_str = path.to_string_lossy();

        // Check exclude patterns first
        for pattern in &_options.exclude_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false)
            {
                return false;
            }
        }

        // Check include patterns
        for pattern in &_options.include_patterns {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false)
            {
                return true;
            }
        }

        false
    }

    pub fn extract_from_file(
        &mut self,
        file_path: &Path,
        language: Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        Self::extract_from_file_static(file_path, language, _options)
    }

    fn extract_from_file_static(
        file_path: &Path,
        language: Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        CommonExtractor::extract_from_file(file_path, language)
    }

    #[allow(dead_code)]
    fn extract_chunks_from_tree(
        &self,
        tree: &tree_sitter::Tree,
        source_code: &str,
        file_path: &Path,
        language: Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        CommonExtractor::extract_chunks_from_tree(tree, source_code, file_path, language)
    }
}
