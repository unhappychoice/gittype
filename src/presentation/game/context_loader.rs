use crate::domain::models::Challenge;
use crate::infrastructure::storage::file_storage::FileStorage;
use crate::presentation::game::GameData;
use crate::Result;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct CodeContext {
    pub pre_context: Vec<String>,
    pub post_context: Vec<String>,
}

impl CodeContext {
    pub fn empty() -> Self {
        Self::default()
    }
}

pub fn load_context_for_challenge(
    challenge: &Challenge,
    context_lines: usize,
) -> Result<CodeContext> {
    let file_storage = FileStorage::new();
    let Some(source_path) = &challenge.source_file_path else {
        return Ok(CodeContext::empty());
    };

    let Some(start_line) = challenge.start_line else {
        return Ok(CodeContext::empty());
    };

    let Some(end_line) = challenge.end_line else {
        return Ok(CodeContext::empty());
    };

    // Convert relative path to absolute path by resolving from git repository root
    let file_path = if Path::new(source_path).is_absolute() {
        PathBuf::from(source_path)
    } else {
        // Get git root from GameData's GitRepository and resolve relative path
        if let Some(git_repository) = GameData::get_git_repository() {
            if let Some(git_root) = git_repository.root_path {
                git_root.join(source_path)
            } else {
                // Fallback to using the path as-is if git root is not found in GitRepository
                PathBuf::from(source_path)
            }
        } else {
            // Fallback to using the path as-is if git repository is not found
            PathBuf::from(source_path)
        }
    };

    load_context_lines(
        &file_storage,
        &file_path,
        start_line,
        end_line,
        context_lines,
    )
}

pub fn load_context_lines(
    file_storage: &FileStorage,
    file_path: &Path,
    start_line: usize,
    end_line: usize,
    context_lines: usize,
) -> Result<CodeContext> {
    if !file_storage.file_exists(file_path) {
        return Ok(CodeContext::empty());
    }

    let content = file_storage.read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Calculate context ranges (1-indexed to 0-indexed)
    let pre_start = start_line.saturating_sub(context_lines + 1);
    let pre_end = (start_line - 1).min(lines.len());

    let post_start = end_line.min(lines.len());
    let post_end = (end_line + context_lines).min(lines.len());

    // Extract pre-context lines
    let pre_context = if pre_start < pre_end {
        lines[pre_start..pre_end]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };

    // Extract post-context lines
    let post_context = if post_start < post_end {
        lines[post_start..post_end]
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };

    Ok(CodeContext {
        pre_context,
        post_context,
    })
}
