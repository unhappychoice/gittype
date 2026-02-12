use crate::domain::models::loading::StepType;
use crate::domain::models::{ExtractionOptions, Languages};
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::infrastructure::storage::file_storage::FileStorageInterface;
use crate::presentation::tui::screens::loading_screen::ProgressReporter;
use crate::Result;
use std::path::{Path, PathBuf};

pub struct SourceFileExtractor {
    file_storage: FileStorage,
}

impl Default for SourceFileExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceFileExtractor {
    pub fn new() -> Self {
        Self {
            file_storage: FileStorage::new(),
        }
    }

    pub fn with_storage(file_storage: FileStorage) -> Self {
        Self { file_storage }
    }

    pub fn collect_with_progress(
        &self,
        repo_path: &Path,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<PathBuf>> {
        let options = ExtractionOptions::default();

        fn compile_patterns(patterns: &[String]) -> Vec<glob::Pattern> {
            patterns
                .iter()
                .filter_map(|p| glob::Pattern::new(p).ok())
                .collect()
        }

        let total_files_estimated = self.count_files(repo_path)?;

        let files = self.collect_files(
            repo_path,
            &compile_patterns(&options.include_patterns),
            &compile_patterns(&options.exclude_patterns),
            total_files_estimated,
            progress,
        )?;

        // Ensure final progress is exactly 100%
        progress.set_file_counts(
            StepType::Scanning,
            total_files_estimated,
            total_files_estimated,
            None,
        );

        Ok(files)
    }

    fn count_files(&self, repo_path: &Path) -> Result<usize> {
        let entries = self.file_storage.walk_directory(repo_path)?;
        Ok(entries.iter().filter(|entry| entry.is_file).count())
    }

    fn collect_files(
        &self,
        repo_path: &Path,
        include_patterns: &[glob::Pattern],
        exclude_patterns: &[glob::Pattern],
        total_files_estimated: usize,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<PathBuf>> {
        let entries = self.file_storage.walk_directory(repo_path)?;

        let files: Vec<PathBuf> = entries
            .into_iter()
            .enumerate()
            .filter(|(_, entry)| entry.is_file)
            .inspect(|(index, _)| {
                let processed = index + 1;
                if processed % 100 == 0 || processed == total_files_estimated {
                    progress.set_file_counts(
                        StepType::Scanning,
                        processed,
                        total_files_estimated,
                        None,
                    );
                }
            })
            .map(|(_, entry)| entry.path)
            .filter(|path| self.is_supported_language(path))
            .filter(|path| Self::should_collect(path, include_patterns, exclude_patterns))
            .collect();

        Ok(files)
    }

    fn is_supported_language(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|extension| Languages::from_extension(extension).is_some())
            .unwrap_or(false)
    }

    fn should_collect(
        path: &Path,
        include_patterns: &[glob::Pattern],
        exclude_patterns: &[glob::Pattern],
    ) -> bool {
        let path_str = path.to_string_lossy();

        if exclude_patterns.iter().any(|p| p.matches(&path_str)) {
            return false;
        }
        include_patterns.iter().any(|p| p.matches(&path_str))
    }
}
