use super::CodeChunk;
use crate::game::Challenge;
use uuid::Uuid;

pub struct ChallengeConverter;

impl Default for ChallengeConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl ChallengeConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_chunk_to_challenge(&self, chunk: CodeChunk) -> Challenge {
        let id = Uuid::new_v4().to_string();
        let language = self.language_to_string(&chunk.language);
        let file_path = chunk.file_path.to_string_lossy().to_string();

        Challenge::new(id, chunk.content)
            .with_source_info(file_path, chunk.start_line, chunk.end_line)
            .with_language(language)
            .with_comment_ranges(chunk.comment_ranges)
    }

    pub fn convert_chunks_to_challenges(&self, chunks: Vec<CodeChunk>) -> Vec<Challenge> {
        let mut all_challenges = Vec::new();

        for chunk in &chunks {
            // Generate challenges for Easy (~100), Normal (~200), Hard (~500), Wild (full chunks) only
            let difficulties = [
                super::super::game::stage_builder::DifficultyLevel::Easy,
                super::super::game::stage_builder::DifficultyLevel::Normal,
                super::super::game::stage_builder::DifficultyLevel::Hard,
                super::super::game::stage_builder::DifficultyLevel::Wild,
            ];

            for difficulty in &difficulties {
                let split_challenges = self.split_chunk_by_difficulty(chunk, difficulty);
                all_challenges.extend(split_challenges);
            }
        }

        // Zen challenges are now handled separately in main.rs
        all_challenges
    }

    pub fn convert_with_filter<F>(&self, chunks: Vec<CodeChunk>, filter: F) -> Vec<Challenge>
    where
        F: Fn(&CodeChunk) -> bool,
    {
        chunks
            .into_iter()
            .filter(filter)
            .map(|chunk| self.convert_chunk_to_challenge(chunk))
            .collect()
    }

    pub fn convert_with_difficulty_split(
        &self,
        chunks: Vec<CodeChunk>,
        difficulty: &super::super::game::stage_builder::DifficultyLevel,
    ) -> Vec<Challenge> {
        let mut challenges = Vec::new();

        for chunk in chunks {
            let split_challenges = self.split_chunk_by_difficulty(&chunk, difficulty);
            challenges.extend(split_challenges);
        }

        challenges
    }

    pub fn convert_whole_files_to_challenges(
        &self,
        file_paths: Vec<std::path::PathBuf>,
    ) -> Vec<Challenge> {
        use super::super::game::stage_builder::DifficultyLevel;
        let mut challenges = Vec::new();

        for file_path in file_paths {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let id = Uuid::new_v4().to_string();
                let language = self.detect_language_from_path(&file_path);
                let file_path_str = file_path.to_string_lossy().to_string();

                let line_count = content.lines().count();
                let challenge = Challenge::new(id, content)
                    .with_source_info(file_path_str, 1, line_count)
                    .with_language(language)
                    .with_difficulty_level(DifficultyLevel::Zen);

                challenges.push(challenge);
            }
        }

        challenges
    }

    fn detect_language_from_path(&self, path: &std::path::Path) -> String {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => "rust".to_string(),
            Some("ts") | Some("tsx") => "typescript".to_string(),
            Some("py") => "python".to_string(),
            Some("go") => "go".to_string(),
            Some("rb") => "ruby".to_string(),
            Some("js") | Some("jsx") => "javascript".to_string(),
            _ => "text".to_string(),
        }
    }

    fn split_chunk_by_difficulty(
        &self,
        chunk: &CodeChunk,
        difficulty: &super::super::game::stage_builder::DifficultyLevel,
    ) -> Vec<Challenge> {
        use super::super::game::stage_builder::DifficultyLevel;

        if matches!(difficulty, DifficultyLevel::Zen) {
            return vec![self.convert_chunk_to_challenge(chunk.clone())];
        }

        // Wild difficulty uses the full chunk as-is
        if matches!(difficulty, DifficultyLevel::Wild) {
            let mut challenge = self.convert_chunk_to_challenge(chunk.clone());
            challenge.difficulty_level = Some(difficulty.clone());
            return vec![challenge];
        }

        let (min_chars, max_chars) = difficulty.char_limits();

        let content = &chunk.content;
        let lines: Vec<&str> = content.lines().collect();

        // Calculate actual code characters (excluding comments) using AST data
        let code_char_count = self.count_code_characters(content, &chunk.comment_ranges);

        // Skip if chunk doesn't meet minimum size for this difficulty
        if code_char_count < min_chars {
            return vec![]; // Don't generate challenge for this difficulty
        }

        // If the chunk is within the target range, return as-is
        if code_char_count <= max_chars {
            let mut challenge = self.convert_chunk_to_challenge(chunk.clone());
            challenge.difficulty_level = Some(difficulty.clone());

            return vec![challenge];
        }

        // Find the best natural break point that keeps us under max_chars
        let break_point = self.find_optimal_break_point(content, &chunk.comment_ranges, max_chars);

        if break_point > 0 && break_point < lines.len() {
            // Create single challenge from beginning to break point only
            // Don't create meaningless fragments from the remainder
            let selected_lines: String = lines
                .iter()
                .take(break_point)
                .map(|l| format!("{}\n", l))
                .collect();

            if !selected_lines.trim().is_empty() {
                let truncated_content = selected_lines.trim_end();

                // Check if truncated content meets minimum requirements
                let adjusted_comment_ranges = self.adjust_comment_ranges_for_truncation(
                    &chunk.comment_ranges,
                    truncated_content.len(),
                );
                let truncated_code_chars =
                    self.count_code_characters(truncated_content, &adjusted_comment_ranges);

                // Only create challenge if it meets minimum size for this difficulty
                if truncated_code_chars >= min_chars {
                    let id = Uuid::new_v4().to_string();
                    let language = self.language_to_string(&chunk.language);
                    let file_path = chunk.file_path.to_string_lossy().to_string();

                    let challenge = Challenge::new(id, truncated_content.to_string())
                        .with_source_info(
                            file_path,
                            chunk.start_line,
                            chunk.start_line + break_point - 1,
                        )
                        .with_language(language)
                        .with_comment_ranges(adjusted_comment_ranges)
                        .with_difficulty_level(difficulty.clone());

                    return vec![challenge];
                }
            }
        }

        // Don't use fallback for difficulty-based splitting - if we can't fit within the target range, don't generate a challenge
        vec![] // Don't generate challenge if it doesn't fit within the difficulty range
    }

    fn count_code_characters(&self, content: &str, comment_ranges: &[(usize, usize)]) -> usize {
        let chars: Vec<char> = content.chars().collect();
        let mut code_char_count = 0;

        for (i, ch) in chars.iter().enumerate() {
            // Skip whitespace-only characters
            if ch.is_whitespace() {
                continue;
            }

            // Check if this character is inside a comment range
            let in_comment = comment_ranges
                .iter()
                .any(|&(start, end)| i >= start && i < end);

            if !in_comment {
                code_char_count += 1;
            }
        }

        code_char_count
    }

    fn find_optimal_break_point(
        &self,
        content: &str,
        comment_ranges: &[(usize, usize)],
        target_chars: usize,
    ) -> usize {
        let lines: Vec<&str> = content.lines().collect();
        let mut current_pos = 0;
        let mut code_char_count = 0;
        let mut last_good_break = 0;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_end = current_pos + line.len();

            // Count non-comment, non-whitespace characters in this line
            for (char_idx, ch) in line.chars().enumerate() {
                let char_pos = current_pos + char_idx;

                if ch.is_whitespace() {
                    continue;
                }

                // Check if this character is in a comment
                let in_comment = comment_ranges
                    .iter()
                    .any(|&(start, end)| char_pos >= start && char_pos < end);

                if !in_comment {
                    code_char_count += 1;
                }
            }

            // Check if we've exceeded the target
            if code_char_count > target_chars {
                // Return the last good break point
                return last_good_break.max(1);
            }

            // Update last good break point if this is a natural boundary
            if self.is_natural_boundary(line) {
                last_good_break = line_idx + 1;
            }

            // Move to next line (add 1 for newline character)
            current_pos = line_end + 1;
        }

        // If we never exceeded the target, return the full length
        lines.len()
    }

    fn is_natural_boundary(&self, line: &str) -> bool {
        let trimmed = line.trim();

        // Empty lines are natural boundaries
        if trimmed.is_empty() {
            return true;
        }

        // Lines ending with closing braces/brackets (end of blocks/scopes)
        if trimmed.ends_with('}') || trimmed.ends_with(']') || trimmed.ends_with(')') {
            return true;
        }

        // Lines ending with semicolons (end of statements)
        if trimmed.ends_with(';') {
            return true;
        }

        false
    }

    fn adjust_comment_ranges_for_truncation(
        &self,
        original_ranges: &[(usize, usize)],
        new_length: usize,
    ) -> Vec<(usize, usize)> {
        original_ranges
            .iter()
            .filter_map(|&(start, end)| {
                // Only include ranges that fall within the truncated content
                if start < new_length {
                    let adjusted_end = end.min(new_length);
                    if adjusted_end > start {
                        Some((start, adjusted_end))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn convert_whole_files_to_zen_challenges_only(
        &self,
        file_paths: Vec<std::path::PathBuf>,
    ) -> Vec<Challenge> {
        let mut zen_challenges = Vec::new();

        for file_path in file_paths {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let id = uuid::Uuid::new_v4().to_string();
                let language = self.detect_language_from_path(&file_path);
                let file_path_str = file_path.to_string_lossy().to_string();

                let line_count = content.lines().count();
                let challenge = Challenge::new(id, content)
                    .with_source_info(file_path_str, 1, line_count)
                    .with_language(language)
                    .with_difficulty_level(super::super::game::stage_builder::DifficultyLevel::Zen);

                zen_challenges.push(challenge);
            }
        }

        zen_challenges
    }

    fn language_to_string(&self, language: &super::Language) -> String {
        match language {
            super::Language::Rust => "rust".to_string(),
            super::Language::TypeScript => "typescript".to_string(),
            super::Language::Python => "python".to_string(),
            super::Language::Ruby => "ruby".to_string(),
            super::Language::Go => "go".to_string(),
            super::Language::Swift => "swift".to_string(),
        }
    }
}
