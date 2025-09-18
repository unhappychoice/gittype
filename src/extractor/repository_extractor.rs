use super::models::language::LanguageRegistry;
use super::{ChallengeConverter, CodeChunkExtractor, ExtractionOptions, ProgressReporter};
use crate::{GitTypeError, Result};
use ignore::WalkBuilder;
use std::path::Path;

pub struct RepositoryExtractor {
    extractor: CodeChunkExtractor,
    converter: ChallengeConverter,
}

impl RepositoryExtractor {
    pub fn new() -> Result<Self> {
        let extractor = CodeChunkExtractor::new()?;
        let converter = ChallengeConverter::new();

        Ok(Self {
            extractor,
            converter,
        })
    }

    pub fn collect_source_files_with_progress(
        &self,
        repo_path: &Path,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<std::path::PathBuf>> {
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
                if LanguageRegistry::from_extension(extension).is_some()
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
                    crate::game::models::loading_steps::StepType::Scanning,
                    processed,
                    total_files_estimated,
                    None,
                );
            }
        }

        // Ensure final progress is exactly 100%
        progress.set_file_counts(
            crate::game::models::loading_steps::StepType::Scanning,
            total_files_estimated,
            total_files_estimated,
            None,
        );

        Ok(files)
    }

    pub fn convert_chunks_and_files_to_challenges_with_progress(
        &self,
        chunks: Vec<crate::extractor::models::CodeChunk>,
        _file_paths: Vec<std::path::PathBuf>, // No longer needed, files are processed as chunks
        _git_root: Option<&std::path::Path>,
        progress: &dyn ProgressReporter,
    ) -> Vec<crate::models::Challenge> {
        self.converter
            .convert_chunks_and_files_to_challenges_with_progress(chunks, progress)
    }

    pub fn extract_chunks_from_scanned_files_with_progress(
        &mut self,
        scanned_files: &[std::path::PathBuf],
        options: ExtractionOptions,
        progress: &dyn ProgressReporter,
    ) -> Result<Vec<crate::extractor::models::CodeChunk>> {
        // Convert scanned files to (path, language) pairs
        let files_to_process: Vec<(
            std::path::PathBuf,
            Box<dyn super::models::language::Language>,
        )> = scanned_files
            .iter()
            .filter_map(|path| {
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    LanguageRegistry::from_extension(extension)
                        .map(|language| (path.to_owned(), language))
                } else {
                    None
                }
            })
            .collect();

        self.extractor
            .extract_chunks_from_files_with_progress(files_to_process, options, progress)
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
