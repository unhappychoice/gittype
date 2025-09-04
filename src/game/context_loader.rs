use crate::models::Challenge;
use crate::Result;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CodeContext {
    pub pre_context: Vec<String>,
    pub post_context: Vec<String>,
}

impl CodeContext {
    pub fn empty() -> Self {
        Self {
            pre_context: Vec::new(),
            post_context: Vec::new(),
        }
    }
}

pub fn load_context_for_challenge(
    challenge: &Challenge,
    context_lines: usize,
) -> Result<CodeContext> {
    let Some(source_path) = &challenge.source_file_path else {
        return Ok(CodeContext::empty());
    };

    let Some(start_line) = challenge.start_line else {
        return Ok(CodeContext::empty());
    };

    let Some(end_line) = challenge.end_line else {
        return Ok(CodeContext::empty());
    };

    load_context_lines(Path::new(source_path), start_line, end_line, context_lines)
}

pub fn load_context_lines(
    file_path: &Path,
    start_line: usize,
    end_line: usize,
    context_lines: usize,
) -> Result<CodeContext> {
    if !file_path.exists() {
        return Ok(CodeContext::empty());
    }

    let content = fs::read_to_string(file_path)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_context_lines() {
        let content =
            "line1\nline2\nline3\nTARGET_START\nTARGET_CONTENT\nTARGET_END\nline7\nline8\nline9";
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, content).unwrap();

        let result = load_context_lines(temp_file.path(), 4, 6, 2).unwrap();

        assert_eq!(result.pre_context, vec!["line2", "line3"]);
        assert_eq!(result.post_context, vec!["line7", "line8"]);
    }

    #[test]
    fn test_load_context_at_file_boundaries() {
        let content = "line1\nTARGET\nline3";
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, content).unwrap();

        let result = load_context_lines(temp_file.path(), 2, 2, 5).unwrap();

        assert_eq!(result.pre_context, vec!["line1"]);
        assert_eq!(result.post_context, vec!["line3"]);
    }
}
