use super::{CodeChunk, ProgressReporter};
use crate::extractor::core::CommonExtractor;
use crate::extractor::models::language::Language;
use crate::extractor::models::ExtractionOptions;
use crate::game::models::StepType;
use crate::Result;
use rayon::prelude::*;
use std::path::Path;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct CodeChunkExtractor;

impl CodeChunkExtractor {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn extract_chunks_from_files_with_progress<P: ProgressReporter + ?Sized>(
        &mut self,
        files_to_process: Vec<(std::path::PathBuf, Box<dyn Language>)>,
        _options: &ExtractionOptions,
        progress: &P,
    ) -> Result<Vec<CodeChunk>> {
        let total_files = files_to_process.len();

        // Sort files by size (largest first) for better loading progress perception
        // Cache metadata to avoid repeated filesystem calls
        let mut files_with_sizes: Vec<_> = files_to_process
            .into_iter()
            .map(|(path, lang)| {
                let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                (path, lang, size)
            })
            .collect();

        files_with_sizes.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by size (largest first)

        let files_to_process: Vec<_> = files_with_sizes
            .into_iter()
            .map(|(path, lang, _size)| (path, lang))
            .collect();

        // Initialize extracting progress from 0
        progress.set_file_counts(StepType::Extracting, 0, total_files, None);

        // Process all files in parallel with atomic progress tracking
        let processed = Arc::new(AtomicUsize::new(0));

        let all_chunks: Vec<CodeChunk> = files_to_process
            .into_par_iter()
            .flat_map(|(path, language)| {
                let result = Self::extract_from_file_static(&path, language.as_ref(), _options);

                // Update progress atomically
                let current = processed.fetch_add(1, Ordering::Relaxed) + 1;

                // Update progress display every 10 files or on completion
                if current % 10 == 0 || current == total_files {
                    progress.set_file_counts(StepType::Extracting, current, total_files, None);
                }

                match result {
                    Ok(file_chunks) => file_chunks,
                    Err(e) => {
                        eprintln!("Warning: Failed to extract from file {:?}: {}", path, e);
                        Vec::new()
                    }
                }
            })
            .collect();

        // Ensure final progress is exactly 100%
        progress.set_file_counts(StepType::Extracting, total_files, total_files, None);
        progress.set_current_file(None);

        Ok(all_chunks)
    }

    fn extract_from_file_static(
        file_path: &Path,
        language: &dyn Language,
        _options: &ExtractionOptions,
    ) -> Result<Vec<CodeChunk>> {
        CommonExtractor::extract_from_file(file_path, language.name())
    }
}
