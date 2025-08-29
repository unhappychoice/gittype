use super::stage_builder::DifficultyLevel;

#[derive(Debug, Clone)]
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

    pub fn with_source_info(mut self, file_path: String, start_line: usize, end_line: usize) -> Self {
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

    pub fn calculate_difficulty_by_char_count(&self) -> DifficultyLevel {
        let char_count = self.code_content.chars().count();
        match char_count {
            0..=100 => DifficultyLevel::Easy,
            101..=300 => DifficultyLevel::Normal,
            301..=500 => DifficultyLevel::Hard,
            _ => DifficultyLevel::Zen,
        }
    }

    pub fn get_display_title(&self) -> String {
        if let Some(ref path) = self.source_file_path {
            if let (Some(start), Some(end)) = (self.start_line, self.end_line) {
                format!("{}:{}-{}", path, start, end)
            } else {
                path.clone()
            }
        } else {
            format!("Challenge {}", self.id)
        }
    }
}