use super::git_repository::GitRepository;
use crate::game::DifficultyLevel;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Challenge {
    pub id: String,
    pub source_file_path: Option<String>,
    pub code_content: String,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub language: Option<String>,
    pub comment_ranges: Vec<(usize, usize)>, // Character-based ranges for comments
    pub difficulty_level: Option<DifficultyLevel>,
}

impl Challenge {
    pub fn new(id: String, code_content: String) -> Self {
        Self {
            id,
            source_file_path: None,
            code_content,
            start_line: None,
            end_line: None,
            language: None,
            comment_ranges: Vec::new(),
            difficulty_level: None,
        }
    }

    pub fn with_source_info(
        mut self,
        file_path: String,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        self.source_file_path = Some(file_path);
        self.start_line = Some(start_line);
        self.end_line = Some(end_line);
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    pub fn with_comment_ranges(mut self, comment_ranges: Vec<(usize, usize)>) -> Self {
        self.comment_ranges = comment_ranges;
        self
    }

    pub fn with_difficulty_level(mut self, difficulty_level: DifficultyLevel) -> Self {
        self.difficulty_level = Some(difficulty_level);
        self
    }

    pub fn get_display_title(&self) -> String {
        if let Some(ref path) = self.source_file_path {
            // Convert absolute path to relative path for cleaner display
            let relative_path = self.get_relative_path(path);
            if let (Some(start), Some(end)) = (self.start_line, self.end_line) {
                format!("{}:{}-{}", relative_path, start, end)
            } else {
                relative_path
            }
        } else {
            format!("Challenge {}", self.id)
        }
    }

    pub fn get_display_title_with_repo(&self, repo_info: &Option<GitRepository>) -> String {
        if let Some(ref path) = self.source_file_path {
            let relative_path = self.get_relative_path(path);
            let file_info = if let (Some(start), Some(end)) = (self.start_line, self.end_line) {
                format!("{}:{}-{}", relative_path, start, end)
            } else {
                relative_path
            };

            if let Some(repo) = repo_info {
                format!(
                    "[{}/{}] {}",
                    repo.user_name, repo.repository_name, file_info
                )
            } else {
                file_info
            }
        } else {
            format!("Challenge {}", self.id)
        }
    }

    fn get_relative_path(&self, path: &str) -> String {
        use std::path::Path;

        // Try to extract just the filename if it's a full path
        if let Some(file_name) = Path::new(path).file_name() {
            if let Some(parent) = Path::new(path).parent() {
                if let Some(parent_name) = parent.file_name() {
                    // Show parent_dir/filename for better context
                    format!(
                        "{}/{}",
                        parent_name.to_string_lossy(),
                        file_name.to_string_lossy()
                    )
                } else {
                    file_name.to_string_lossy().to_string()
                }
            } else {
                file_name.to_string_lossy().to_string()
            }
        } else {
            // Fallback to original path if extraction fails
            path.to_string()
        }
    }
}
