use super::{
    ChallengeConverter, CodeExtractor, ExtractionOptions, GitRepository, GitRepositoryExtractor,
    NoOpProgressReporter, ProgressReporter,
};
use crate::models::Challenge;
use crate::{GitTypeError, Result};
use std::path::Path;

pub struct RepositoryLoader {
    extractor: CodeExtractor,
    converter: ChallengeConverter,
    git_repository: Option<GitRepository>,
}

impl RepositoryLoader {
    pub fn new() -> Result<Self> {
        let extractor = CodeExtractor::new()?;
        let converter = ChallengeConverter::new();

        Ok(Self {
            extractor,
            converter,
            git_repository: None,
        })
    }

    pub fn load_challenges_from_repository(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
    ) -> Result<Vec<Challenge>> {
        self.load_challenges_from_repository_with_progress(
            repo_path,
            options,
            &NoOpProgressReporter,
        )
    }

    pub fn load_challenges_from_repository_with_progress<P: ProgressReporter>(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
        progress: &P,
    ) -> Result<Vec<Challenge>> {
        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        // Extract git information
        self.git_repository = GitRepositoryExtractor::extract_git_repository(repo_path)?;

        let extraction_options = options.unwrap_or_default();
        let chunks =
            self.extractor
                .extract_chunks_with_progress(repo_path, extraction_options, progress)?;

        if chunks.is_empty() {
            return Err(GitTypeError::NoSupportedFiles);
        }

        progress.set_step(crate::game::models::loading_steps::StepType::Generating);
        // Expand chunks into multiple challenges across difficulties
        let mut challenges = self
            .converter
            .convert_chunks_to_challenges_with_progress(chunks, progress);

        // Also add Zen challenges (entire files)
        let file_paths = self.collect_source_files(repo_path)?;
        let zen_challenges = self.converter.convert_whole_files_to_challenges(file_paths);
        challenges.extend(zen_challenges);

        progress.set_step(crate::game::models::loading_steps::StepType::Finalizing);
        Ok(challenges)
    }

    pub fn load_challenges_with_difficulty<P: ProgressReporter>(
        &mut self,
        repo_path: &Path,
        options: Option<ExtractionOptions>,
        difficulty: &crate::game::stage_builder::DifficultyLevel,
        progress: &P,
    ) -> Result<Vec<Challenge>> {
        use crate::game::stage_builder::DifficultyLevel;

        if !repo_path.exists() {
            return Err(GitTypeError::RepositoryNotFound(repo_path.to_path_buf()));
        }

        match difficulty {
            DifficultyLevel::Zen => {
                progress.set_step(crate::game::models::loading_steps::StepType::Generating);
                let file_paths = self.collect_source_files(repo_path)?;
                let challenges = self.converter.convert_whole_files_to_challenges(file_paths);
                progress.set_step(crate::game::models::loading_steps::StepType::Finalizing);
                Ok(challenges)
            }
            _ => {
                let extraction_options = options.unwrap_or_default();
                let chunks = self.extractor.extract_chunks_with_progress(
                    repo_path,
                    extraction_options,
                    progress,
                )?;

                if chunks.is_empty() {
                    return Err(GitTypeError::NoSupportedFiles);
                }

                progress.set_step(crate::game::models::loading_steps::StepType::Generating);
                let challenges = self
                    .converter
                    .convert_with_difficulty_split(chunks, difficulty);

                progress.set_step(crate::game::models::loading_steps::StepType::Finalizing);
                Ok(challenges)
            }
        }
    }

    fn collect_source_files(&self, repo_path: &Path) -> Result<Vec<std::path::PathBuf>> {
        use super::models::language::LanguageRegistry;
        use super::models::ExtractionOptions;
        use ignore::WalkBuilder;

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

        let walker = WalkBuilder::new(repo_path)
            .hidden(false) // Include hidden files
            .git_ignore(true) // Respect .gitignore
            .git_global(true) // Respect global gitignore
            .git_exclude(true) // Respect .git/info/exclude
            .build();

        let mut files = Vec::new();

        for entry in walker {
            let entry =
                entry.map_err(|e| GitTypeError::ExtractionFailed(format!("Walk error: {}", e)))?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

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
        }

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

    pub fn load_all_files_as_zen_challenges(&mut self, repo_path: &Path) -> Result<Vec<Challenge>> {
        // Get all source files in the repository
        let all_source_files = self.collect_source_files(repo_path)?;

        // Create Zen challenges for each file
        let zen_challenges = self
            .converter
            .convert_whole_files_to_zen_challenges_only(all_source_files);

        Ok(zen_challenges)
    }

    pub fn get_git_repository(&self) -> &Option<GitRepository> {
        &self.git_repository
    }
}
