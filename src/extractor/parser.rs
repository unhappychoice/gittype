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

    pub fn extract_chunks_with_progress<P: ProgressReporter>(
        &mut self,
        repo_path: &Path,
        _options: ExtractionOptions,
        progress: &P,
    ) -> Result<Vec<CodeChunk>> {
        progress.set_step(crate::game::models::loading_steps::StepType::Scanning);

        // First pass: count total files to process
        let walker_count = WalkBuilder::new(repo_path)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .git_global(true) // Respect global gitignore
            .git_exclude(true) // Respect .git/info/exclude
            .build();

        let mut total_files_to_process = 0;
        let mut scanned_count = 0;

        for entry in walker_count {
            let entry =
                entry.map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))?;
            let path = entry.path();

            scanned_count += 1;

            if scanned_count % 200 == 0 {}

            if !path.is_file() {
                continue;
            }

            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                if let Some(_language) = Language::from_extension(extension) {
                    if Self::should_process_file_static(path, &_options) {
                        total_files_to_process += 1;
                    }
                }
            }
        }

        // Second pass: actually collect files with proper progress
        let walker = WalkBuilder::new(repo_path)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .git_global(true) // Respect global gitignore
            .git_exclude(true) // Respect .git/info/exclude
            .build();

        let mut files_to_process = Vec::new();
        let mut processed_count = 0;

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
                        processed_count += 1;

                        // Update progress with known total
                        if processed_count % 10 == 0 || processed_count == total_files_to_process {
                            progress.set_file_counts(processed_count, total_files_to_process);
                        }
                    }
                }
            }
        }

        let total_files = files_to_process.len();
        progress.set_file_counts(total_files, total_files);
        progress.set_progress(1.0); // Scanning completed

        progress.set_step(crate::game::models::loading_steps::StepType::Extracting);

        // Process files in parallel with better progress tracking
        // Split files into smaller chunks for better progress visibility
        let chunk_size = (total_files / 20).clamp(1, 10); // Process in smaller chunks of 1-10 files
        let mut all_chunks = Vec::new();
        let mut processed_files = 0;

        for chunk in files_to_process.chunks(chunk_size) {
            // Process this chunk in parallel
            let chunk_results: Vec<Result<Vec<CodeChunk>>> = chunk
                .par_iter()
                .map(|(path, language)| Self::extract_from_file_static(path, *language, &_options))
                .collect();

            // Update progress after each chunk
            processed_files += chunk.len();
            progress.set_file_counts(processed_files, total_files);

            // Progress updates are now cheap - LoadingScreen controls rendering

            // Collect results, skip failed files but continue processing
            for (i, result) in chunk_results.into_iter().enumerate() {
                match result {
                    Ok(file_chunks) => {
                        all_chunks.extend(file_chunks);
                    }
                    Err(e) => {
                        let file_path = &chunk[i].0;
                        eprintln!(
                            "Warning: Failed to extract from file {:?}: {}",
                            file_path, e
                        );
                        // Continue processing other files instead of crashing
                    }
                }
            }
        }

        progress.set_file_counts(total_files, total_files);
        progress.set_current_file(None);
        progress.set_step(crate::game::models::loading_steps::StepType::Finalizing);

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
