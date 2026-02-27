use crate::domain::models::loading::StepType;
use crate::domain::models::{ExtractionOptions, Languages};
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::infrastructure::storage::file_storage::FileStorageInterface;
use crate::presentation::tui::screens::loading_screen::ProgressReporter;
use crate::Result;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
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
        self.collect_with_progress_with_options(repo_path, &options, progress)
    }

    pub fn collect_with_progress_with_options(
        &self,
        repo_path: &Path,
        options: &ExtractionOptions,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<PathBuf>> {
        fn compile_patterns(patterns: &[String]) -> Vec<glob::Pattern> {
            patterns
                .iter()
                .filter_map(|p| glob::Pattern::new(p).ok())
                .collect()
        }

        let exclude_patterns = compile_patterns(&options.exclude_patterns);
        let gittypeignore_matcher = self.load_gittypeignore_matcher(repo_path);

        let total_files_estimated = self.count_files(repo_path)?;

        let files = self.collect_files(
            repo_path,
            &compile_patterns(&options.include_patterns),
            &exclude_patterns,
            gittypeignore_matcher.as_ref(),
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
        gittypeignore_matcher: Option<&Gitignore>,
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
            .filter(|path| {
                Self::should_collect(
                    path,
                    repo_path,
                    include_patterns,
                    exclude_patterns,
                    gittypeignore_matcher,
                )
            })
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
        repo_path: &Path,
        include_patterns: &[glob::Pattern],
        exclude_patterns: &[glob::Pattern],
        gittypeignore_matcher: Option<&Gitignore>,
    ) -> bool {
        let full_path = path.to_string_lossy();
        let relative_path = path
            .strip_prefix(repo_path)
            .unwrap_or(path)
            .to_string_lossy();

        if gittypeignore_matcher
            .map(|matcher| Self::matches_gittypeignore(path, matcher))
            .unwrap_or(false)
        {
            return false;
        }

        if exclude_patterns
            .iter()
            .any(|pattern| pattern.matches(&full_path) || pattern.matches(&relative_path))
        {
            return false;
        }

        include_patterns
            .iter()
            .any(|pattern| pattern.matches(&full_path) || pattern.matches(&relative_path))
    }

    fn matches_gittypeignore(path: &Path, matcher: &Gitignore) -> bool {
        if matcher.matched(path, false).is_ignore() {
            return true;
        }

        let mut parent = path.parent();
        while let Some(dir) = parent {
            if matcher.matched(dir, true).is_ignore() {
                return true;
            }
            parent = dir.parent();
        }

        false
    }

    fn load_gittypeignore_matcher(&self, repo_path: &Path) -> Option<Gitignore> {
        let ignore_path = repo_path.join(".gittypeignore");

        if !self.file_storage.file_exists(&ignore_path) {
            return None;
        }

        match self.file_storage.read_to_string(&ignore_path) {
            Ok(content) => {
                let mut builder = GitignoreBuilder::new(repo_path);

                for (index, raw_line) in content.lines().enumerate() {
                    // Match gitignore's BOM behavior on the first line.
                    let line = if index == 0 {
                        raw_line.trim_start_matches('\u{feff}')
                    } else {
                        raw_line
                    };

                    if let Err(error) = builder.add_line(Some(ignore_path.clone()), line) {
                        log::warn!(
                            "Invalid .gittypeignore pattern at {}:{}: {}",
                            ignore_path.display(),
                            index + 1,
                            error
                        );
                    }
                }

                match builder.build() {
                    Ok(matcher) => Some(matcher),
                    Err(error) => {
                        log::warn!(
                            "Failed to compile patterns from {}: {}. Continuing without custom excludes.",
                            ignore_path.display(),
                            error
                        );
                        None
                    }
                }
            }
            Err(error) => {
                log::warn!(
                    "Failed to read {}: {}. Continuing without custom excludes.",
                    ignore_path.display(),
                    error
                );
                None
            }
        }
    }
}
