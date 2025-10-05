use crate::domain::models::Language;
use crate::domain::models::{CodeChunk, ExtractionOptions};
use crate::domain::services::source_code_parser::parsers::parse_with_thread_local;
use crate::domain::services::source_code_parser::ChunkExtractor;
use crate::infrastructure::git::LocalGitRepositoryClient;
use crate::presentation::game::models::StepType;
use crate::presentation::game::screens::loading_screen::ProgressReporter;
use crate::{GitTypeError, Result};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct SourceCodeParser;

impl SourceCodeParser {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn extract_chunks_with_progress<P: ProgressReporter + ?Sized>(
        &mut self,
        files_to_process: Vec<(PathBuf, Box<dyn Language>)>,
        options: &ExtractionOptions,
        progress: &P,
    ) -> Result<Vec<CodeChunk>> {
        let git_root = Self::find_git_root(&files_to_process)?;
        let valid_files = Self::filter_and_sort_files(files_to_process, options);
        let valid_files_count = valid_files.len();

        // Initialize extracting progress from 0
        let processed = Arc::new(AtomicUsize::new(0));
        progress.set_file_counts(StepType::Extracting, 0, valid_files_count, None);

        let all_chunks: Vec<CodeChunk> = valid_files
            .into_par_iter()
            .inspect(|_| {
                let current = processed.fetch_add(1, Ordering::Relaxed) + 1;
                Self::update_progress_if_needed(progress, current, valid_files_count);
            })
            .flat_map(|(path, language, _size)| {
                Self::read_and_parse_file(&git_root, &path, language).into_par_iter()
            })
            .flat_map(|(tree, content, file_path, git_root, language)| {
                ChunkExtractor::extract_chunks_from_tree(
                    &tree,
                    &content,
                    &file_path,
                    &git_root,
                    language.as_ref(),
                )
                .unwrap_or_default()
            })
            .collect();

        // Get final count and ensure final progress is exactly 100%
        let final_count = processed.load(Ordering::Relaxed);
        progress.set_file_counts(StepType::Extracting, final_count, final_count, None);
        progress.set_current_file(None);

        Ok(all_chunks)
    }

    fn find_git_root(files_to_process: &[(PathBuf, Box<dyn Language>)]) -> Result<PathBuf> {
        files_to_process
            .first()
            .map(|(first_file, _)| first_file)
            .and_then(|path| LocalGitRepositoryClient::get_repository_root(path))
            .ok_or_else(|| GitTypeError::ExtractionFailed("Git repository not found".to_string()))
    }

    fn filter_and_sort_files(
        files_to_process: Vec<(PathBuf, Box<dyn Language>)>,
        options: &ExtractionOptions,
    ) -> Vec<(PathBuf, Box<dyn Language>, u64)> {
        let mut valid_files: Vec<_> = files_to_process
            .into_par_iter()
            .filter_map(|(path, lang)| {
                let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                if size > options.max_file_size_bytes {
                    log::warn!(
                        "Skipping large file: {:?} ({}MB > {}MB limit)",
                        path,
                        size / (1024 * 1024),
                        options.max_file_size_bytes / (1024 * 1024)
                    );
                    None
                } else {
                    Some((path, lang, size))
                }
            })
            .collect();

        valid_files.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by size (largest first)
        valid_files
    }

    fn update_progress_if_needed<P: ProgressReporter + ?Sized>(
        progress: &P,
        current: usize,
        total_files: usize,
    ) {
        // Dynamic progress update frequency: every 10 files, but every 1 file when >99%
        let progress_threshold = if current as f64 / total_files as f64 > 0.99 {
            1 // Update every file when >99%
        } else {
            10 // Update every 10 files normally
        };

        if current.is_multiple_of(progress_threshold) || current == total_files {
            progress.set_file_counts(StepType::Extracting, current, total_files, None);
        }
    }

    #[allow(clippy::type_complexity)]
    fn read_and_parse_file(
        git_root: &Path,
        file_path: &Path,
        language: Box<dyn Language>,
    ) -> Option<(
        tree_sitter::Tree,
        String,
        PathBuf,
        PathBuf,
        Box<dyn Language>,
    )> {
        let content = fs::read_to_string(file_path).ok()?;
        let tree = parse_with_thread_local(language.name(), &content)?;

        Some((
            tree,
            content,
            file_path.to_path_buf(),
            git_root.to_path_buf(),
            language,
        ))
    }
}
