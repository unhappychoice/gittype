use crate::domain::models::{ExtractionOptions, Languages};
use crate::presentation::game::models::StepType;
use crate::presentation::game::screens::loading_screen::ProgressReporter;
use crate::{GitTypeError, Result};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct SourceFileExtractor;

impl SourceFileExtractor {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn collect_source_files_with_progress(
        &self,
        repo_path: &Path,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<PathBuf>> {
        let options = ExtractionOptions::default();

        // Compile glob patterns once for faster matching
        let include_patterns: Vec<glob::Pattern> = options
            .include_patterns
            .iter()
            .filter_map(|p| glob::Pattern::new(p).ok())
            .collect();
        let exclude_patterns: Vec<glob::Pattern> = options
            .exclude_patterns
            .iter()
            .filter_map(|p| glob::Pattern::new(p).ok())
            .collect();

        // First pass: count total files to estimate progress
        let walker_count = WalkBuilder::new(repo_path)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .git_global(true) // Respect global gitignore
            .git_exclude(true) // Respect .git/info/exclude
            .build();

        let mut total_files_estimated = 0;
        for entry in walker_count {
            let entry =
                entry.map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))?;
            let path = entry.path();

            if path.is_file() {
                total_files_estimated += 1;
            }
        }

        // Second pass: collect matching files with progress
        let walker = WalkBuilder::new(repo_path)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .git_global(true) // Respect global gitignore
            .git_exclude(true) // Respect .git/info/exclude
            .build();

        let mut files = Vec::new();
        let mut processed = 0;

        for entry in walker {
            let entry =
                entry.map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            processed += 1;

            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                if Languages::from_extension(extension).is_some()
                    && Self::should_process_file_compiled(
                        path,
                        &include_patterns,
                        &exclude_patterns,
                    )
                {
                    files.push(path.to_path_buf());
                }
            }

            if processed % 100 == 0 || processed == total_files_estimated {
                // Update progress with estimated total
                progress.set_file_counts(
                    StepType::Scanning,
                    processed,
                    total_files_estimated,
                    None,
                );
            }
        }

        // Ensure final progress is exactly 100%
        progress.set_file_counts(
            StepType::Scanning,
            total_files_estimated,
            total_files_estimated,
            None,
        );

        Ok(files)
    }

    fn should_process_file_compiled(
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
