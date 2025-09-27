use crate::domain::models::Language;
use crate::domain::models::{CodeChunk, ExtractionOptions};
use crate::domain::services::extractor::core::extractor::CommonExtractor;
use crate::domain::services::extractor::parsers::parse_with_thread_local;
use crate::infrastructure::git::LocalGitRepositoryClient;
use crate::presentation::game::models::StepType;
use crate::presentation::game::screens::loading_screen::ProgressReporter;
use crate::{GitTypeError, Result};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

pub struct CodeChunkExtractor;

impl CodeChunkExtractor {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn extract_chunks_from_files_with_progress<P: ProgressReporter + ?Sized>(
        &mut self,
        files_to_process: Vec<(PathBuf, Box<dyn Language>)>,
        options: &ExtractionOptions,
        progress: &P,
    ) -> Result<Vec<CodeChunk>> {
        let total_files = files_to_process.len();

        // Find git root once at the beginning using the first file
        let git_root = files_to_process
            .first()
            .map(|(first_file, _)| first_file)
            .and_then(|path| LocalGitRepositoryClient::get_repository_root(path))
            .ok_or_else(|| GitTypeError::ExtractionFailed("Git repository not found".to_string()))?;

        // Sort files by size (largest first) for better loading progress perception
        // Cache metadata to avoid repeated filesystem calls
        let mut files_with_sizes: Vec<_> = files_to_process
            .into_par_iter()
            .map(|(path, lang)| {
                let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                (path, lang, size)
            })
            .collect();

        files_with_sizes.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by size (largest first)

        // Convert back but keep size info for logging
        let files_to_process: Vec<_> = files_with_sizes
            .into_par_iter()
            .map(|(path, lang, _size)| (path, lang, _size))
            .collect();

        // Initialize extracting progress from 0
        progress.set_file_counts(StepType::Extracting, 0, total_files, None);

        // Process all files in parallel with atomic progress tracking
        let processed = Arc::new(AtomicUsize::new(0));

        // Use into_par_iter for better work-stealing efficiency
        let all_chunks: Vec<CodeChunk> = files_to_process
            .into_par_iter()
            .flat_map(|(path, language, _size)| {
                let result = Self::extract_from_file_static(&git_root, &path, language.as_ref(), options);

                // Update progress atomically
                let current = processed.fetch_add(1, Ordering::Relaxed) + 1;

                // Dynamic progress update frequency: every 10 files, but every 1 file when >99%
                let progress_threshold = if current as f64 / total_files as f64 > 0.99 {
                    1 // Update every file when >99%
                } else {
                    10 // Update every 10 files normally
                };

                if current.is_multiple_of(progress_threshold) || current == total_files {
                    progress.set_file_counts(StepType::Extracting, current, total_files, None);
                }

                result.unwrap_or(Vec::new())
            })
            .collect();

        // Ensure final progress is exactly 100%
        progress.set_file_counts(StepType::Extracting, total_files, total_files, None);
        progress.set_current_file(None);

        Ok(all_chunks)
    }

    fn extract_from_file_static(
        git_root: &Path,
        file_path: &Path,
        language: &dyn Language,
        options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let file_size = metadata.len();
            if file_size > options.max_file_size_bytes {
                log::warn!(
                    "Skipping large file: {:?} ({}MB > {}MB limit)",
                    file_path,
                    file_size / (1024 * 1024),
                    options.max_file_size_bytes / (1024 * 1024)
                );
                return Ok(Vec::new());
            }
        }

        let content = fs::read_to_string(file_path)?;
        let tree = parse_with_thread_local(language.name(), &content).ok_or_else(|| {
            GitTypeError::ExtractionFailed(format!("Failed to parse file: {:?}", file_path))
        })?;

        CommonExtractor::extract_chunks_from_tree(&tree, &content, file_path, git_root, language.name())
    }
}
